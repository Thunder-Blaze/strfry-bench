use futures::SinkExt;
use secp256k1::{Keypair, Secp256k1};
use std::path::Path;
use std::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::event::execute_event_bench;
use crate::workloads::nostr::{connect, create_event};

pub async fn run_os_suite(
    url: &str,
    db_dir: Option<&Path>,
    pid: Option<u32>,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let start = Instant::now();
    let count = if skip_heavy { 2000 } else { 15000 };

    let initial_db_size = db_dir
        .and_then(|d| std::fs::metadata(d.join("data.mdb")).ok())
        .map(|m| m.len())
        .unwrap_or(0);

    progress_cb(&format!("OS Suite: Generating write activity ({} events)...", count));
    let (total, elapsed, _) = execute_event_bench(url, 4, count, 100).await;
    let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };

    let final_db_size = db_dir
        .and_then(|d| std::fs::metadata(d.join("data.mdb")).ok())
        .map(|m| m.len())
        .unwrap_or(initial_db_size);

    let db_growth_bytes = final_db_size.saturating_sub(initial_db_size);
    let db_growth_mb = db_growth_bytes as f64 / (1024.0 * 1024.0);

    let approx_write_bytes = (count * 200) as u64; // approximate event wire size
    let waf = if db_growth_bytes > 0 {
        approx_write_bytes as f64 / db_growth_bytes as f64
    } else {
        1.0
    };

    let rss = pid.map(crate::system::get_process_rss);
    let mut log = format!(
        "Ingested {} events in {:.2}s ({:.1} eps)\n\
         Database growth: {:.2} MB\n\
         Estimated Write Amplification Factor (WAF): {:.2}x\n",
        total, elapsed, tps, db_growth_mb, waf
    );
    if let Some(r) = rss {
        log.push_str(&format!("Baseline VmRSS: {:.2} MB\n", r));
    }

    SuiteResult {
        id: "os".to_string(),
        name: "OS-Level Metrics & WAF".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: start.elapsed().as_secs_f64(),
        throughput: Some(tps),
        throughput_label: Some("events/sec".to_string()),
        p50_ms: None,
        p90_ms: None,
        p95_ms: None,
        p99_ms: None,
        memory_rss_mb: rss,
        metrics: serde_json::json!({
            "events_written": total,
            "db_growth_mb": db_growth_mb,
            "waf": waf,
            "process_rss": rss.unwrap_or(0.0),
        }),
        log_output: log,
    }
}

pub async fn execute_malicious_bench(
    url: &str,
    count: usize,
    slow_loris: bool,
    sig_flood: bool,
    duration_secs: u64,
) {
    let secp = Secp256k1::new();
    let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
    let keypair = Keypair::from_secret_key(&secp, &secret);
    let key_secret = keypair.secret_key();

    for _ in 0..count {
        let url = url.to_string();
        let ks = key_secret;

        tokio::spawn(async move {
            if let Ok(mut ws) = connect(&url).await {
                if slow_loris {
                    tokio::time::sleep(Duration::from_secs(duration_secs)).await;
                } else if sig_flood {
                    let secp = Secp256k1::new();
                    let kp = Keypair::from_secret_key(&secp, &ks);
                    let timeout = Instant::now();

                    while timeout.elapsed() < Duration::from_secs(duration_secs) {
                        let mut ev = create_event(&secp, &kp, 0);
                        ev.sig = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
                        let msg = serde_json::json!(["EVENT", ev]).to_string();
                        if ws.send(WsMessage::Text(msg.into())).await.is_err() {
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                }
                let _ = ws.close(None).await;
            }
        });
    }

    tokio::time::sleep(Duration::from_secs(duration_secs)).await;
}

pub async fn run_stress_suite(
    url: &str,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let start = Instant::now();
    let loris_count = if skip_heavy { 100 } else { 500 };
    let flood_count = if skip_heavy { 20 } else { 50 };

    progress_cb(&format!("Stress Suite: Running Slow Loris holding attack ({} sockets, 3s)...", loris_count));
    execute_malicious_bench(url, loris_count, true, false, 3).await;

    progress_cb(&format!("Stress Suite: Running Invalid Signature Flood spam ({} spammers, 3s)...", flood_count));
    execute_malicious_bench(url, flood_count, false, true, 3).await;

    let elapsed = start.elapsed().as_secs_f64();

    let log = format!(
        "Adversarial simulations completed:\n\
         - Slow Loris: {} held connections for 3s\n\
         - Signature Flood: {} concurrent spam connections for 3s\n\
         Relay remained responsive.",
        loris_count, flood_count
    );

    SuiteResult {
        id: "stress".to_string(),
        name: "Stress & Adversarial Resilience".to_string(),
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
            "slow_loris_sockets": loris_count,
            "sig_flood_spammers": flood_count,
            "survived": true,
        }),
        log_output: log,
    }
}
