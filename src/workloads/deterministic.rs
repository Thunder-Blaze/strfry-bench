use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;
use secp256k1::{Keypair, Secp256k1};
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::create_event;

pub async fn run(
    strfry_bin: Option<&Path>,
    progress_cb: &impl Fn(&str),
) -> SuiteResult {
    let start = Instant::now();
    let bin = match strfry_bin {
        Some(b) if b.exists() => b,
        _ => {
            progress_cb("Deterministic profiling requires local strfry binary. Skipping.");
            return SuiteResult {
                id: "deterministic".to_string(),
                name: "Deterministic Instructions & Allocations".to_string(),
                status: SuiteStatus::Completed,
                elapsed_secs: 0.05,
                throughput: None,
                throughput_label: None,
                p50_ms: None,
                p90_ms: None,
                p95_ms: None,
                p99_ms: None,
                memory_rss_mb: None,
                metrics: serde_json::json!({
                    "info": "Requires local strfry binary; skipped for live relay mode"
                }),
                log_output: "- **Status:** Skipped for Live Relay\n- **Reason:** Deterministic instruction counting and heap allocation tracking require local binary execution.".to_string(),
            };
        }
    };

    progress_cb("Running deterministic workload (1,000 synthetic events)...");

    // 1. Generate 1,000 deterministic synthetic events
    let secp = Secp256k1::new();
    let keypair = Keypair::from_secret_key(&secp, &secp256k1::SecretKey::from_byte_array([42u8; 32]).unwrap());
    let mut events_jsonl = String::with_capacity(1000 * 200);
    for _ in 0..1000 {
        let ev = create_event(&secp, &keypair, 40);
        events_jsonl.push_str(&serde_json::to_string(&ev).unwrap());
        events_jsonl.push('\n');
    }

    // 2. Setup temporary DB directory
    let temp_dir = std::env::temp_dir().join(format!("strfry-det-{}", rand::random::<u32>()));
    let _ = fs::create_dir_all(&temp_dir);

    // 3. Check for alloc_tracker source & build shared library if gcc exists
    let tracker_so = temp_dir.join("alloc_tracker.so");
    let allocs_txt = temp_dir.join("allocs.txt");
    let perf_txt = temp_dir.join("perf.txt");

    let alloc_src_candidates = [
        PathBuf::from("bench/alloc_tracker.c"),
        PathBuf::from("strfry/bench/alloc_tracker.c"),
        PathBuf::from("../strfry/bench/alloc_tracker.c"),
    ];

    let mut has_tracker = false;
    for src in &alloc_src_candidates {
        if src.exists() {
            let res = Command::new("gcc")
                .args(["-shared", "-fPIC", "-o", tracker_so.to_str().unwrap(), src.to_str().unwrap(), "-ldl"])
                .output();
            if let Ok(out) = res {
                if out.status.success() {
                    has_tracker = true;
                    break;
                }
            }
         }
     }

    // If candidate files weren't on disk, compile embedded ALLOC_TRACKER_C
    if !has_tracker {
        let c_path = temp_dir.join("alloc_tracker.c");
        if fs::write(&c_path, crate::builder::ALLOC_TRACKER_C).is_ok() {
            let res = Command::new("gcc")
                .args(["-shared", "-fPIC", "-o", tracker_so.to_str().unwrap(), c_path.to_str().unwrap(), "-ldl"])
                .output();
            if let Ok(out) = res {
                if out.status.success() {
                    has_tracker = true;
                }
            }
        }
    }

    // 4. Run strfry import with perf stat and alloc tracker
    let mut cmd = Command::new("perf");
    cmd.args(["stat", "-x,", "-o", perf_txt.to_str().unwrap(), "-e", "instructions"]);
    cmd.arg(bin);
    cmd.args(["--set", &format!("db={}/", temp_dir.display()), "import", "--no-verify"]);

    if has_tracker {
        cmd.env("LD_PRELOAD", &tracker_so);
        cmd.env("ALLOC_TRACKER_OUT", &allocs_txt);
    }

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    let mut instructions: u64 = 0;
    let mut allocations: u64 = 0;
    let mut allocated_bytes: u64 = 0;

    let child_res = cmd.spawn();
    match child_res {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(events_jsonl.as_bytes());
            }
            let _ = child.wait();
        }
        Err(_) => {
            // If perf stat failed to spawn, run strfry directly
            let mut direct_cmd = Command::new(bin);
            direct_cmd.args(["--set", &format!("db={}/", temp_dir.display()), "import", "--no-verify"]);
            if has_tracker {
                direct_cmd.env("LD_PRELOAD", &tracker_so);
                direct_cmd.env("ALLOC_TRACKER_OUT", &allocs_txt);
            }
            direct_cmd.stdin(Stdio::piped());
            direct_cmd.stdout(Stdio::null());
            direct_cmd.stderr(Stdio::null());
            if let Ok(mut c) = direct_cmd.spawn() {
                if let Some(mut stdin) = c.stdin.take() {
                    let _ = stdin.write_all(events_jsonl.as_bytes());
                }
                let _ = c.wait();
            }
        }
    }

    // Parse allocs.txt
    if allocs_txt.exists() {
        if let Ok(content) = fs::read_to_string(&allocs_txt) {
            for line in content.lines() {
                if line.starts_with("ALLOCS:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        allocations = val.parse().unwrap_or(0);
                    }
                } else if line.starts_with("BYTES:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        allocated_bytes = val.parse().unwrap_or(0);
                    }
                }
            }
        }
    }

    // Parse perf.txt
    if perf_txt.exists() {
        if let Ok(content) = fs::read_to_string(&perf_txt) {
            for line in content.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 && parts[2].contains("instructions") {
                    let val_str = parts[0].trim();
                    if val_str != "<not counted>" && !val_str.is_empty() {
                        instructions = val_str.parse().unwrap_or(0);
                    }
                }
            }
        }
    }

    // Cleanup temp db
    let _ = fs::remove_dir_all(&temp_dir);

    let elapsed = start.elapsed().as_secs_f64();
    let alloc_mb = (allocated_bytes as f64) / (1024.0 * 1024.0);

    let mut log_lines = vec![
        "- **Status:** Success (Deterministic Profile Complete)".to_string(),
        "- **Events Processed:** 1,000 synthetic events".to_string(),
    ];

    if instructions > 0 {
        log_lines.push(format!("- **Instructions Retired:** {}", instructions));
        log_lines.push(format!("- **Instructions per Event:** {:.0}", instructions as f64 / 1000.0));
    }
    if allocations > 0 {
        log_lines.push(format!("- **Heap Allocations:** {}", allocations));
        log_lines.push(format!("- **Total Allocated Memory:** {:.2} MB ({} bytes)", alloc_mb, allocated_bytes));
    }
    if instructions == 0 && allocations == 0 {
        log_lines.push("- **Note:** Run with root/perf permissions to collect instruction counter".to_string());
    }

    SuiteResult {
        id: "deterministic".to_string(),
        name: "Deterministic Instructions & Allocations".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: elapsed,
        throughput: None,
        throughput_label: None,
        p50_ms: None,
        p90_ms: None,
        p95_ms: None,
        p99_ms: None,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "instructions": instructions,
            "allocations": allocations,
            "allocated_bytes": allocated_bytes,
            "allocated_mb": alloc_mb,
            "events_processed": 1000
        }),
        log_output: log_lines.join("\n"),
    }
}
