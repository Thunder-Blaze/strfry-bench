use futures::{SinkExt, StreamExt};
use secp256k1::{Keypair, Secp256k1};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::workloads::nostr::{connect, create_event};

pub async fn execute_churn_bench(
    url: &str,
    concurrency: usize,
    count: usize,
) -> (usize, f64, Vec<u64>, Vec<u64>) {
    let per_conn = (count / concurrency).max(1);
    let success = Arc::new(AtomicUsize::new(0));
    let mut tasks = Vec::with_capacity(concurrency);
    let start = Instant::now();

    let secp = Secp256k1::new();
    let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
    let keypair = Keypair::from_secret_key(&secp, &secret);
    let key_secret = keypair.secret_key();

    for _ in 0..concurrency {
        let url = url.to_string();
        let succ = success.clone();
        let ks = key_secret;

        tasks.push(tokio::spawn(async move {
            let mut lats = Vec::with_capacity(per_conn);
            let mut close_lats = Vec::with_capacity(per_conn);
            let secp = Secp256k1::new();
            let kp = Keypair::from_secret_key(&secp, &ks);

            for _ in 0..per_conn {
                let req_start = Instant::now();
                if let Ok(mut ws) = connect(&url).await {
                    let ev = create_event(&secp, &kp, 0);
                    let msg = serde_json::json!(["EVENT", ev]).to_string();
                    let _ = ws.send(WsMessage::Text(msg.into())).await;
                    let _ = ws.next().await; // wait for OK

                    let close_start = Instant::now();
                    let _ = ws.close(None).await;
                    close_lats.push(close_start.elapsed().as_micros() as u64);
                    lats.push(req_start.elapsed().as_micros() as u64);
                    succ.fetch_add(1, Ordering::Relaxed);
                }
            }
            (lats, close_lats)
        }));
    }

    let mut all_lats = Vec::new();
    let mut all_close_lats = Vec::new();
    for t in tasks {
        if let Ok((lats, closes)) = t.await {
            all_lats.extend(lats);
            all_close_lats.extend(closes);
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total = success.load(Ordering::Relaxed);
    (total, elapsed, all_lats, all_close_lats)
}
