use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use crate::web::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx_log = state.tx_log.subscribe();
    let status_arc = state.status.clone();
    let completed_arc = state.completed_suites.clone();

    let mut ticker = tokio::time::interval(Duration::from_millis(200));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let status = status_arc.read().clone();
                let completed = completed_arc.read().clone();
                let packet = serde_json::json!({
                    "type": "telemetry",
                    "status": status,
                    "completed_suites": completed,
                });

                if let Ok(msg_text) = serde_json::to_string(&packet)
                    && socket.send(Message::Text(msg_text.into())).await.is_err() {
                        break;
                    }
            }
            Ok(log_line) = rx_log.recv() => {
                let packet = serde_json::json!({
                    "type": "log",
                    "line": log_line,
                });
                if let Ok(msg_text) = serde_json::to_string(&packet)
                    && socket.send(Message::Text(msg_text.into())).await.is_err() {
                        break;
                    }
            }
            msg = socket.next() => {
                if let Some(Ok(Message::Close(_))) | None = msg {
                    break;
                }
            }
        }
    }
}
