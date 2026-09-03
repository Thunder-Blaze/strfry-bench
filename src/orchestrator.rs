use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::broadcast;
use crate::analysis::{compute_comparison_deltas, write_comparison_report, write_single_report};
use crate::builder::Builder;
use crate::config::{BenchmarkConfig, ComparisonReport, DeterministicMetrics, RunReport, Suite, SuiteResult, TestType};
use crate::git::{GitManager, StashGuard};
use crate::profiler::{generate_flamegraph_from_perf, parse_alloc_tracker_output, run_perf_stat};
use crate::relay::{wait_for_relay_ready, RelaySupervisor};
use crate::system::SystemInfo;
use crate::workloads::SuiteRunner;

#[derive(Clone, Debug)]
pub enum ProgressEvent {
    RunStarted {
        total_suites: usize,
    },
    Step {
        message: String,
    },
    SuiteStarted {
        suite: Suite,
        index: usize,
        total: usize,
    },
    SuiteCompleted {
        result: SuiteResult,
        completed: usize,
        total: usize,
        elapsed_secs: f64,
        peak_rss_mb: f64,
    },
    RunFinished {
        output_dir: PathBuf,
        elapsed_secs: f64,
    },
    RunFailed {
        error: String,
    },
}

pub struct Orchestrator {
    pub config: BenchmarkConfig,
    pub tx_log: Option<broadcast::Sender<String>>,
    pub progress: Option<Arc<dyn Fn(ProgressEvent) + Send + Sync>>,
}

