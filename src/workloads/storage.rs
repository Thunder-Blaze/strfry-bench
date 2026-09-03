use std::path::Path;
use std::process::Command;
use std::time::Instant;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::paginate::execute_paginate_bench;

pub async fn run(
    url: &str,
    strfry_bin: Option<&Path>,
    db_dir: Option<&Path>,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let start = Instant::now();
    let depth = if skip_heavy { 10 } else { 25 };
    let concurrency = 2;

    progress_cb("Storage Suite: Measuring scan performance...");
    let mut scan_time = 0.0;
    let mut scan_tps = 0.0;

    if let Some(bin) = strfry_bin
        && bin.exists() {
            let scan_start = Instant::now();
            let scan_res = Command::new(bin).args(["scan", "{}"]).output();
            if let Ok(out) = scan_res {
                scan_time = scan_start.elapsed().as_secs_f64();
                let lines_count = out.stdout.iter().filter(|&&b| b == b'\n').count();
                if scan_time > 0.0 {
                    scan_tps = lines_count as f64 / scan_time;
                }
            }
        }

    progress_cb(&format!("Storage Suite: Running deep pagination (depth {}, concurrency {})...", depth, concurrency));
    let (pages, paginate_time, page_lats) = execute_paginate_bench(url, depth, concurrency).await;

    let mut mdb_stat_out = String::new();
    if let Some(db) = db_dir {
        let stat_res = Command::new("mdb_stat").args(["-e", db.to_str().unwrap()]).output();
        if let Ok(out) = stat_res {
            mdb_stat_out = String::from_utf8_lossy(&out.stdout).to_string();
        }
    }

    let mut log = format!(
        "- **Sequential scan throughput (events/sec):** {:.2}\n\
         - **In-Core Pagination Time:** {:.2} seconds\n\
         - **Out-of-Core Pagination Time (256MB RAM):** {:.2} seconds\n",
        scan_tps,
        paginate_time,
        -1.0
    );
    if !mdb_stat_out.trim().is_empty() {
        log.push_str(&format!("\n### DB Stat Output\n```\n{}\n```\n", mdb_stat_out.trim()));
    }

    let p50 = if !page_lats.is_empty() {
        let mut sorted = page_lats.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        Some(sorted[sorted.len() / 2] * 1000.0)
    } else {
        None
    };
    let elapsed = start.elapsed().as_secs_f64();

    SuiteResult {
        id: "storage".to_string(),
        name: "Storage (In-Core vs Out-of-Core)".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: elapsed,
        throughput: Some(scan_tps),
        throughput_label: Some("scan_eps".to_string()),
        p50_ms: p50,
        p90_ms: None,
        p95_ms: None,
        p99_ms: None,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "scan_time_sec": scan_time,
            "scan_tps": scan_tps,
            "in_core_time": paginate_time,
            "out_of_core_time": -1.0,
            "pagination_time_sec": paginate_time,
            "pages_retrieved": pages,
            "mdb_stat": mdb_stat_out,
        }),
        log_output: log,
    }
}
