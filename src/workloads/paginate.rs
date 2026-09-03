use futures::{SinkExt, StreamExt};
use std::time::Instant;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::workloads::nostr::connect;

pub async fn execute_paginate_bench(
    url: &str,
    depth: usize,
    concurrency: usize,
) -> (usize, f64, Vec<f64>) {
    let mut tasks = Vec::with_capacity(concurrency);
    let start = Instant::now();

    for _ in 0..concurrency {
        let url = url.to_string();
        tasks.push(tokio::spawn(async move {
            let mut page_times = Vec::with_capacity(depth);
            if let Ok(mut ws) = connect(&url).await {
                let mut until: Option<u64> = None;
                for i in 0..depth {
                    let subid = format!("pag-{}", i);
                    let mut fv = serde_json::json!({
                        "kinds": [1, 30382],
                        "limit": 500
                    });

                    if let Some(u) = until {
                        fv.as_object_mut().unwrap().insert("until".to_string(), serde_json::json!(u));
                    }

                    let msg = serde_json::json!(["REQ", subid, fv]).to_string();
                    let req_start = Instant::now();

                    if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                        let mut last_created_at = None;
                        loop {
                            if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                if resp.starts_with("[\"EVENT\"") {
                                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&resp)
                                        && let Some(ev) = parsed.get(2)
                                            && let Some(ca) = ev.get("created_at").and_then(|c| c.as_u64()) {
                                                last_created_at = Some(ca);
                                            }
                                } else if resp.starts_with("[\"EOSE\"") {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        page_times.push(req_start.elapsed().as_secs_f64());
                        let close_msg = serde_json::json!(["CLOSE", subid]).to_string();
                        let _ = ws.send(WsMessage::Text(close_msg.into())).await;

                        until = last_created_at.map(|c| c.saturating_sub(1));
                        if until.is_none() {
                            break;
                        }
                    }
                }
                let _ = ws.close(None).await;
            }
            page_times
        }));
    }

    let mut all_times = Vec::new();
    for t in tasks {
        if let Ok(times) = t.await {
            all_times.extend(times);
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    (all_times.len(), elapsed, all_times)
}
