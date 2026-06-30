use clap::{Parser, Subcommand};
use futures::{SinkExt, StreamExt};
use secp256k1::{Keypair, Secp256k1};
use rand::{SeedableRng, RngExt};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage, MaybeTlsStream, WebSocketStream};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Event {
        url: String,
        #[arg(short, long, default_value_t = 10)]
        concurrency: usize,
        #[arg(short = 'n', long, default_value_t = 10000)]
        count: usize,
        #[arg(short, long, default_value_t = 0)]
        payload_size: usize,
    },
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
    Paginate {
        url: String,
        #[arg(short, long, default_value_t = 10)]
        depth: usize,
        #[arg(short, long, default_value_t = 1)]
        concurrency: usize,
    },
    Monitor {
        url: String,
        #[arg(short, long, default_value_t = 100)]
        subs: usize,
        #[arg(short, long, default_value_t = 100)]
        publish: usize,
    },
    Connections {
        url: String,
        #[arg(short, long, default_value_t = 1000)]
        count: usize,
    },
    Churn {
        url: String,
        #[arg(short, long, default_value_t = 100)]
        concurrency: usize,
        #[arg(short = 'n', long, default_value_t = 1000)]
        count: usize,
    },
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
    }
}

#[derive(Serialize, Deserialize)]
struct Event {
    id: String,
    pubkey: String,
    created_at: u64,
    kind: u16,
    tags: Vec<Vec<String>>,
    content: String,
    sig: String,
}

fn create_event(secp: &Secp256k1<secp256k1::All>, keypair: &Keypair, payload_size: usize) -> Event {
    let mut content = format!("Bench event {}", rand::random::<u64>());
    if payload_size > content.len() {
        content.push_str(&"a".repeat(payload_size - content.len()));
    }
    let created_at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    
    let pubkey = keypair.public_key().x_only_public_key().0.to_string();
    
    let mut event = Event {
        id: String::new(),
        pubkey,
        created_at,
        kind: 1,
        tags: vec![],
        content,
        sig: String::new(),
    };
    
    let serialized = serde_json::json!([
        0,
        event.pubkey,
        event.created_at,
        event.kind,
        event.tags,
        event.content
    ]).to_string();
    
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let id_bytes = hasher.finalize();
    event.id = hex::encode(id_bytes);
    
    let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, &keypair);
    event.sig = sig.to_string();
    
    event
}

async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn std::error::Error + Send + Sync>> {
    let (ws_stream, _) = connect_async(url).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    Ok(ws_stream)
}

async fn connect_or_panic(url: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    match connect(url).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error: Failed to connect to relay at {}: {}", url, e);
            std::process::exit(1);
        }
    }
}

