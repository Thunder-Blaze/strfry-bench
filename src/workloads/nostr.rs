use secp256k1::{Keypair, Secp256k1};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u16,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

pub fn create_event(secp: &Secp256k1<secp256k1::All>, keypair: &Keypair, payload_size: usize) -> Event {
    let mut content = format!("Bench event {}", rand::random::<u64>());
    if payload_size > content.len() {
        content.push_str(&"a".repeat(payload_size - content.len()));
    }
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

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
    ])
    .to_string();

    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    let id_bytes = hasher.finalize();
    event.id = hex::encode(id_bytes);

    let sig = secp.sign_schnorr_no_aux_rand(&id_bytes, keypair);
    event.sig = sig.to_string();

    event
}

pub async fn connect(url: &str) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn std::error::Error + Send + Sync>> {
    let (ws_stream, _) = connect_async(url).await?;
    Ok(ws_stream)
}

pub fn calculate_percentiles(mut lats_micros: Vec<u64>) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
    if lats_micros.is_empty() {
        return (None, None, None, None);
    }
    lats_micros.sort_unstable();
    let len = lats_micros.len() as f64;
    let p50 = lats_micros[(len * 0.50) as usize] as f64 / 1000.0;
    let p90 = lats_micros[(len * 0.90) as usize] as f64 / 1000.0;
    let p95 = lats_micros[(len * 0.95) as usize] as f64 / 1000.0;
    let p99 = lats_micros[(len * 0.99) as usize] as f64 / 1000.0;
    (Some(p50), Some(p90), Some(p95), Some(p99))
}
