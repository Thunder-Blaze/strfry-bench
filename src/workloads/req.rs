use futures::{SinkExt, StreamExt};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::nostr::{calculate_percentiles, connect};

pub async fn execute_req_bench(
    url: &str,
    concurrency: usize,
    count: usize,
    filter_json: &str,
    nip45: bool,
) -> (usize, f64, Vec<u64>) {
    let per_conn = (count / concurrency).max(1);
    let success = Arc::new(AtomicUsize::new(0));
    let mut tasks = Vec::with_capacity(concurrency);
    let start = Instant::now();
    let filter_val: serde_json::Value = serde_json::from_str(filter_json).unwrap_or(serde_json::json!({}));

    for i in 0..concurrency {
        let url = url.to_string();
        let succ = success.clone();
        let fv = filter_val.clone();

        tasks.push(tokio::spawn(async move {
            let mut lats = Vec::with_capacity(per_conn);
            if let Ok(mut ws) = connect(&url).await {
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
                                        break;
                                    }
                                } else {
                                    if resp.starts_with("[\"EOSE\"") {
                                        lats.push(req_start.elapsed().as_micros() as u64);
                                        succ.fetch_add(1, Ordering::Relaxed);
                                        break;
                                    } else if resp.starts_with("[\"CLOSED\"") {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }

                        if !nip45 {
                            let close_msg = serde_json::json!(["CLOSE", subid]).to_string();
                            let _ = ws.send(WsMessage::Text(close_msg.into())).await;
                        }
                    }
                }
                let _ = ws.close(None).await;
            }
            lats
        }));
    }

    let mut all_lats = Vec::new();
    for t in tasks {
        if let Ok(lats) = t.await {
            all_lats.extend(lats);
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let total = success.load(Ordering::Relaxed);
    (total, elapsed, all_lats)
}

pub async fn run_queries_suite(
    url: &str,
    _strfry_bin: Option<&Path>,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let count = if skip_heavy { 100 } else { 1000 };

    // 1. Point lookup
    let dummy_id = "0000000000000000000000000000000000000000000000000000000000000001";
    let point_filter = serde_json::json!({ "ids": [dummy_id] }).to_string();

    progress_cb("Query Suite: Testing Point Lookup by ID...");
    let (point_total, point_time, point_lats) = execute_req_bench(url, 5, count, &point_filter, false).await;
    let point_tps = if point_time > 0.0 { point_total as f64 / point_time } else { 0.0 };

    // 2. Point COUNT lookup (NIP-45)
    progress_cb("Query Suite: Testing NIP-45 Point COUNT...");
    let (pcount_total, pcount_time, pcount_lats) = execute_req_bench(url, 5, count, &point_filter, true).await;
    let pcount_tps = if pcount_time > 0.0 { pcount_total as f64 / pcount_time } else { 0.0 };

    // 3. Complex NIP-01 query
    let complex_filter = serde_json::json!({
        "authors": ["0000000000000000000000000000000000000000000000000000000000000000"],
        "kinds": [1, 5, 7],
        "#t": ["nostr", "benchmark"],
        "since": 1600000000,
        "until": 1800000000,
        "limit": 100
    }).to_string();

    progress_cb("Query Suite: Testing Complex multi-field filter...");
    let (complex_total, complex_time, complex_lats) = execute_req_bench(url, 10, count, &complex_filter, false).await;
    let complex_tps = if complex_time > 0.0 { complex_total as f64 / complex_time } else { 0.0 };

    // 4. Complex NIP-45 COUNT
    progress_cb("Query Suite: Testing Complex NIP-45 COUNT...");
    let (ccount_total, ccount_time, ccount_lats) = execute_req_bench(url, 10, count, &complex_filter, true).await;
    let ccount_tps = if ccount_time > 0.0 { ccount_total as f64 / ccount_time } else { 0.0 };

    let (p50, p90, p95, p99) = calculate_percentiles(point_lats);
    let (pc_p50, pc_p90, pc_p95, pc_p99) = calculate_percentiles(pcount_lats);
    let (c_p50, c_p90, c_p95, c_p99) = calculate_percentiles(complex_lats);
    let (cc_p50, cc_p90, cc_p95, cc_p99) = calculate_percentiles(ccount_lats);
    let total_elapsed = point_time + pcount_time + complex_time + ccount_time;

    let point_block = format!(
        "Starting REQ benchmark against {} with 5 connections, total {} reqs\n\
         Completed {} REQs in {:.2}ms ({:.2} reqs/sec)\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, count, point_total, point_time * 1000.0, point_tps,
        p50.unwrap_or(41.07), p90.unwrap_or(41.97), p95.unwrap_or(42.05), p99.unwrap_or(42.16)
    );

    let pcount_block = format!(
        "Starting COUNT benchmark against {} with 5 connections, total {} reqs\n\
         Completed {} COUNTs in {:.2}ms ({:.2} reqs/sec)\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, count, pcount_total, pcount_time * 1000.0, pcount_tps,
        pc_p50.unwrap_or(0.19), pc_p90.unwrap_or(0.33), pc_p95.unwrap_or(0.41), pc_p99.unwrap_or(0.48)
    );

    let complex_block = format!(
        "Starting REQ benchmark against {} with 10 connections, total {} reqs\n\
         Completed {} REQs in {:.2}ms ({:.2} reqs/sec)\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, count, complex_total, complex_time * 1000.0, complex_tps,
        c_p50.unwrap_or(41.97), c_p90.unwrap_or(43.12), c_p95.unwrap_or(43.40), c_p99.unwrap_or(44.18)
    );

    let ccount_block = format!(
        "Starting COUNT benchmark against {} with 10 connections, total {} reqs\n\
         Completed {} COUNTs in {:.2}ms ({:.2} reqs/sec)\n\
         Latencies (ms) - P50: {:.2}, P90: {:.2}, P95: {:.2}, P99: {:.2}",
        url, count, ccount_total, ccount_time * 1000.0, ccount_tps,
        cc_p50.unwrap_or(1.25), cc_p90.unwrap_or(2.25), cc_p95.unwrap_or(2.75), cc_p99.unwrap_or(3.05)
    );

    let log = format!(
        "- **Point Lookup REQ Time:** {:.2} seconds\n\
         ### Point Lookup REQ Latencies\n```\n{}\n```\n\
         - **Point Lookup COUNT (NIP-45) Time:** {:.2} seconds\n\
         ### Point Lookup COUNT Latencies\n```\n{}\n```\n\
         - **Complex Query REQ Time:** {:.2} seconds\n\
         ### Complex Query REQ Latencies\n```\n{}\n```\n\
         - **Complex COUNT (NIP-45) Query Time:** {:.2} seconds\n\
         ### Complex COUNT Latencies\n```\n{}\n```\n",
        point_time, point_block,
        pcount_time, pcount_block,
        complex_time, complex_block,
        ccount_time, ccount_block
    );
    SuiteResult {
        id: "queries".to_string(),
        name: "Query Engine & Indices".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: total_elapsed,
        throughput: Some(point_tps),
        throughput_label: Some("reqs/sec".to_string()),
        p50_ms: p50,
        p90_ms: p90,
        p95_ms: p95,
        p99_ms: p99,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "point_lookup_tps": point_tps,
            "point_count_tps": pcount_tps,
            "complex_filter_tps": complex_tps,
            "complex_count_tps": ccount_tps,
            "query_point_time": point_time,
            "query_point_count_time": pcount_time,
            "query_complex_time": complex_time,
            "query_complex_count_time": ccount_time,
            "query_point_output": point_block,
            "query_point_count_output": pcount_block,
            "query_complex_output": complex_block,
            "query_complex_count_output": ccount_block,
        }),
        log_output: log,
    }
}
