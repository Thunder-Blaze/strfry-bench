use crate::config::{DeltaStatus, MetricDelta, RunReport};

pub fn compute_comparison_deltas(base: &RunReport, target: &RunReport) -> Vec<MetricDelta> {
    let mut deltas = Vec::new();

    // 1. Deterministic metrics
    if let (Some(b_det), Some(t_det)) = (&base.deterministic, &target.deterministic) {
        add_metric_delta(
            &mut deltas,
            "Instructions Retired",
            b_det.instructions as f64,
            t_det.instructions as f64,
            "instr",
            true, // lower is better
        );
        add_metric_delta(
            &mut deltas,
            "Heap Allocations Count",
            b_det.allocations as f64,
            t_det.allocations as f64,
            "allocs",
            true,
        );
        add_metric_delta(
            &mut deltas,
            "Total Bytes Allocated",
            b_det.allocated_bytes as f64,
            t_det.allocated_bytes as f64,
            "bytes",
            true,
        );
    }

    // 2. Suite metrics
    for t_suite in &target.suites {
        if let Some(b_suite) = base.suites.iter().find(|s| s.id == t_suite.id) {
            // Throughput
            if let (Some(b_tps), Some(t_tps)) = (b_suite.throughput, t_suite.throughput) {
                let unit = t_suite.throughput_label.as_deref().unwrap_or("ops/s");
                add_metric_delta(
                    &mut deltas,
                    &format!("{}: Throughput", t_suite.name),
                    b_tps,
                    t_tps,
                    unit,
                    false, // higher is better
                );
            }

            // Latency P50
            if let (Some(b_p50), Some(t_p50)) = (b_suite.p50_ms, t_suite.p50_ms) {
                add_metric_delta(
                    &mut deltas,
                    &format!("{}: P50 Latency", t_suite.name),
                    b_p50,
                    t_p50,
                    "ms",
                    true,
                );
            }

            // Latency P99
            if let (Some(b_p99), Some(t_p99)) = (b_suite.p99_ms, t_suite.p99_ms) {
                add_metric_delta(
                    &mut deltas,
                    &format!("{}: P99 Latency", t_suite.name),
                    b_p99,
                    t_p99,
                    "ms",
                    true,
                );
            }
        }
    }

    deltas
}

fn add_metric_delta(
    deltas: &mut Vec<MetricDelta>,
    metric: &str,
    base_val: f64,
    target_val: f64,
    unit: &str,
    lower_better: bool,
) {
    if base_val <= 0.0 && target_val <= 0.0 {
        return;
    }

    let delta_pct = if base_val > 0.0 {
        ((target_val - base_val) / base_val) * 100.0
    } else {
        0.0
    };

    let status = if delta_pct.abs() < 1.0 {
        DeltaStatus::Stable
    } else if target_val < base_val {
        if lower_better {
            DeltaStatus::Improved
        } else {
            DeltaStatus::Regressed
        }
    } else {
        if lower_better {
            DeltaStatus::Regressed
        } else {
            DeltaStatus::Improved
        }
    };

    deltas.push(MetricDelta {
        metric: metric.to_string(),
        base_value: base_val,
        target_value: target_val,
        delta_pct,
        status,
        unit: unit.to_string(),
    });
}