fn print_percentiles(mut lats: Vec<u64>) {
    if lats.is_empty() { return; }
    lats.sort_unstable();
    let p50 = lats[(lats.len() as f64 * 0.50) as usize];
    let p90 = lats[(lats.len() as f64 * 0.90) as usize];
    let p95 = lats[(lats.len() as f64 * 0.95) as usize];
    let p99 = lats[(lats.len() as f64 * 0.99) as usize];
    println!("Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}", 
        p50 as f64 / 1000.0, p90 as f64 / 1000.0, p95 as f64 / 1000.0, p99 as f64 / 1000.0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let secp = Secp256k1::new();
    let secret = secp256k1::SecretKey::new(&mut secp256k1::rand::rng());
    let keypair = Keypair::from_secret_key(&secp, &secret);

    match cli.command {
        Commands::Event { url, concurrency, count, payload_size } => {
            println!("Starting Event benchmark against {} with {} connections, total {} events", url, concurrency, count);
            let per_conn = count / concurrency;
            let mut tasks = vec![];
            let start = Instant::now();
            let success = Arc::new(AtomicUsize::new(0));
            
            for _ in 0..concurrency {
                let url = url.clone();
                let keypair = keypair.secret_key();
                let succ = success.clone();
                tasks.push(tokio::spawn(async move {
                    let mut lats = Vec::with_capacity(per_conn);
                    let secp = Secp256k1::new();
                    let kp = Keypair::from_secret_key(&secp, &keypair);
                    let mut ws = connect_or_panic(&url).await;
                    for _ in 0..per_conn {
                        let ev = create_event(&secp, &kp, payload_size);
                        let msg = serde_json::json!(["EVENT", ev]).to_string();
                        let req_start = Instant::now();
                        if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                            if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                if resp.contains("\"OK\"") {
                                    lats.push(req_start.elapsed().as_micros() as u64);
                                    succ.fetch_add(1, Ordering::Relaxed);
                                }
                            }
                        }
                    }
                    lats
                }));
            }
            
            let mut all_lats = vec![];
            for t in tasks {
                all_lats.extend(t.await.unwrap());
            }
            
            let elapsed = start.elapsed();
            let total = success.load(Ordering::Relaxed);
            println!("Sent {} events in {:.2?} ({:.2} events/sec)", total, elapsed, total as f64 / elapsed.as_secs_f64());
            print_percentiles(all_lats);
        }
        Commands::Req { url, concurrency, count, filter, nip45 } => {
            let label = if nip45 { "COUNT" } else { "REQ" };
            println!("Starting {} benchmark against {} with {} connections, total {} reqs", label, url, concurrency, count);
            let per_conn = count / concurrency;
            let mut tasks = vec![];
            let start = Instant::now();
            let success = Arc::new(AtomicUsize::new(0));
            let filter_val: serde_json::Value = serde_json::from_str(&filter).unwrap();

            for i in 0..concurrency {
                let url = url.clone();
                let succ = success.clone();
                let fv = filter_val.clone();
                tasks.push(tokio::spawn(async move {
                    let mut lats = Vec::with_capacity(per_conn);
                    let mut ws = connect_or_panic(&url).await;
                    for j in 0..per_conn {
                        let subid = format!("sub-{}-{}", i, j);
                        let msg = if nip45 {
                            serde_json::json!(["COUNT", subid, fv]).to_string()
                        } else {
                            serde_json::json!(["REQ", subid, fv]).to_string()
                        };
                        let req_start = Instant::now();
                        if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                            loop {
                                if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                    if nip45 {
                                        if resp.starts_with("[\"COUNT\"") {
                                            lats.push(req_start.elapsed().as_micros() as u64);
                                            succ.fetch_add(1, Ordering::Relaxed);
                                            break;
                                        } else if resp.starts_with("[\"CLOSED\"") {
                                            if resp.contains("unrecognised filter item: search") || resp.contains("search not supported") {
                                                println!("SEARCH_UNSUPPORTED");
                                            }
                                            break;
                                        }
                                    } else {
                                        if resp.starts_with("[\"EOSE\"") {
                                            lats.push(req_start.elapsed().as_micros() as u64);
                                            succ.fetch_add(1, Ordering::Relaxed);
                                            break;
                                        } else if resp.starts_with("[\"CLOSED\"") {
                                            if resp.contains("unrecognised filter item: search") || resp.contains("search not supported") {
                                                println!("SEARCH_UNSUPPORTED");
                                            }
                                            break;
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            if !nip45 {
                                let msg = serde_json::json!(["CLOSE", subid]).to_string();
                                let _ = ws.send(WsMessage::Text(msg.into())).await;
                            }
                        }
                    }
                    lats
                }));
            }
            
            let mut all_lats = vec![];
            for t in tasks {
                all_lats.extend(t.await.unwrap());
            }
            
            let elapsed = start.elapsed();
            let total = success.load(Ordering::Relaxed);
            println!("Completed {} {}s in {:.2?} ({:.2} reqs/sec)", total, label, elapsed, total as f64 / elapsed.as_secs_f64());
            print_percentiles(all_lats);
        }
        Commands::Paginate { url, depth, concurrency } => {
            println!("Starting Paginate benchmark against {} with depth {}", url, depth);
            let start = Instant::now();
            let mut tasks = vec![];
            let url = url.clone();
            
            for _ in 0..concurrency {
                let url = url.clone();
                tasks.push(tokio::spawn(async move {
                    let mut ws = connect_or_panic(&url).await;
                    let mut until: Option<u64> = None;
                    for i in 0..depth {
                        let subid = format!("pag-{}", i);
                        let mut fv = serde_json::json!({"limit": 100});
                        if let Some(u) = until {
                            fv.as_object_mut().unwrap().insert("until".to_string(), serde_json::json!(u));
                        }
                        let msg = serde_json::json!(["REQ", subid, fv]).to_string();
                        let req_start = Instant::now();
                        if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                            let mut last_created_at = None;
                            loop {
                                if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                    if resp.starts_with("[\"EVENT\"") {
                                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&resp) {
                                            if let Some(ev) = parsed.get(2) {
                                                if let Some(ca) = ev.get("created_at").and_then(|c| c.as_u64()) {
                                                    last_created_at = Some(ca);
                                                }
                                            }
                                        }
                                    } else if resp.starts_with("[\"EOSE\"") {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }
                            println!("Depth {} (until {:?}): {:.2?}", i, until, req_start.elapsed());
                            let msg = serde_json::json!(["CLOSE", subid]).to_string();
                            let _ = ws.send(WsMessage::Text(msg.into())).await;
                            until = last_created_at.map(|c| c - 1);
                            if until.is_none() {
                                break;
                            }
                        }
                    }
                }));
            }
            
            for t in tasks {
                t.await.unwrap();
            }
            println!("Paginate complete in {:.2?}", start.elapsed());
        }
        Commands::Monitor { url, subs, publish } => {
            println!("Starting Monitor benchmark: {} subscriptions, {} events published", subs, publish);
            
            let (tx, mut rx) = tokio::sync::mpsc::channel(subs * 2);
            let publish_start_time = Arc::new(std::sync::RwLock::new(Instant::now()));
            let active_subs = Arc::new(AtomicUsize::new(0));
            let mut sub_tasks = vec![];

            let sub_start = Instant::now();
            for i in 0..subs {
                let url = url.clone();
                let tx = tx.clone();
                let publish_start_time = publish_start_time.clone();
                let active_subs = active_subs.clone();
                sub_tasks.push(tokio::spawn(async move {
                    let mut ws = connect_or_panic(&url).await;
                        let subid = format!("sub-{}", i);
                        let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                        if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                            if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                if resp.starts_with("[\"EOSE\"") {
                                    active_subs.fetch_add(1, Ordering::Relaxed);
                                    for _ in 0..publish {
                                        loop {
                                            match ws.next().await {
                                                Some(Ok(WsMessage::Text(resp))) => {
                                                    if resp.starts_with("[\"EVENT\"") {
                                                        let elapsed = *publish_start_time.read().unwrap();
                                                        let _ = tx.send(elapsed.elapsed()).await;
                                                        break;
                                                    }
                                                }
                                                _ => return, // Connection closed/error
                                            }
                                        }
                                    }
                                }
                            }
                        }
                }));
            }

            // Wait a moment to let all connections start connecting and subscribing
            tokio::time::sleep(Duration::from_secs(2)).await;
            let active_count = active_subs.load(Ordering::Relaxed);
            println!("Opened and confirmed {} of {} subscriptions in {:.2?}", active_count, subs, sub_start.elapsed());
            
            if active_count == 0 {
                println!("Error: No active subscriptions established.");
                return Ok(());
            }

            let mut ws_pub = connect_or_panic(&url).await;
            let mut first_lats = Vec::with_capacity(publish);
            let mut last_lats = Vec::with_capacity(publish);

            let pub_start = Instant::now();
            for _ in 0..publish {
                let ev = create_event(&secp, &keypair, 0);
                let msg = serde_json::json!(["EVENT", ev]).to_string();
                
                // Update publish start time
                *publish_start_time.write().unwrap() = Instant::now();
                
                ws_pub.send(WsMessage::Text(msg.into())).await.unwrap();
                if let Some(Ok(WsMessage::Text(_resp))) = ws_pub.next().await {
                    // ignore OK
                }
                
                let mut lats = Vec::with_capacity(active_count);
                for _ in 0..active_count {
                    if let Some(elapsed) = rx.recv().await {
                        lats.push(elapsed.as_micros() as u64);
                    }
                }
                
                if !lats.is_empty() {
                    lats.sort_unstable();
                    first_lats.push(lats[0]);
                    last_lats.push(lats[lats.len() - 1]);
                }
            }

            println!("Published and matched {} events across {} subs in {:.2?}", publish, active_count, pub_start.elapsed());
            println!("Time-to-First-Client:");
            print_percentiles(first_lats);
            println!("Time-to-Last-Client:");
            print_percentiles(last_lats);
        }
        Commands::Connections { url, count } => {
            println!("Starting Connection storm benchmark: {} connections", count);
            let mut tasks = vec![];
            let start = Instant::now();
            let success = Arc::new(AtomicUsize::new(0));
            
            for _ in 0..count {
                let url = url.clone();
                let succ = success.clone();
                tasks.push(tokio::spawn(async move {
                    let req_start = Instant::now();
                    if let Ok(_ws) = connect(&url).await {
                        succ.fetch_add(1, Ordering::Relaxed);
                        // Hold open momentarily to simulate concurrent load
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        Some(req_start.elapsed().as_micros() as u64)
                    } else {
                        None
                    }
                }));
            }
            
            let mut all_lats = vec![];
            for t in tasks {
                if let Ok(Some(lat)) = t.await {
                    all_lats.push(lat);
                }
            }
            
            let elapsed = start.elapsed();
            let total = success.load(Ordering::Relaxed);
            println!("Successfully established {} connections in {:.2?} ({:.2} conn/sec)", total, elapsed, total as f64 / elapsed.as_secs_f64());
            print_percentiles(all_lats);
        }
        Commands::Churn { url, concurrency, count } => {
            println!("Starting Churn benchmark: {} total connections, {} concurrent", count, concurrency);
            let start = Instant::now();
            let success = Arc::new(AtomicUsize::new(0));
            let per_conn = count / concurrency;
            let mut tasks = vec![];
            
            let secp = Secp256k1::new();
            let keypair = Keypair::new(&secp, &mut secp256k1::rand::rng());
            let keypair_secret = keypair.secret_key();
            
            for _ in 0..concurrency {
                let url = url.clone();
                let succ = success.clone();
                let keypair_secret = keypair_secret.clone();
                tasks.push(tokio::spawn(async move {
                    let mut lats = Vec::with_capacity(per_conn);
                    let mut close_lats = Vec::with_capacity(per_conn);
                    let secp = Secp256k1::new();
                    let kp = Keypair::from_secret_key(&secp, &keypair_secret);
                    for _ in 0..per_conn {
                        let req_start = Instant::now();
                        let mut ws = connect_or_panic(&url).await;
                            let ev = create_event(&secp, &kp, 0);
                            let msg = serde_json::json!(["EVENT", ev]).to_string();
                            if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                                if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                    if resp.contains("\"OK\"") {
                                        // Success
                                    }
                                }
                            }
                            let close_start = Instant::now();
                            let _ = ws.close(None).await;
                            close_lats.push(close_start.elapsed().as_micros() as u64);
                            lats.push(req_start.elapsed().as_micros() as u64);
                            succ.fetch_add(1, Ordering::Relaxed);
                    }
                    (lats, close_lats)
                }));
            }
            
            let mut all_lats = vec![];
            let mut all_close_lats = vec![];
            for t in tasks {
                let (lats, close_lats) = t.await.unwrap();
                all_lats.extend(lats);
                all_close_lats.extend(close_lats);
            }
            let elapsed = start.elapsed();
            let total = success.load(Ordering::Relaxed);
            println!("Successfully churned {} connections in {:.2?} ({:.2} conn/sec)", total, elapsed, total as f64 / elapsed.as_secs_f64());
            println!("Total Connection Churn Latency:");
            print_percentiles(all_lats);
            println!("Socket Close Handshake Latency:");
            print_percentiles(all_close_lats);
        }
        Commands::Backpressure { url, fast_clients, slow_clients, count, slow_delay } => {
            println!("Starting Backpressure benchmark against {} with {} fast clients, {} slow clients, total {} events", url, fast_clients, slow_clients, count);
            
            let fast_success = Arc::new(AtomicUsize::new(0));
            let fast_latencies = Arc::new(tokio::sync::Mutex::new(Vec::new()));
            let mut fast_tasks = vec![];
            
            for i in 0..fast_clients {
                let url = url.clone();
                let succ = fast_success.clone();
                let lats = fast_latencies.clone();
                let count_val = count;
                fast_tasks.push(tokio::spawn(async move {
                    let mut ws = connect_or_panic(&url).await;
                        let subid = format!("fast-{}", i);
                        let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                        if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                            if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                                if resp.starts_with("[\"EOSE\"") {
                                    for _ in 0..count_val {
                                        match ws.next().await {
                                            Some(Ok(WsMessage::Text(resp))) => {
                                                if resp.starts_with("[\"EVENT\"") {
                                                    if let Some(ts) = parse_bp_timestamp(&resp) {
                                                        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as u64;
                                                        if now >= ts {
                                                            lats.lock().await.push(now - ts);
                                                        }
                                                    }
                                                    succ.fetch_add(1, Ordering::Relaxed);
                                                }
                                            }
                                            _ => break,
                                        }
                                    }
                                }
                            }
                        }
                }));
            }

            let slow_success = Arc::new(AtomicUsize::new(0));
            let slow_disconnected = Arc::new(AtomicUsize::new(0));
            let mut slow_tasks = vec![];
            
            for i in 0..slow_clients {
                let url = url.clone();
                let succ = slow_success.clone();
                let disc = slow_disconnected.clone();
                let delay = slow_delay;
                let count_val = count;
                slow_tasks.push(tokio::spawn(async move {
                    let mut ws = connect_or_panic(&url).await;
                    let subid = format!("slow-{}", i);
                    let msg = serde_json::json!(["REQ", subid, {"kinds": [1]}]).to_string();
                    if ws.send(WsMessage::Text(msg.into())).await.is_ok() {
                        if let Some(Ok(WsMessage::Text(resp))) = ws.next().await {
                            if resp.starts_with("[\"EOSE\"") {
                                for _ in 0..count_val {
                                    match ws.next().await {
                                        Some(Ok(WsMessage::Text(resp))) => {
                                            if resp.starts_with("[\"EVENT\"") {
                                                succ.fetch_add(1, Ordering::Relaxed);
                                                tokio::time::sleep(Duration::from_millis(delay)).await;
                                            }
                                        }
                                        _ => {
                                            disc.fetch_add(1, Ordering::Relaxed);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }));
            }

            // Wait for clients to connect and subscribe
            tokio::time::sleep(Duration::from_secs(2)).await;

            let mut ws_pub = connect_or_panic(&url).await;
            let pub_start = Instant::now();
            
            for index in 0..count {
                let now_micros = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
                let content = format!("BP_BENCH {} {}", index, now_micros);
                let mut ev = create_event(&secp, &keypair, 0);
                ev.content = content;
                
                let serialized = serde_json::json!([
                    0,
                    ev.pubkey,
                    ev.created_at,
                    ev.kind,
                    ev.tags,
                    ev.content
                ]).to_string();
                let mut hasher = Sha256::new();
                hasher.update(serialized.as_bytes());
                let id_bytes = hasher.finalize();
                ev.id = hex::encode(id_bytes);
                let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, &keypair);
                ev.sig = sig.to_string();

                let msg = serde_json::json!(["EVENT", ev]).to_string();
                let _ = ws_pub.send(WsMessage::Text(msg.into())).await;
                let _ = ws_pub.next().await; // wait for OK
            }
            
            // Wait for fast clients to receive all events or timeout
            let test_timeout = Instant::now();
            loop {
                let fast_recv = fast_success.load(Ordering::Relaxed);
                let expected = fast_clients * count;
                if fast_recv >= expected || test_timeout.elapsed() > Duration::from_secs(10) {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            let elapsed = pub_start.elapsed();
            let fast_recv = fast_success.load(Ordering::Relaxed);
            let slow_recv = slow_success.load(Ordering::Relaxed);
            let slow_disc = slow_disconnected.load(Ordering::Relaxed);
            
            println!("Backpressure completed in {:.2?}", elapsed);
            println!("Fast clients received: {}/{}", fast_recv, fast_clients * count);
            println!("Slow clients received: {}/{}", slow_recv, slow_clients * count);
            println!("Slow clients disconnected: {}/{}", slow_disc, slow_clients);

            let lats = fast_latencies.lock().await.clone();
            print_percentiles(lats);
        }
        Commands::Generate { count, authors, seed, payload_size: _ } => {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            
            let mut keypairs = Vec::with_capacity(authors);
            for _ in 0..authors {
                let mut secret_bytes = [0u8; 32];
                rng.fill(&mut secret_bytes);
                while secp256k1::SecretKey::from_byte_array(secret_bytes).is_err() {
                    rng.fill(&mut secret_bytes);
                }
                let secret = secp256k1::SecretKey::from_byte_array(secret_bytes).unwrap();
                let kp = Keypair::from_secret_key(&secp, &secret);
                keypairs.push(kp);
            }
            
            let kinds = [0, 1, 1, 1, 1, 3, 7, 30023];
            
            for _ in 0..count {
                let kp = &keypairs[rng.random_range(0..authors)];
                let kind = kinds[rng.random_range(0..kinds.len())];
                let content = format!("Deterministic bench event {}", rng.random::<u64>());
                let created_at = rng.random_range(1600000000..1800000000);
                
                let pubkey = kp.public_key().x_only_public_key().0.to_string();
                
                let mut event = Event {
                    id: String::new(),
                    pubkey,
                    created_at,
                    kind,
                    tags: vec![],
                    content,
                    sig: String::new(),
                };
                
                let serialized = serde_json::json!([
                    0,
                    event.pubkey,
                    event.created_at,
                    event.kind,
                    event.tags,
                    event.content
                ]).to_string();
                
                let mut hasher = Sha256::new();
                hasher.update(serialized.as_bytes());
                let id_bytes = hasher.finalize();
                event.id = hex::encode(id_bytes);
                
                let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, &kp);
                event.sig = sig.to_string();
                
                let json = serde_json::to_string(&event).unwrap();
                println!("{}", json);
            }
        }
        Commands::Malicious { url, count, slow_loris, sig_flood, duration } => {
            println!("Starting Malicious client simulation against {} for {} seconds", url, duration);
            
            for _ in 0..count {
                let url = url.clone();
                let keypair_secret = keypair.secret_key();
                tokio::spawn(async move {
                    let mut ws = connect_or_panic(&url).await;
                        if slow_loris {
                            // Just hold the connection open to tie up resources
                            tokio::time::sleep(Duration::from_secs(3600 * 24)).await;
                        } else if sig_flood {
                            let secp = Secp256k1::new();
                            let kp = Keypair::from_secret_key(&secp, &keypair_secret);
                            loop {
                                let mut ev = create_event(&secp, &kp, 0);
                                // Corrupt the signature by supplying an invalid dummy 64-byte hex string
                                ev.sig = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string();
                                
                                let msg = serde_json::json!(["EVENT", ev]).to_string();
                                if ws.send(WsMessage::Text(msg.into())).await.is_err() {
                                    break;
                                }
                                // Yield so we don't completely lock the tokio thread, but still hammer the relay
                                tokio::task::yield_now().await;
                            }
                        }
                });
            }
            
            tokio::time::sleep(Duration::from_secs(duration)).await;
            println!("Malicious simulation complete.");
        }
    }

    Ok(())
}

fn parse_bp_timestamp(resp: &str) -> Option<u64> {
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(resp) {
        if let Some(ev) = parsed.get(2) {
            if let Some(content) = ev.get("content").and_then(|c| c.as_str()) {
                if content.starts_with("BP_BENCH ") {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() == 3 {
                        if let Ok(ts) = parts[2].parse::<u64>() {
                            return Some(ts);
                        }
                    }
                }
            }
        }
    }
    None
}

