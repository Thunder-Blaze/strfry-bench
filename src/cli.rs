use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::config::{BenchmarkConfig, Suite, TestType};

#[derive(Parser, Debug)]
#[command(
    name = "strfry-bench",
    author = "Doug Hoyte & Contributors",
    version = "0.2.0",
    about = "Unified high-performance benchmarking, profiling, and comparison tool for strfry",
    long_about = "strfry-bench orchestrates reproducible benchmarking, relative A/B commit comparisons, \
hardware-agnostic profiling (perf stat, alloc_tracker), flamegraph rendering, and an interactive Web UI."
)]
pub struct Cli {
    /// Path to strfry Git repository (if omitted, defaults to live test against --url)
    #[arg(short, long, value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Target relay WebSocket URL (used when --path is omitted or for live testing)
    #[arg(short, long, default_value = "ws://localhost:7777", value_name = "URL")]
    pub url: String,

    /// Test execution mode: single | compare | matrix
    #[arg(short, long, value_enum, default_value_t = TestType::Single)]
    pub test_type: TestType,

    /// Base commit or branch for comparison
    #[arg(long, default_value = "master", value_name = "REF")]
    pub base: String,

    /// Target commit or branch for comparison
    #[arg(long, default_value = "HEAD", value_name = "REF")]
    pub target: String,

    /// Compare current codebase (including uncommitted edits via git stash) against --base
    #[arg(long)]
    pub current: bool,

    /// Comma-separated list or range of commits for matrix traversal (e.g. HEAD~5..HEAD)
    #[arg(long, value_delimiter = ',', value_name = "COMMITS")]
    pub commits: Vec<String>,

    /// Comma-separated benchmark suites to execute (default: all)
    #[arg(long, value_delimiter = ',', value_name = "SUITES")]
    pub suites: Option<Vec<String>>,

    /// Alias to run a single specific suite
    #[arg(long, value_name = "SUITE")]
    pub suite: Option<String>,

    /// Skip heavy database generation (e.g. 1M event storage test) for faster local runs
    #[arg(long)]
    pub skip_heavy: bool,

    /// Run full production workloads including out-of-core memory stress
    #[arg(long)]
    pub full: bool,

    /// Use make -j$(nproc) instead of default make -j4 in builder
    #[arg(long)]
    pub high_performance: bool,

    /// Generate SVG CPU flamegraphs via perf + inferno
    #[arg(long)]
    pub flamegraph: bool,

    /// Enable LD_PRELOAD deterministic heap allocation and byte tracking
    #[arg(long)]
    pub alloc_tracker: bool,

    /// Enable hardware instruction counting via perf stat
    #[arg(long)]
    pub perf_stat: bool,

    /// Launch interactive Web UI on localhost:7787
    #[arg(short, long)]
    pub web: bool,

    /// Port for Web UI server
    #[arg(long, default_value_t = 7787, value_name = "PORT")]
    pub web_port: u16,

    /// Output directory for reports and artifacts
    #[arg(short, long, default_value = "report", value_name = "DIR")]
    pub output: PathBuf,

    /// Legacy / standalone load testing subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Ingest events at high concurrency
    Event {
        url: String,
        #[arg(short, long, default_value_t = 10)]
        concurrency: usize,
        #[arg(short = 'n', long, default_value_t = 10000)]
        count: usize,
        #[arg(short, long, default_value_t = 0)]
        payload_size: usize,
    },
    /// Query events or NIP-45 COUNT with filters
    Req {
        url: String,
        #[arg(short, long, default_value_t = 10)]
        concurrency: usize,
        #[arg(short = 'n', long, default_value_t = 10000)]
        count: usize,
        #[arg(short, long, default_value = "{}")]
        filter: String,
        #[arg(long, default_value_t = false)]
        nip45: bool,
    },
    /// Deep pagination query traversal
    Paginate {
        url: String,
        #[arg(short, long, default_value_t = 10)]
        depth: usize,
        #[arg(short, long, default_value_t = 1)]
        concurrency: usize,
    },
    /// Active monitor fanout to multiple subscribers
    Monitor {
        url: String,
        #[arg(short, long, default_value_t = 100)]
        subs: usize,
        #[arg(short, long, default_value_t = 100)]
        publish: usize,
    },
    /// Connection storm benchmark
    Connections {
        url: String,
        #[arg(short, long, default_value_t = 1000)]
        count: usize,
    },
    /// Rapid connection churn benchmark
    Churn {
        url: String,
        #[arg(short, long, default_value_t = 100)]
        concurrency: usize,
        #[arg(short = 'n', long, default_value_t = 1000)]
        count: usize,
    },
    /// Flow control backpressure testing with mixed client speeds
    Backpressure {
        url: String,
        #[arg(long, default_value_t = 100)]
        fast_clients: usize,
        #[arg(long, default_value_t = 10)]
        slow_clients: usize,
        #[arg(short = 'n', long, default_value_t = 1000)]
        count: usize,
        #[arg(long, default_value_t = 50)]
        slow_delay: u64,
    },
    /// Generate deterministic synthetic events to stdout
    Generate {
        #[arg(short, long, default_value_t = 100000)]
        count: usize,
        #[arg(short, long, default_value_t = 100)]
        authors: usize,
        #[arg(short, long, default_value_t = 0)]
        seed: u64,
        #[arg(short, long, default_value_t = 0)]
        payload_size: usize,
    },
    /// Adversarial traffic simulation
    Malicious {
        url: String,
        #[arg(short, long, default_value_t = 1000)]
        count: usize,
        #[arg(long, default_value_t = false)]
        slow_loris: bool,
        #[arg(long, default_value_t = false)]
        sig_flood: bool,
        #[arg(short, long, default_value_t = 5)]
        duration: u64,
    },
}

impl Cli {
    pub fn to_benchmark_config(&self) -> BenchmarkConfig {
        let skip_heavy = if self.full {
            false
        } else {
            self.skip_heavy
        };

        let suites = if let Some(single_suite) = &self.suite {
            match single_suite.parse::<Suite>() {
                Ok(s) => vec![s],
                Err(err) => {
                    eprintln!("[ERROR] {}", err);
                    Suite::default_suites(skip_heavy)
                }
            }
        } else if let Some(list) = &self.suites {
            let mut res = Vec::new();
            for item in list {
                if item.to_lowercase() == "all" {
                    res = Suite::all();
                    break;
                }
                match item.parse::<Suite>() {
                    Ok(s) => res.push(s),
                    Err(err) => eprintln!("[WARNING] {}", err),
                }
            }
            if res.is_empty() {
                Suite::default_suites(skip_heavy)
            } else {
                res
            }
        } else {
            Suite::default_suites(skip_heavy)
        };

        BenchmarkConfig {
            path: self.path.clone(),
            url: self.url.clone(),
            test_type: if self.current {
                TestType::Compare
            } else {
                self.test_type
            },
            base: self.base.clone(),
            target: self.target.clone(),
            current: self.current,
            commits: self.commits.clone(),
            suites,
            skip_heavy,
            full: self.full,
            high_performance: self.high_performance,
            flamegraph: self.flamegraph,
            alloc_tracker: self.alloc_tracker,
            perf_stat: self.perf_stat,
            web: self.web,
            web_port: self.web_port,
            output_dir: self.output.clone(),
        }
    }
}
