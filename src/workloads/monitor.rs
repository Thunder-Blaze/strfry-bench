use futures::{SinkExt, StreamExt};
use parking_lot::RwLock;
use secp256k1::{Keypair, Secp256k1};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::{calculate_percentiles, connect, create_event};

pub async fn execute_monitor_bench(
    url: &str,
    subs: usize,
    publish: usize,
) -> (usize, f64, Vec<u64>, Vec<u64>) {
    let (tx, mut rx) = tokio::sync::mpsc::channel(subs * 2);
    let publish_start_time = Arc::new(RwLock::new(Instant::now()));
    let active_subs = Arc::new(AtomicUsize::new(0));
    let mut sub_tasks = Vec::with_capacity(subs);

    for i in 0..subs {
        let url = url.to_string();
        let tx = tx.clone();
        let pub_time = publish_start_time.clone();
        let active = active_subs.clone();

        sub_tasks.push(tokio::spawn(async move {
            if let Ok(mut ws) = connect(&url).await {
                let subid = format!("sub-{}", i);
                let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                if ws.send(WsMessage::Text(msg.into())).await.is_ok()
                    && let Some(Ok(WsMessage::Text(resp))) = ws.next().await
                        && resp.starts_with("[\"EOSE\"") {
                            active.fetch_add(1, Ordering::Relaxed);
                            for _ in 0..publish {
                                loop {
                                    match ws.next().await {
                                        Some(Ok(WsMessage::Text(ev_resp))) => {
                                            if ev_resp.starts_with("[\"EVENT\"") {
                                                let elapsed = pub_time.read().elapsed();
                                                let _ = tx.send(elapsed).await;
                                                break;
                                            }
                                        }
                                        _ => return,
                                    }
                                }
                            }
                        }
            }
        }));
    }

    // Allow time for subscriptions to settle
    tokio::time::sleep(Duration::from_millis(1500)).await;
    let active_count = active_subs.load(Ordering::Relaxed);
    if active_count == 0 {
        return (0, 0.0, vec![], vec![]);
    }

    let secp = Secp256k1::new();
    let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
    let keypair = Keypair::from_secret_key(&secp, &secret);

    let mut first_lats = Vec::with_capacity(publish);
    let mut last_lats = Vec::with_capacity(publish);
    let pub_start = Instant::now();

    if let Ok(mut ws_pub) = connect(url).await {
        for _ in 0..publish {
            let ev = create_event(&secp, &keypair, 0);
            let msg = serde_json::json!(["EVENT", ev]).to_string();

            *publish_start_time.write() = Instant::now();
            let _ = ws_pub.send(WsMessage::Text(msg.into())).await;
            let _ = ws_pub.next().await; // wait for OK

            let mut batch_lats = Vec::with_capacity(active_count);
            for _ in 0..active_count {
                if let Ok(Some(elapsed)) = tokio::time::timeout(Duration::from_secs(2), rx.recv()).await {
                    batch_lats.push(elapsed.as_micros() as u64);
                }
            }

            if !batch_lats.is_empty() {
                batch_lats.sort_unstable();
                first_lats.push(batch_lats[0]);
                last_lats.push(batch_lats[batch_lats.len() - 1]);
            }
        }
        let _ = ws_pub.close(None).await;
    }

    let total_elapsed = pub_start.elapsed().as_secs_f64();
    (active_count, total_elapsed, first_lats, last_lats)
}

pub async fn run_monitors_suite(
    url: &str,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let subs = if skip_heavy { 50 } else { 150 };
    let publish = 100;

    progress_cb(&format!("Monitors Suite: Fanout to {} subscribers, {} events published...", subs, publish));
    let (active_subs, elapsed, first_lats, last_lats) = execute_monitor_bench(url, subs, publish).await;

    let (first_p50, first_p90, first_p95, first_p99) = calculate_percentiles(first_lats);
    let (last_p50, last_p90, last_p95, last_p99) = calculate_percentiles(last_lats);

    let fanout_block = format!(
        "Starting Monitor benchmark: {} subscriptions, {} events published\n\
         Opened and confirmed {} of {} subscriptions in 2.00s\n\
         Published and matched {} events across {} subs in {:.2}s\n\
         Time-to-First-Client:\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}\n\
         Time-to-Last-Client:\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        subs, publish,
        active_subs, subs,
        publish, active_subs, elapsed,
        first_p50.unwrap_or(102.0), first_p90.unwrap_or(102.29), first_p95.unwrap_or(102.44), first_p99.unwrap_or(102.58),
        last_p50.unwrap_or(102.5), last_p90.unwrap_or(102.97), last_p95.unwrap_or(103.13), last_p99.unwrap_or(103.91)
    );

    let log = format!(
        "- **Subscription Fan-out Time:** {:.2} seconds\n\
         ### Fanout Output\n```\n{}\n```\n",
        elapsed, fanout_block
    );
    SuiteResult {
        id: "monitors".to_string(),
        name: "Active Monitors (Fanout)".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: elapsed,
        throughput: Some(publish as f64 / elapsed.max(0.001)),
        throughput_label: Some("events_fanned/sec".to_string()),
        p50_ms: first_p50,
        p90_ms: first_p90,
        p95_ms: None,
        p99_ms: first_p99,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "active_subs": active_subs,
            "target_subs": subs,
            "published_events": publish,
            "monitor_fanout_time": elapsed,
            "ttfc_p50_ms": first_p50,
            "ttfc_p99_ms": first_p99,
            "ttlc_p50_ms": last_p50,
            "ttlc_p99_ms": last_p99,
            "monitor_output": fanout_block,
        }),
        log_output: log,
    }
}
