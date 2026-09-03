use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::create_event;
use secp256k1::{Keypair, Secp256k1};

pub async fn run(
    strfry_bin: Option<&Path>,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let start = Instant::now();
    let events_count = if skip_heavy { 2000 } else { 20000 };

    let mut import_time = 0.0;
    let mut import_tps = 0.0;
    let mut export_time = 0.0;
    let mut dict_time = 0.0;

    if let Some(bin) = strfry_bin {
        if bin.exists() {
            let temp_db = tempfile::tempdir().map_err(|e| e.to_string());
            if let Ok(db_dir) = temp_db {
                let db_arg = format!("db={}/", db_dir.path().display());

                // 1. Ingest via strfry import
                progress_cb(&format!("CLI Suite: Streaming {} events to 'strfry import'...", events_count));
                let import_start = Instant::now();

                let child = Command::new(bin)
                    .args(["--set", &db_arg, "import", "--no-verify"])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn();

                if let Ok(mut c) = child {
                    if let Some(mut stdin) = c.stdin.take() {
                        let secp = Secp256k1::new();
                        let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
                        let kp = Keypair::from_secret_key(&secp, &secret);

                        for _ in 0..events_count {
                            let ev = create_event(&secp, &kp, 50);
                            let json = serde_json::to_string(&ev).unwrap();
                            let _ = writeln!(stdin, "{}", json);
                        }
                    }
                    let _ = c.wait();
                    import_time = import_start.elapsed().as_secs_f64();
                    import_tps = if import_time > 0.0 { events_count as f64 / import_time } else { 0.0 };
                }

                // 2. Export
                progress_cb("CLI Suite: Testing 'strfry export'...");
                let export_start = Instant::now();
                let _ = Command::new(bin)
                    .args(["--set", &db_arg, "export"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .output();
                export_time = export_start.elapsed().as_secs_f64();

                // 3. Train dictionary
                progress_cb("CLI Suite: Testing 'strfry dict train'...");
                let dict_start = Instant::now();
                let _ = Command::new(bin)
                    .args(["--set", &db_arg, "dict", "train"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .output();
                dict_time = dict_start.elapsed().as_secs_f64();
            }
        }
    } else {
        progress_cb("CLI Suite: Skipped binary-level import/export (live relay mode)");
    }

    let elapsed = start.elapsed().as_secs_f64();

    let log = format!(
        "- **Import Time:** {:.2} seconds\n\
         - **Export Time:** {:.2} seconds\n\
         - **Dictionary Generation Time:** {:.2} seconds\n",
        import_time, export_time, dict_time
    );

    SuiteResult {
        id: "cli".to_string(),
        name: "CLI & Dictionary Compression".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: elapsed,
        throughput: Some(import_tps),
        throughput_label: Some("import_eps".to_string()),
        p50_ms: None,
        p90_ms: None,
        p95_ms: None,
        p99_ms: None,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "import_time": import_time,
            "export_time": export_time,
            "dict_gen_time": dict_time,
            "import_time_sec": import_time,
            "import_tps": import_tps,
            "export_time_sec": export_time,
            "dict_train_time_sec": dict_time,
        }),
        log_output: log,
    }
}
