use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum TestType {
    #[default]
    Single,
    Compare,
    Matrix,
}


impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Single => write!(f, "single"),
            TestType::Compare => write!(f, "compare"),
            TestType::Matrix => write!(f, "matrix"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Suite {
    Storage,
    Ingestion,
    Concurrency,
    Websockets,
    Queries,
    Monitors,
    Negentropy,
    Plugin,
    Cli,
    Os,
    Stress,
    Backpressure,
    Deterministic,
}

impl Suite {
    pub fn all() -> Vec<Suite> {
        vec![
            Suite::Storage,
            Suite::Ingestion,
            Suite::Concurrency,
            Suite::Websockets,
            Suite::Queries,
            Suite::Monitors,
            Suite::Negentropy,
            Suite::Plugin,
            Suite::Cli,
            Suite::Os,
            Suite::Stress,
            Suite::Backpressure,
            Suite::Deterministic,
        ]
    }

    pub fn default_suites(skip_heavy: bool) -> Vec<Suite> {
        if skip_heavy {
            Self::all().into_iter().filter(|s| *s != Suite::Storage).collect()
        } else {
            Self::all()
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Suite::Storage => "storage",
            Suite::Ingestion => "ingestion",
            Suite::Concurrency => "concurrency",
            Suite::Websockets => "websockets",
            Suite::Queries => "queries",
            Suite::Monitors => "monitors",
            Suite::Negentropy => "negentropy",
            Suite::Plugin => "plugin",
            Suite::Cli => "cli",
            Suite::Os => "os",
            Suite::Stress => "stress",
            Suite::Backpressure => "backpressure",
            Suite::Deterministic => "deterministic",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Suite::Storage => "Storage (In-Core vs Out-of-Core)",
            Suite::Ingestion => "Event Ingestion Pipeline",
            Suite::Concurrency => "Concurrency & Thread Pool",
            Suite::Websockets => "WebSockets & Connections",
            Suite::Queries => "Query Engine & Indices",
            Suite::Monitors => "Active Monitors (Fanout)",
            Suite::Negentropy => "Negentropy Sync",
            Suite::Plugin => "Write Policy Plugin",
            Suite::Cli => "CLI & Dictionary Compression",
            Suite::Os => "OS-Level Metrics & WAF",
            Suite::Stress => "Stress & Adversarial Attacks",
            Suite::Backpressure => "Backpressure Performance",
            Suite::Deterministic => "Deterministic Instructions & Allocations",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Suite::Storage => "Sequential scan and deep pagination latencies under memory pressure",
            Suite::Ingestion => "Write throughput for small (50B) and large (10KB) payloads",
            Suite::Concurrency => "REQ query performance under heavy background write load",
            Suite::Websockets => "Connection storms, memory per socket (VmRSS), and socket churn",
            Suite::Queries => "Point lookup latency, NIP-45 COUNT, and complex NIP-01 filters",
            Suite::Monitors => "Subscription fanout latencies (Time-to-First & Time-to-Last client)",
            Suite::Negentropy => "NIP-77 set reconciliation synchronization speed between relays",
            Suite::Plugin => "IPC overhead when delegating validation to an external policy plugin",
            Suite::Cli => "Native CLI operations: import, export, and zstd dictionary training",
            Suite::Os => "Memory RSS, CPU utilization %, physical disk I/O, and Write Amplification",
            Suite::Stress => "Adversarial workloads: Slow Loris holding and signature flood spam",
            Suite::Backpressure => "Flow control behavior with fast and slow WebSocket clients",
            Suite::Deterministic => "Hardware-agnostic retired CPU instructions and heap allocation tracking",
        }
    }
}

impl FromStr for Suite {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "storage" => Ok(Suite::Storage),
            "ingestion" => Ok(Suite::Ingestion),
            "concurrency" => Ok(Suite::Concurrency),
            "websockets" | "websocket" | "ws" => Ok(Suite::Websockets),
            "queries" | "query" => Ok(Suite::Queries),
            "monitors" | "monitor" => Ok(Suite::Monitors),
            "negentropy" | "sync" => Ok(Suite::Negentropy),
            "plugin" => Ok(Suite::Plugin),
            "cli" | "dict" | "cli_dict" => Ok(Suite::Cli),
            "os" | "system" => Ok(Suite::Os),
            "stress" | "malicious" => Ok(Suite::Stress),
            "backpressure" | "bp" => Ok(Suite::Backpressure),
            "deterministic" | "perf" | "alloc" => Ok(Suite::Deterministic),
            other => Err(format!("Unknown suite: {}", other)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub path: Option<PathBuf>,
    pub url: String,
    pub test_type: TestType,
    pub base: String,
    pub target: String,
    pub current: bool,
    pub commits: Vec<String>,
    pub suites: Vec<Suite>,
    pub skip_heavy: bool,
    pub full: bool,
    pub high_performance: bool,
    pub flamegraph: bool,
    pub alloc_tracker: bool,
    pub perf_stat: bool,
    pub web: bool,
    pub web_port: u16,
    pub output_dir: PathBuf,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            path: None,
            url: "ws://localhost:7777".to_string(),
            test_type: TestType::Single,
            base: "master".to_string(),
            target: "HEAD".to_string(),
            current: false,
            commits: vec![],
            suites: Suite::all(),
            skip_heavy: false,
            full: false,
            high_performance: false,
            flamegraph: false,
            alloc_tracker: false,
            perf_stat: false,
            web: false,
            web_port: 7787,
            output_dir: PathBuf::from("report"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterministicMetrics {
    pub instructions: u64,
    pub allocations: u64,
    pub allocated_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteResult {
    pub id: String,
    pub name: String,
    pub status: SuiteStatus,
    pub elapsed_secs: f64,
    pub throughput: Option<f64>,
    pub throughput_label: Option<String>,
    pub p50_ms: Option<f64>,
    pub p90_ms: Option<f64>,
    pub p95_ms: Option<f64>,
    pub p99_ms: Option<f64>,
    pub memory_rss_mb: Option<f64>,
    pub metrics: serde_json::Value,
    pub log_output: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SuiteStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Skipped,
}

impl std::fmt::Display for SuiteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuiteStatus::Queued => write!(f, "queued"),
            SuiteStatus::Running => write!(f, "running"),
            SuiteStatus::Completed => write!(f, "completed"),
            SuiteStatus::Failed => write!(f, "failed"),
            SuiteStatus::Skipped => write!(f, "skipped"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunReport {
    pub id: String,
    pub title: String,
    pub test_type: TestType,
    pub target_ref: String,
    pub git_commit: Option<String>,
    pub git_branch: Option<String>,
    pub timestamp: String,
    pub suites: Vec<SuiteResult>,
    pub deterministic: Option<DeterministicMetrics>,
    pub os_info: String,
    pub cpu_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDelta {
    pub metric: String,
    pub base_value: f64,
    pub target_value: f64,
    pub delta_pct: f64,
    pub status: DeltaStatus,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeltaStatus {
    Improved,
    Regressed,
    Stable,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonReport {
    pub id: String,
    pub base_ref: String,
    pub target_ref: String,
    pub timestamp: String,
    pub base_report: RunReport,
    pub target_report: RunReport,
    pub deltas: Vec<MetricDelta>,
}
