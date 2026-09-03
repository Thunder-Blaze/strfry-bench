use std::time::Instant;
use crate::config::{SuiteResult, SuiteStatus};
use crate::workloads::event::execute_event_bench;
use crate::workloads::nostr::calculate_percentiles;
use crate::workloads::req::execute_req_bench;

pub async fn run(
    url: &str,
    skip_heavy: bool,
    progress_cb: &(dyn Fn(&str) + Send + Sync),
) -> SuiteResult {
    let write_count = if skip_heavy { 2000 } else { 20000 };
    let req_count = if skip_heavy { 200 } else { 1000 };

    progress_cb(&format!(
        "Concurrency Suite: Spawning background writers ({} events) while querying ({} REQs)...",
        write_count, req_count
    ));

    let start = Instant::now();

    // Spawn background writer task
    let url_clone = url.to_string();
    let writer_task = tokio::spawn(async move {
        execute_event_bench(&url_clone, 10, write_count, 100).await
    });

    // Small delay to allow write pressure to mount
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    // Run REQ queries under write pressure
    let (req_total, req_time, req_lats) = execute_req_bench(url, 10, req_count, "{\"limit\":10}", false).await;
    let req_tps = if req_time > 0.0 { req_total as f64 / req_time } else { 0.0 };

    // Await background writer completion
    let (write_total, write_time, _) = writer_task.await.unwrap_or((0, 0.0, vec![]));
    let write_tps = if write_time > 0.0 { write_total as f64 / write_time } else { 0.0 };

    let total_elapsed = start.elapsed().as_secs_f64();
    let (p50, p90, p95, p99) = calculate_percentiles(req_lats);

    let log = format!(
        "- **Mixed Read-Write REQ Time:** {:.2} seconds\n\n\
         ### Concurrency Workload Details\n\
         Background writers: {} events in {:.2}s ({:.1} eps)\n\
         Concurrent queries: {} REQs in {:.2}s ({:.1} req/s)\n\
         Query Latency under contention - P50: {:?}ms, P99: {:?}ms",
        req_time,
        write_total, write_time, write_tps,
        req_total, req_time, req_tps,
        p50, p99
    );

    SuiteResult {
        id: "concurrency".to_string(),
        name: "Concurrency & Thread Pool".to_string(),
        status: SuiteStatus::Completed,
        elapsed_secs: total_elapsed,
        throughput: Some(req_tps),
        throughput_label: Some("reqs_under_contention/sec".to_string()),
        p50_ms: p50,
        p90_ms: p90,
        p95_ms: p95,
        p99_ms: p99,
        memory_rss_mb: None,
        metrics: serde_json::json!({
            "mixed_req_time": req_time,
            "req_tps_under_contention": req_tps,
            "write_tps_contention": write_tps,
            "total_reqs": req_total,
            "total_events_written": write_total,
        }),
        log_output: log,
    }
}
