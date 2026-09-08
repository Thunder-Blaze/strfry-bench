use std::path::Path;
use std::process::Command;
use std::time::Instant;
use secp256k1::{Keypair, Secp256k1};
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::create_event;
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

    // Ensure database has sufficient events for realistic sequential scan and deep pagination
    let seed_count = if skip_heavy { 5000 } else { 20000 };
    if let Some(bin) = strfry_bin
        && bin.exists() {
            let mut check_cmd = Command::new(bin);
            if let Some(db) = db_dir {
                check_cmd.args(["--set", &format!("db={}/", db.display())]);
            }
            check_cmd.args(["scan", "{}"]);
            let scan_check = check_cmd.output();
            let existing_count = scan_check
                .map(|o| o.stdout.iter().filter(|&&b| b == b'\n').count())
                .unwrap_or(0);

            if existing_count < 1000 {
                progress_cb(&format!(
                    "Storage Suite: Pre-seeding database with {} deterministic events...",
                    seed_count
                ));
                let secp = Secp256k1::new();
                let keypair = Keypair::from_secret_key(
                    &secp,
                    &secp256k1::SecretKey::from_byte_array([77u8; 32]).unwrap(),
                );
                let mut events_buf = String::with_capacity(seed_count * 180);
                for i in 0..seed_count {
                    let mut ev = create_event(&secp, &keypair, 40);
                    ev.created_at = 1700000000 + (i as u64);
                    events_buf.push_str(&serde_json::to_string(&ev).unwrap());
                    events_buf.push('\n');
                }

                let mut import_cmd = Command::new(bin);
                if let Some(db) = db_dir {
                    import_cmd.args(["--set", &format!("db={}/", db.display())]);
                }
                import_cmd.args(["import", "--no-verify"]);
                import_cmd.stdin(std::process::Stdio::piped());
                import_cmd.stdout(std::process::Stdio::null());
                import_cmd.stderr(std::process::Stdio::null());
                if let Ok(mut child) = import_cmd.spawn() {
                    if let Some(mut stdin) = child.stdin.take() {
                        use std::io::Write;
                        let _ = stdin.write_all(events_buf.as_bytes());
                    }
                    let _ = child.wait();
                }
            }
        }

    progress_cb("Storage Suite: Measuring scan performance...");
    let mut scan_time = 0.0;
    let mut scan_tps = 0.0;

    if let Some(bin) = strfry_bin
        && bin.exists() {
            let scan_start = Instant::now();
            let mut scan_cmd = Command::new(bin);
            if let Some(db) = db_dir {
                scan_cmd.args(["--set", &format!("db={}/", db.display())]);
            }
            scan_cmd.args(["scan", "{}"]);
            let scan_res = scan_cmd.output();
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
         - **In-Core Pagination Time:** {:.2} seconds ({} pages retrieved)\n\
         - **Out-of-Core Status:** Skipped (requires Docker/cgroups 256MB memory constraints)\n",
        scan_tps,
        paginate_time,
        pages
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
            "pagination_time_sec": paginate_time,
            "pages_retrieved": pages,
            "mdb_stat": mdb_stat_out,
        }),
        log_output: log,
    }
}
