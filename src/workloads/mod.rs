pub mod nostr;
pub mod event;
pub mod req;
pub mod paginate;
pub mod monitor;
pub mod connections;
pub mod churn;
pub mod backpressure;
pub mod storage;
pub mod concurrency;
pub mod negentropy;
pub mod plugin;
pub mod cli_dict;
pub mod os_stress;

use std::path::Path;
use std::time::Instant;
use crate::config::{BenchmarkConfig, Suite, SuiteResult, SuiteStatus};

pub struct SuiteRunner<'a> {
    pub config: &'a BenchmarkConfig,
    pub relay_url: String,
    pub strfry_bin: Option<&'a Path>,
    pub db_dir: Option<&'a Path>,
    pub pid: Option<u32>,
}

impl<'a> SuiteRunner<'a> {
    pub fn new(
        config: &'a BenchmarkConfig,
        relay_url: impl Into<String>,
        strfry_bin: Option<&'a Path>,
        db_dir: Option<&'a Path>,
    ) -> Self {
        Self {
            config,
            relay_url: relay_url.into(),
            strfry_bin,
            db_dir,
            pid: None,
        }
    }

    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = Some(pid);
        self
    }

    /// Execute a single suite and return its structured result
    pub async fn run_suite(
        &self,
        suite: Suite,
        progress_cb: impl Fn(&str) + Send + Sync,
    ) -> SuiteResult {
        let start = Instant::now();
        progress_cb(&format!("Starting {}", suite.display_name()));

        let mut res = match suite {
            Suite::Storage => storage::run(&self.relay_url, self.strfry_bin, self.db_dir, self.config.skip_heavy, &progress_cb).await,
            Suite::Ingestion => event::run_ingestion_suite(&self.relay_url, self.config.skip_heavy, &progress_cb).await,
            Suite::Concurrency => concurrency::run(&self.relay_url, self.config.skip_heavy, &progress_cb).await,
            Suite::Websockets => connections::run_websockets_suite(&self.relay_url, self.pid, self.config.skip_heavy, &progress_cb).await,
            Suite::Queries => req::run_queries_suite(&self.relay_url, self.strfry_bin, self.config.skip_heavy, &progress_cb).await,
            Suite::Monitors => monitor::run_monitors_suite(&self.relay_url, self.config.skip_heavy, &progress_cb).await,
            Suite::Negentropy => negentropy::run(&self.relay_url, self.strfry_bin, self.config.skip_heavy, &progress_cb).await,
            Suite::Plugin => plugin::run(&self.relay_url, self.strfry_bin, self.config.skip_heavy, &progress_cb).await,
            Suite::Cli => cli_dict::run(self.strfry_bin, self.config.skip_heavy, &progress_cb).await,
            Suite::Os => os_stress::run_os_suite(&self.relay_url, self.db_dir, self.pid, self.config.skip_heavy, &progress_cb).await,
            Suite::Stress => os_stress::run_stress_suite(&self.relay_url, self.config.skip_heavy, &progress_cb).await,
            Suite::Backpressure => backpressure::run_backpressure_suite(&self.relay_url, self.config.skip_heavy, &progress_cb).await,
            Suite::Deterministic => SuiteResult {
                id: suite.id().to_string(),
                name: suite.display_name().to_string(),
                status: SuiteStatus::Completed,
                elapsed_secs: 0.0,
                throughput: None,
                throughput_label: None,
                p50_ms: None,
                p90_ms: None,
                p95_ms: None,
                p99_ms: None,
                memory_rss_mb: None,
                metrics: serde_json::json!({ "info": "Deterministic metrics collected via perf_stat/alloc_tracker" }),
                log_output: "Deterministic profiling configured via profiler module".to_string(),
            },
        };

        if res.elapsed_secs == 0.0 {
            res.elapsed_secs = start.elapsed().as_secs_f64();
        }

        progress_cb(&format!("Finished {} in {:.2}s", suite.display_name(), res.elapsed_secs));
        res
    }
}
