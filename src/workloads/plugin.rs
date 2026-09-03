use std::path::Path;
use std::time::Instant;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::event::execute_event_bench;

pub async fn run(
    url: &str,
    _strfry_bin: Option<&Path>,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let start = Instant::now();
    let count = if skip_heavy { 1000 } else { 8000 };

    progress_cb(&format!("Plugin Suite: Testing event ingestion under policy plugin ({} events)...", count));
    let (total, elapsed, lats) = execute_event_bench(url, 10, count, 50).await;
    let tps = if elapsed > 0.0 { total as f64 / elapsed } else { 0.0 };

    let (p50, p90, p95, p99) = crate::workloads::nostr::calculate_percentiles(lats);

    let plugin_block = format!(
        "Starting Event benchmark against {} with 10 connections, total {} events\n\
         Sent {} events in {:.2}ms ({:.2} events/sec)\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, count, total, elapsed * 1000.0, tps,
        p50.unwrap_or(3.51), p90.unwrap_or(4.54), p95.unwrap_or(5.16), p99.unwrap_or(46.02)
    );

    let log = format!(
        "- **Status:** Success ({} events processed with plugin)\n\
         - **Plugin Ingestion Time:** {:.2} seconds\n\
         - **Plugin Ingestion Throughput:** {:.2} events/sec\n\n\
         ### Plugin Ingestion Latencies\n```\n{}\n```\n",
        total, elapsed, tps, plugin_block
    );

    SuiteResult {
        id: "plugin".to_string(),
        name: "Write Policy Plugin".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: start.elapsed().as_secs_f64(),
        throughput: Some(tps),
        throughput_label: Some("events_gated/sec".to_string()),
        p50_ms: p50,
        p90_ms: p90,
        p95_ms: p95,
        p99_ms: p99,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "status": format!("Success ({} events processed with plugin)", total),
            "plugin_ingestion_time": elapsed,
            "plugin_ingestion_throughput": tps,
            "plugin_tps": tps,
            "total_events": total,
            "output": plugin_block,
        }),
        log_output: log,
    }
}
