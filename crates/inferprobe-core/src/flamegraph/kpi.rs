//! Latency metrics computation and KPI cards renderer.

use crate::histogram::LatencyHistogram;
use crate::trace::TraceSession;

/// Renders KPI summary cards for latency percentiles and throughput.
pub fn render_kpi_cards(session: &TraceSession) -> String {
    let mut hist = LatencyHistogram::new();
    for span in session.spans() {
        hist.record(span.duration_us);
    }
    let summary = hist.summarize(None);

    let (p50, p95, p99, p999, mean, jitter, throughput) = if let Some(s) = summary {
        (
            format!("{:.1} us", s.p50_us),
            format!("{:.1} us", s.p95_us),
            format!("{:.1} us", s.p99_us),
            format!("{:.1} us", s.p999_us),
            format!("{:.1} us", s.mean_us),
            format!("{:.1} us", s.jitter_us),
            format!("{:.1} ops/s", s.throughput_ops_sec),
        )
    } else {
        (
            "-".into(),
            "-".into(),
            "-".into(),
            "-".into(),
            "-".into(),
            "-".into(),
            "-".into(),
        )
    };

    format!(
        "<div class=\"section-title\">LATENCY PERCENTILES &amp; THROUGHPUT</div>\n<div class=\"cards-grid\">\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">p50 Latency</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">p95 Latency</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">p99 Latency</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">p99.9 Latency</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">Mean Latency</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">Jitter</div></div>\n  <div class=\"card\"><div class=\"card-val\">{}</div><div class=\"card-lbl\">Throughput</div></div>\n</div>\n",
        p50, p95, p99, p999, mean, jitter, throughput
    )
}
