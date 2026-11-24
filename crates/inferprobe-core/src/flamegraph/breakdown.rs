//! Stage breakdown aggregation for flamegraph reports.

use crate::trace::TraceSession;
use std::collections::BTreeMap;

/// Aggregated metrics for an execution stage.
#[derive(Debug, Clone, PartialEq)]
pub struct StageStat {
    pub name: String,
    pub count: usize,
    pub total_us: u64,
    pub pct_of_total: f64,
}

/// Computes per-stage aggregate statistics from a trace session.
pub fn compute_stage_breakdown(session: &TraceSession) -> (Vec<StageStat>, u64) {
    let mut map: BTreeMap<String, (usize, u64)> = BTreeMap::new();
    let mut grand_total_us = 0u64;

    for span in session.spans() {
        let entry = map.entry(span.stage.clone()).or_insert((0, 0));
        entry.0 += 1;
        entry.1 = entry.1.saturating_add(span.duration_us);
        grand_total_us = grand_total_us.saturating_add(span.duration_us);
    }

    let mut list = Vec::new();
    for (stage, (count, total_us)) in map {
        let pct = if grand_total_us > 0 {
            (total_us as f64 / grand_total_us as f64) * 100.0
        } else {
            0.0
        };
        list.push(StageStat {
            name: stage,
            count,
            total_us,
            pct_of_total: pct,
        });
    }

    (list, grand_total_us)
}