impl Orchestrator {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            tx_log: None,
            progress: None,
        }
    }

    pub fn with_log_sender(mut self, tx: broadcast::Sender<String>) -> Self {
        self.tx_log = Some(tx);
        self
    }

    pub fn with_progress(mut self, progress: Arc<dyn Fn(ProgressEvent) + Send + Sync>) -> Self {
        self.progress = Some(progress);
        self
    }

    fn emit(&self, event: ProgressEvent) {
        if let Some(progress) = &self.progress {
            progress(event);
        }
    }

    fn log(&self, msg: &str) {
        println!("{}", msg);
        if let Some(tx) = &self.tx_log {
            let _ = tx.send(msg.to_string());
        }
    }

    /// Execute the complete benchmark according to configuration
    pub async fn run(&self) -> Result<PathBuf, String> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let started = Instant::now();
        self.emit(ProgressEvent::RunStarted { total_suites: self.config.suites.len() });

        let result = if let Some(repo_path) = &self.config.path {
            let git = GitManager::new(repo_path);
            if !git.is_git_repo() {
                Err(format!("Path {} is not a valid git repository", repo_path.display()))
            } else {
                match self.config.test_type {
                    TestType::Compare => self.run_comparison_benchmark(&git, &timestamp).await,
                    _ => self.run_single_git_benchmark(&git, &timestamp).await,
                }
            }
        } else {
            self.run_live_benchmark(&timestamp).await
        };

        match &result {
            Ok(out_dir) => self.emit(ProgressEvent::RunFinished {
                output_dir: out_dir.clone(),
                elapsed_secs: started.elapsed().as_secs_f64(),
            }),
            Err(error) => self.emit(ProgressEvent::RunFailed { error: error.clone() }),
        }

        result
    }

    async fn run_live_benchmark(&self, timestamp: &str) -> Result<PathBuf, String> {
        self.log(&format!("[BENCH] Target: Live relay at {}", self.config.url));
        wait_for_relay_ready(&self.config.url, 5).await?;

        let run_id = format!("{}_live", timestamp);
        let out_dir = self.config.output_dir.join(&run_id);
        std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

        let sys_info = SystemInfo::collect();
        let runner = SuiteRunner::new(&self.config, &self.config.url, None, None);

        let mut suite_results = Vec::new();
        let total_suites = self.config.suites.len();
        let run_started = Instant::now();
        for (idx, &suite) in self.config.suites.iter().enumerate() {
            self.emit(ProgressEvent::SuiteStarted { suite, index: idx + 1, total: total_suites });
            let res = runner.run_suite(suite, |msg| self.log(msg)).await;
            self.emit(ProgressEvent::SuiteCompleted {
                result: res.clone(),
                completed: idx + 1,
                total: total_suites,
                elapsed_secs: run_started.elapsed().as_secs_f64(),
                peak_rss_mb: 0.0,
            });
            suite_results.push(res);
        }

        let report = RunReport {
            id: run_id,
            title: format!("Live Relay Benchmark ({})", self.config.url),
            test_type: TestType::Single,
            target_ref: self.config.url.clone(),
            git_commit: None,
            git_branch: None,
            timestamp: chrono::Local::now().to_rfc3339(),
            suites: suite_results,
            deterministic: None,
            os_info: format!("{} ({})", sys_info.os_name, sys_info.kernel_version),
            cpu_info: format!("{} ({} cores)", sys_info.cpu_brand, sys_info.physical_cores),
        };

        write_single_report(&out_dir, &report)?;
        Ok(out_dir)
    }

    async fn run_single_git_benchmark(&self, git: &GitManager, timestamp: &str) -> Result<PathBuf, String> {
        let commit = git.get_current_commit()?;
        let branch = git.get_current_branch().unwrap_or_else(|_| "HEAD".to_string());
        let short_commit = if commit.len() >= 7 { &commit[..7] } else { &commit };

        let run_id = format!("{}_single_{}", timestamp, short_commit);
        let out_dir = self.config.output_dir.join(&run_id);
        std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

        self.log(&format!("[BENCH] Building strfry at {} ({})", branch, short_commit));
        let builder = Builder::new(&git.repo_path, self.config.high_performance);
        let binary_path = builder.build_strfry()?;

        // Build alloc_tracker if requested
        let tracker_so = if self.config.alloc_tracker {
            builder.build_alloc_tracker(&out_dir)?
        } else {
            None
        };

        let db_dir = out_dir.join("strfry-db-bench");
        let mut relay = RelaySupervisor::new(&binary_path, &db_dir);

        let allocs_file = out_dir.join("allocs.txt");
        let perf_data = out_dir.join("perf.data");
        let flamegraph_svg = out_dir.join("flamegraph.svg");
        let pid = relay.start(
            None,
            tracker_so.as_deref(),
            if self.config.alloc_tracker { Some(&allocs_file) } else { None },
            if self.config.flamegraph { Some(&perf_data) } else { None },
        )?;

        let monitor = crate::system::ProcessMonitor::start(pid);
        wait_for_relay_ready("ws://127.0.0.1:7777", 10).await?;
        let sys_info = SystemInfo::collect();
        let runner = SuiteRunner::new(&self.config, "ws://127.0.0.1:7777", Some(&binary_path), Some(&db_dir)).with_pid(pid);

        let mut suite_results = Vec::new();
        let total_suites = self.config.suites.len();
        let run_started = Instant::now();
        for (idx, &suite) in self.config.suites.iter().enumerate() {
            self.emit(ProgressEvent::SuiteStarted { suite, index: idx + 1, total: total_suites });
            let mut res = runner.run_suite(suite, |msg| self.log(msg)).await;
            let snap = monitor.snapshot();
            res.memory_rss_mb = Some(snap.rss_mb);
            if let Some(obj) = res.metrics.as_object_mut() {
                obj.insert("process_rss_mb".to_string(), serde_json::json!(snap.rss_mb));
                obj.insert("peak_rss_mb".to_string(), serde_json::json!(snap.peak_rss_mb));
                obj.insert("cpu_percent".to_string(), serde_json::json!(snap.cpu_pct));
            }
            self.emit(ProgressEvent::SuiteCompleted {
                result: res.clone(),
                completed: idx + 1,
                total: total_suites,
                elapsed_secs: run_started.elapsed().as_secs_f64(),
                peak_rss_mb: snap.peak_rss_mb,
            });
            suite_results.push(res);
        }
        let _ = monitor.stop();
        relay.stop();

        if self.config.flamegraph && perf_data.exists() {
            let _ = generate_flamegraph_from_perf(&perf_data, &flamegraph_svg);
        }
        // Deterministic metrics
        // Deterministic metrics & instructions
        let mut det_metrics = None;
        let mut instructions = 0u64;
        if self.config.perf_stat {
            let perf_file = out_dir.join("perf.txt");
            if let Ok(instr) = run_perf_stat(&[binary_path.to_str().unwrap(), "--version"], &perf_file) {
                instructions = instr;
            }
        }

        if self.config.alloc_tracker || self.config.perf_stat {
            let (allocs, bytes) = if self.config.alloc_tracker {
                parse_alloc_tracker_output(&allocs_file)
            } else {
                (0, 0)
            };
            det_metrics = Some(DeterministicMetrics {
                instructions,
                allocations: allocs,
                allocated_bytes: bytes,
            });
        }

        // Flamegraph already generated if requested
        let report = RunReport {
            id: run_id,
            title: format!("strfry Benchmark - {} ({})", branch, short_commit),
            test_type: TestType::Single,
            target_ref: branch.clone(),
            git_commit: Some(commit),
            git_branch: Some(branch),
            timestamp: chrono::Local::now().to_rfc3339(),
            suites: suite_results,
            deterministic: det_metrics,
            os_info: format!("{} ({})", sys_info.os_name, sys_info.kernel_version),
            cpu_info: format!("{} ({} cores)", sys_info.cpu_brand, sys_info.physical_cores),
        };

        write_single_report(&out_dir, &report)?;
        Ok(out_dir)
    }

    async fn run_comparison_benchmark(&self, git: &GitManager, timestamp: &str) -> Result<PathBuf, String> {
        let base_ref = &self.config.base;
        let target_ref = if self.config.current {
            "current-codebase"
        } else {
            &self.config.target
        };

        let run_id = format!("{}_compare_{}_vs_{}", timestamp, base_ref.replace('/', "_"), target_ref.replace('/', "_"));
        let out_dir = self.config.output_dir.join(&run_id);
        std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

        self.log(&format!("\n=== RUNNING FINAL (TARGET): {} ===", target_ref));
        let (target_report, guard_for_clean_run) = if self.config.current {
            // Benchmark current working tree first
            let rep = self.benchmark_current_worktree(git, &out_dir, "final").await?;
            (rep, None)
        } else {
            let guard = StashGuard::enter(git, "strfry_bench_auto_stash")?;
            git.checkout(target_ref)?;
            let rep = self.benchmark_current_worktree(git, &out_dir, "final").await?;
            (rep, Some(guard))
        };

        // Stash current changes to checkout initial (base) commit safely
        self.log(&format!("\n=== RUNNING INITIAL (BASE): {} ===", base_ref));
        let mut stash_guard = StashGuard::enter(git, "strfry_bench_auto_stash")?;
        git.checkout(base_ref)?;

        let base_report = self.benchmark_current_worktree(git, &out_dir, "initial").await?;
        stash_guard.restore()?;

        if let Some(mut g) = guard_for_clean_run {
            g.restore()?;
        }

        let deltas = compute_comparison_deltas(&base_report, &target_report);

        let comparison = ComparisonReport {
            id: run_id,
            base_ref: base_ref.clone(),
            target_ref: target_ref.to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
            base_report,
            target_report,
            deltas,
        };

        if self.config.flamegraph {
            let target_svg = out_dir.join("final").join("flamegraph.svg");
            let base_svg = out_dir.join("initial").join("flamegraph.svg");
            if target_svg.exists() {
                let _ = std::fs::copy(&target_svg, out_dir.join("final_flamegraph.svg"));
            }
            if base_svg.exists() {
                let _ = std::fs::copy(&base_svg, out_dir.join("initial_flamegraph.svg"));
            }
        }

        write_comparison_report(&out_dir, &comparison)?;
        Ok(out_dir)
    }

    async fn benchmark_current_worktree(&self, git: &GitManager, parent_out_dir: &Path, role: &str) -> Result<RunReport, String> {
        let commit = git.get_current_commit()?;
        let branch = git.get_current_branch().unwrap_or_else(|_| "HEAD".to_string());
        let short_commit = if commit.len() >= 7 { &commit[..7] } else { &commit };

        self.log(&format!("[BENCH] Building {} on {} ({})", role, branch, short_commit));
        let builder = Builder::new(&git.repo_path, self.config.high_performance);
        let binary_path = builder.build_strfry()?;

        let role_dir = parent_out_dir.join(role);
        std::fs::create_dir_all(&role_dir).map_err(|e| e.to_string())?;

        let tracker_so = if self.config.alloc_tracker {
            builder.build_alloc_tracker(&role_dir)?
        } else {
            None
        };

        let db_dir = role_dir.join("strfry-db");
        let mut relay = RelaySupervisor::new(&binary_path, &db_dir);

        let allocs_file = role_dir.join("allocs.txt");
        let perf_data = role_dir.join("perf.data");
        let flamegraph_svg = role_dir.join("flamegraph.svg");
        let pid = relay.start(
            None,
            tracker_so.as_deref(),
            if self.config.alloc_tracker { Some(&allocs_file) } else { None },
            if self.config.flamegraph { Some(&perf_data) } else { None },
        )?;

        let monitor = crate::system::ProcessMonitor::start(pid);
        wait_for_relay_ready("ws://127.0.0.1:7777", 10).await?;
        let sys_info = SystemInfo::collect();
        let runner = SuiteRunner::new(&self.config, "ws://127.0.0.1:7777", Some(&binary_path), Some(&db_dir)).with_pid(pid);

        let mut suite_results = Vec::new();
        let total_suites = self.config.suites.len();
        let run_started = Instant::now();
        for (idx, &suite) in self.config.suites.iter().enumerate() {
            self.emit(ProgressEvent::SuiteStarted { suite, index: idx + 1, total: total_suites });
            let mut res = runner.run_suite(suite, |msg| self.log(msg)).await;
            let snap = monitor.snapshot();
            res.memory_rss_mb = Some(snap.rss_mb);
            if let Some(obj) = res.metrics.as_object_mut() {
                obj.insert("process_rss_mb".to_string(), serde_json::json!(snap.rss_mb));
                obj.insert("peak_rss_mb".to_string(), serde_json::json!(snap.peak_rss_mb));
                obj.insert("cpu_percent".to_string(), serde_json::json!(snap.cpu_pct));
            }
            self.emit(ProgressEvent::SuiteCompleted {
                result: res.clone(),
                completed: idx + 1,
                total: total_suites,
                elapsed_secs: run_started.elapsed().as_secs_f64(),
                peak_rss_mb: snap.peak_rss_mb,
            });
            suite_results.push(res);
        }
        let _ = monitor.stop();
        relay.stop();

        if self.config.flamegraph && perf_data.exists() {
            let _ = generate_flamegraph_from_perf(&perf_data, &flamegraph_svg);
        }
        let mut det_metrics = None;
        if self.config.alloc_tracker {
            let (allocs, bytes) = parse_alloc_tracker_output(&allocs_file);
            det_metrics = Some(DeterministicMetrics {
                instructions: 0,
                allocations: allocs,
                allocated_bytes: bytes,
            });
        }

        Ok(RunReport {
            id: role.to_string(),
            title: format!("{} ({})", role, short_commit),
            test_type: TestType::Single,
            target_ref: branch.clone(),
            git_commit: Some(commit),
            git_branch: Some(branch),
            timestamp: chrono::Local::now().to_rfc3339(),
            suites: suite_results,
            deterministic: det_metrics,
            os_info: format!("{} ({})", sys_info.os_name, sys_info.kernel_version),
            cpu_info: format!("{} ({} cores)", sys_info.cpu_brand, sys_info.physical_cores),
        })
    }
}
