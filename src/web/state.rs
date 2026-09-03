use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use crate::config::{BenchmarkConfig, ComparisonReport, RunReport, SuiteResult};
use crate::git::CommitInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveStatus {
    pub is_running: bool,
    pub current_suite: Option<String>,
    pub current_step: String,
    pub total_suites: usize,
    pub completed_suites_count: usize,
    pub peak_tps: f64,
    pub best_p99_ms: Option<f64>,
    pub peak_rss_mb: f64,
    pub elapsed_secs: f64,
}

impl Default for LiveStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            current_suite: None,
            current_step: "Idle".to_string(),
            total_suites: 13,
            completed_suites_count: 0,
            peak_tps: 0.0,
            best_p99_ms: None,
            peak_rss_mb: 0.0,
            elapsed_secs: 0.0,
        }
    }
}

pub struct AppState {
    pub repo_path: Option<PathBuf>,
    pub default_config: BenchmarkConfig,
    pub status: Arc<RwLock<LiveStatus>>,
    pub completed_suites: Arc<RwLock<Vec<SuiteResult>>>,
    pub last_single_report: Arc<RwLock<Option<RunReport>>>,
    pub last_comparison_report: Arc<RwLock<Option<ComparisonReport>>>,
    pub tx_log: broadcast::Sender<String>,
    pub commits_cache: Arc<RwLock<Vec<CommitInfo>>>,
}

impl AppState {
    pub fn new(repo_path: Option<PathBuf>, default_config: BenchmarkConfig) -> Self {
        let (tx_log, _) = broadcast::channel(500);
        Self {
            repo_path,
            default_config,
            status: Arc::new(RwLock::new(LiveStatus::default())),
            completed_suites: Arc::new(RwLock::new(Vec::new())),
            last_single_report: Arc::new(RwLock::new(None)),
            last_comparison_report: Arc::new(RwLock::new(None)),
            tx_log,
            commits_cache: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
