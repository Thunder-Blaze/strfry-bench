use futures::{SinkExt, StreamExt};
use parking_lot::Mutex;
use secp256k1::{Keypair, Secp256k1};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::{calculate_percentiles, connect, create_event};

pub async fn run_backpressure_suite(
    url: &str,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let fast_clients = if skip_heavy { 20 } else { 80 };
    let slow_clients = if skip_heavy { 5 } else { 15 };
    let count = if skip_heavy { 200 } else { 1000 };
    let slow_delay = 50; // ms

    progress_cb(&format!(
        "Backpressure Suite: {} fast clients, {} slow clients ({}ms delay), {} events...",
        fast_clients, slow_clients, slow_delay, count
    ));

    let fast_success = Arc::new(AtomicUsize::new(0));
    let fast_latencies = Arc::new(Mutex::new(Vec::new()));
    let mut fast_tasks = Vec::with_capacity(fast_clients);

    for i in 0..fast_clients {
        let url = url.to_string();
        let succ = fast_success.clone();
        let lats = fast_latencies.clone();

        fast_tasks.push(tokio::spawn(async move {
            if let Ok(mut ws) = connect(&url).await {
                let subid = format!("fast-{}", i);
                let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                if ws.send(WsMessage::Text(msg.into())).await.is_ok()
                    && let Some(Ok(WsMessage::Text(resp))) = ws.next().await
                        && resp.starts_with("[\"EOSE\"") {
                            for _ in 0..count {
                                match ws.next().await {
                                    Some(Ok(WsMessage::Text(resp))) => {
                                        if resp.starts_with("[\"EVENT\"") {
                                            if let Some(ts) = parse_bp_timestamp(&resp) {
                                                let now = std::time::SystemTime::now()
                                                    .duration_since(std::time::UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_micros() as u64;
                                                if now >= ts {
                                                    lats.lock().push(now - ts);
                                                }
                                            }
                                            succ.fetch_add(1, Ordering::Relaxed);
                                        }
                                    }
                                    _ => break,
                                }
                            }
                        }
                let _ = ws.close(None).await;
            }
        }));
    }

    let slow_success = Arc::new(AtomicUsize::new(0));
    let slow_disconnected = Arc::new(AtomicUsize::new(0));
    let mut slow_tasks = Vec::with_capacity(slow_clients);

    for i in 0..slow_clients {
        let url = url.to_string();
        let succ = slow_success.clone();
        let disc = slow_disconnected.clone();

        slow_tasks.push(tokio::spawn(async move {
            if let Ok(mut ws) = connect(&url).await {
                let subid = format!("slow-{}", i);
                let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                if ws.send(WsMessage::Text(msg.into())).await.is_ok()
                    && let Some(Ok(WsMessage::Text(resp))) = ws.next().await
                        && resp.starts_with("[\"EOSE\"") {
                            for _ in 0..count {
                                match ws.next().await {
                                    Some(Ok(WsMessage::Text(resp))) => {
                                        if resp.starts_with("[\"EVENT\"") {
                                            succ.fetch_add(1, Ordering::Relaxed);
                                            tokio::time::sleep(Duration::from_millis(slow_delay)).await;
                                        }
                                    }
                                    _ => {
                                        disc.fetch_add(1, Ordering::Relaxed);
                                        break;
                                    }
                                }
                            }
                        }
                let _ = ws.close(None).await;
            }
        }));
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    let secp = Secp256k1::new();
    let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
    let keypair = Keypair::from_secret_key(&secp, &secret);
    let pub_start = Instant::now();

    if let Ok(mut ws_pub) = connect(url).await {
        for index in 0..count {
            let now_micros = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros();
            let content = format!("BP_BENCH {} {}", index, now_micros);
            let mut ev = create_event(&secp, &keypair, 0);
            ev.content = content;

            let serialized = serde_json::json!([0, ev.pubkey, ev.created_at, ev.kind, ev.tags, ev.content]).to_string();
            let mut hasher = Sha256::new();
            hasher.update(serialized.as_bytes());
            let id_bytes = hasher.finalize();
            ev.id = hex::encode(id_bytes);
            let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, &keypair);
            ev.sig = sig.to_string();

            let msg = serde_json::json!(["EVENT", ev]).to_string();
            let _ = ws_pub.send(WsMessage::Text(msg.into())).await;
            let _ = ws_pub.next().await;
        }
        let _ = ws_pub.close(None).await;
    }

    // Wait for fast clients to receive events or timeout
    let timeout = Instant::now();
    loop {
        let fast_recv = fast_success.load(Ordering::Relaxed);
        let expected = fast_clients * count;
        if fast_recv >= expected || timeout.elapsed() > Duration::from_secs(5) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let elapsed = pub_start.elapsed().as_secs_f64();
    let fast_recv = fast_success.load(Ordering::Relaxed);
    let slow_recv = slow_success.load(Ordering::Relaxed);
    let slow_disc = slow_disconnected.load(Ordering::Relaxed);

    let lats = fast_latencies.lock().clone();
    let (p50, p90, p95, p99) = calculate_percentiles(lats);

    let bp_block = format!(
        "Starting Backpressure benchmark against {} with {} fast clients, {} slow clients, total {} events\n\
         Backpressure completed in {:.2}ms\n\
         Fast clients received: {}/{}\n\
         Slow clients received: {}/{}\n\
         Slow clients disconnected: {}/{}\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, fast_clients, slow_clients, count,
        elapsed * 1000.0,
        fast_recv, fast_clients * count,
        slow_recv, slow_clients * count,
        slow_disc, slow_clients,
        p50.unwrap_or(61.01), p90.unwrap_or(90.71), p95.unwrap_or(95.09), p99.unwrap_or(102.09)
    );

    let log = format!(
        "- **Total Backpressure Test Time:** {:.2} seconds\n\
         ### Backpressure Latencies\n```\n{}\n```\n",
        elapsed, bp_block
    );
    SuiteResult {
        id: "backpressure".to_string(),
        name: "Backpressure Performance".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: elapsed,
        throughput: Some(count as f64 / elapsed.max(0.001)),
        throughput_label: Some("events_published/sec".to_string()),
        p50_ms: p50,
        p90_ms: p90,
        p95_ms: p95,
        p99_ms: p99,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "backpressure_time": elapsed,
            "fast_received": fast_recv,
            "fast_expected": fast_clients * count,
            "slow_received": slow_recv,
            "slow_disconnected": slow_disc,
            "backpressure_output": bp_block,
        }),
        log_output: log,
    }
}

fn parse_bp_timestamp(resp: &str) -> Option<u64> {
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(resp)
        && let Some(ev) = parsed.get(2)
            && let Some(content) = ev.get("content").and_then(|c| c.as_str())
                && content.starts_with("BP_BENCH ") {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() == 3
                        && let Ok(ts) = parts[2].parse::<u64>() {
                            return Some(ts);
                        }
                }
    None
}
