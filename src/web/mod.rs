pub mod state;
pub mod routes;
pub mod ws;
pub mod ui;

use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use crate::web::state::AppState;

pub async fn start_web_server(port: u16, state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(routes::get_index))
        .route("/assets/alpine.min.js", get(routes::get_alpine_js))
        .route("/api/status", get(routes::get_status))
        .route("/api/commits", get(routes::get_commits))
        .route("/api/branches", get(routes::get_branches))
        .route("/api/run", post(routes::trigger_run))
        .route("/api/reports", get(routes::list_reports))
        .route("/api/reports/{id}", get(routes::get_report_detail))
        .route("/api/reports/{id}/files", get(routes::list_report_files))
        .route("/api/reports/{id}/{filename}", get(routes::get_report_file))
        .route("/api/ws", get(ws::ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    let repo_desc = state
        .repo_path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| state.default_config.url.clone());

    println!("\n================================================================================");
    println!("🚀 strfry-bench Web UI is live!");
    println!("👉 Open in browser: http://localhost:{}", port);
    println!("📂 Target: {}", repo_desc);
    println!("================================================================================\n");

    axum::serve(listener, app).await?;
    Ok(())
}
