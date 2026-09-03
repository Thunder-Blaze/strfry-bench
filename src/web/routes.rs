use axum::extract::{Path as AxumPath, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Instant;
use crate::config::{Suite, TestType};
use crate::git::GitManager;
use crate::orchestrator::{Orchestrator, ProgressEvent};
use crate::web::state::AppState;
use crate::web::ui::RENDERED_HTML;

pub async fn get_index() -> Html<&'static str> {
    Html(RENDERED_HTML)
}

pub async fn get_alpine_js() -> impl IntoResponse {
    ([(axum::http::header::CONTENT_TYPE, "application/javascript")], crate::web::ui::ALPINE_JS)
}

pub async fn get_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let status = state.status.read().clone();
    let completed = state.completed_suites.read().clone();
    let single = state.last_single_report.read().clone();
    let comparison = state.last_comparison_report.read().clone();

    Json(serde_json::json!({
        "status": status,
        "completed_suites": completed,
        "single_report": single,
        "comparison_report": comparison,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CommitsQuery {
    pub branch: Option<String>,
}

pub async fn get_commits(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(query): axum::extract::Query<CommitsQuery>,
) -> impl IntoResponse {
    if let Some(repo) = &state.repo_path {
        let git = GitManager::new(repo);
        if let Ok(commits) = git.get_commits_for_ref(query.branch.as_deref(), 50) {
            *state.commits_cache.write() = commits.clone();
            return Json(commits);
        }
    }
    Json(state.commits_cache.read().clone())
}

pub async fn get_branches(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if let Some(repo) = &state.repo_path {
        let git = GitManager::new(repo);
        if let Ok(branches) = git.get_all_branches() {
            return Json(branches);
        }
    }
    Json(vec!["master".to_string(), "HEAD".to_string()])
}

#[derive(Debug, Deserialize)]
pub struct TriggerRunRequest {
    pub target_mode: Option<String>, // "source" or "live"
    pub url: Option<String>,
    pub test_type: Option<TestType>,
    pub base: Option<String>,
    pub target: Option<String>,
    pub current: Option<bool>,
    pub suites: Option<Vec<String>>,
    pub skip_heavy: Option<bool>,
    pub high_performance: Option<bool>,
    pub flamegraph: Option<bool>,
    pub alloc_tracker: Option<bool>,
}
pub async fn trigger_run(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TriggerRunRequest>,
) -> impl IntoResponse {
    {
        let mut status = state.status.write();
        if status.is_running {
            return (
                StatusCode::CONFLICT,
                Json(serde_json::json!({ "error": "A benchmark is already running" })),
            );
        }
        status.is_running = true;
        status.current_step = "Initializing...".to_string();
        status.completed_suites_count = 0;
        status.elapsed_secs = 0.0;
        state.completed_suites.write().clear();
    }

    let mut cfg = state.default_config.clone();
    if let Some(mode) = &req.target_mode {
        if mode == "live" {
            cfg.path = None;
            if let Some(u) = &req.url {
                cfg.url = u.clone();
            }
        } else if mode == "source" {
            cfg.path = state.repo_path.clone();
        }
    } else if let Some(u) = &req.url {
        cfg.url = u.clone();
    }

    if let Some(tt) = req.test_type {
        cfg.test_type = tt;
    }
    if let Some(b) = req.base {
        cfg.base = b;
    }
    if let Some(t) = req.target {
        cfg.target = t;
    }
    if let Some(c) = req.current {
        cfg.current = c;
    }
    if let Some(sh) = req.skip_heavy {
        cfg.skip_heavy = sh;
    }
    if let Some(hp) = req.high_performance {
        cfg.high_performance = hp;
    }
    if let Some(fg) = req.flamegraph {
        cfg.flamegraph = fg;
    }
    if let Some(at) = req.alloc_tracker {
        cfg.alloc_tracker = at;
    }
    if let Some(suites_str) = req.suites {
        cfg.suites = suites_str
            .iter()
            .filter_map(|s| s.parse::<Suite>().ok())
            .collect();
    }

    let state_clone = state.clone();
    let tx_log = state.tx_log.clone();

    tokio::spawn(async move {
        let progress_state = state_clone.clone();
        let progress = Arc::new(move |event: ProgressEvent| {
            match event {
                ProgressEvent::RunStarted { total_suites } => {
                    let mut status = progress_state.status.write();
                    status.is_running = true;
                    status.current_step = format!("Starting benchmark run with {} suites", total_suites);
                    status.total_suites = total_suites;
                    status.completed_suites_count = 0;
                    status.current_suite = None;
                    progress_state.completed_suites.write().clear();
                }
                ProgressEvent::Step { message } => {
                    progress_state.status.write().current_step = message;
                }
                ProgressEvent::SuiteStarted { suite, index, total } => {
                    let mut status = progress_state.status.write();
                    status.current_suite = Some(suite.id().to_string());
                    status.current_step = format!("Testing {} ({}/{})", suite.display_name(), index, total);
                }
                ProgressEvent::SuiteCompleted { result, completed, total, elapsed_secs, peak_rss_mb } => {
                    let mut suites = progress_state.completed_suites.write();
                    suites.push(result.clone());

                    let mut status = progress_state.status.write();
                    status.current_suite = None;
                    status.completed_suites_count = completed;
                    status.total_suites = total;
                    status.elapsed_secs = elapsed_secs;
                    if peak_rss_mb > status.peak_rss_mb {
                        status.peak_rss_mb = peak_rss_mb;
                    }
                    if let Some(tps) = result.throughput
                        && tps > status.peak_tps {
                            status.peak_tps = tps;
                        }
                    if let Some(p99) = result.p99_ms {
                        status.best_p99_ms = Some(status.best_p99_ms.map_or(p99, |old| old.min(p99)));
                    }
                    status.current_step = format!("Completed {} ({}/{})", result.name, completed, total);
                }
                ProgressEvent::RunFinished { output_dir, elapsed_secs } => {
                    let mut status = progress_state.status.write();
                    status.is_running = false;
                    status.current_suite = None;
                    status.elapsed_secs = elapsed_secs;
                    status.current_step = format!("Completed! Reports saved to {}", output_dir.display());
                }
                ProgressEvent::RunFailed { error } => {
                    let mut status = progress_state.status.write();
                    status.is_running = false;
                    status.current_suite = None;
                    status.current_step = format!("Failed: {}", error);
                }
            }
        });

        let orch = Orchestrator::new(cfg.clone())
            .with_log_sender(tx_log)
            .with_progress(progress);
        let start = Instant::now();

        let res = orch.run().await;

        let mut status = state_clone.status.write();
        status.is_running = false;
        status.elapsed_secs = start.elapsed().as_secs_f64();

        match res {
            Ok(dir) => {
                status.current_step = format!("Completed! Reports saved to {}", dir.display());
                let _ = state_clone.tx_log.send(format!("[WEB] Benchmark complete. Artifacts saved in {}", dir.display()));
            }
            Err(e) => {
                status.current_step = format!("Failed: {}", e);
                let _ = state_clone.tx_log.send(format!("[WEB ERROR] {}", e));
            }
        }
    });

    (
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Benchmark started successfully" })),
    )
}

pub async fn list_reports(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut list = Vec::new();
    let out_dir = &state.default_config.output_dir;
    if out_dir.exists()
        && let Ok(entries) = std::fs::read_dir(out_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    list.push(name);
                }
            }
        }
    list.sort();
    list.reverse();
    Json(list)
}

pub async fn get_report_detail(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    let dir_path = state.default_config.output_dir.join(&id);
    let json_path = dir_path.join("report.json");

    if json_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&json_path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                return (StatusCode::OK, Json(val)).into_response();
            }
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Report not found" }))).into_response()
}
pub async fn list_report_files(
    State(state): State<Arc<AppState>>,
    AxumPath(id): AxumPath<String>,
) -> impl IntoResponse {
    let mut files = Vec::new();
    let dir_path = state.default_config.output_dir.join(&id);
    if dir_path.exists()
        && let Ok(entries) = std::fs::read_dir(&dir_path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                files.push(name);
            }
        }
    files.sort();
    Json(files)
}


pub async fn get_report_file(
    State(state): State<Arc<AppState>>,
    AxumPath((id, filename)): AxumPath<(String, String)>,
) -> impl IntoResponse {
    let file_path = state.default_config.output_dir.join(&id).join(&filename);
    if file_path.exists()
        && let Ok(content) = std::fs::read_to_string(&file_path) {
            let content_type = if filename.ends_with(".json") {
                "application/json"
            } else if filename.ends_with(".svg") {
                "image/svg+xml"
            } else if filename.ends_with(".md") {
                "text/markdown"
            } else {
                "text/plain"
            };

            return (
                StatusCode::OK,
                [("content-type", content_type)],
                content,
            ).into_response();
        }
    (StatusCode::NOT_FOUND, "Not found").into_response()
}
