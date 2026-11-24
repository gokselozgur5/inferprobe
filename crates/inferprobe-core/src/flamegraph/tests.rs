//! Unit tests for flamegraph report rendering.

use crate::flamegraph::{escape_html, render_html_report};
use crate::hw::HardwareSnapshot;
use crate::trace::{Span, TraceSession};

#[test]
fn test_escape_html_special_chars() {
    let input = "<script>alert('xss') & \"quotes\"</script>";
    let escaped = escape_html(input);
    assert!(!escaped.contains('<'));
    assert!(!escaped.contains('>'));
    assert!(!escaped.contains('"'));
    assert!(!escaped.contains('\''));
    assert!(escaped.contains("&lt;script&gt;"));
    assert!(escaped.contains("&amp;"));
    assert!(escaped.contains("&#39;xss&#39;"));
}

#[test]
fn test_render_html_report_contains_kpi_and_timeline() {
    let mut session = TraceSession::new();
    session.add_span(Span::new("kernel_matmul", "infer", 0, 1200, "gpu:0", 40960));
    session.add_span(Span::new("layer_norm", "infer", 1200, 300, "gpu:0", 40960));

    let hw = HardwareSnapshot::new(
        "linux-x86_64",
        "AMD EPYC",
        Some("NVIDIA A100".into()),
        Some(92.0),
        Some(45000),
        Some(81920),
        Some(58.0),
        Some(380.0),
    );

    let html = render_html_report(&session, Some(&hw));
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("p50 Latency"));
    assert!(html.contains("p99 Latency"));
    assert!(html.contains("kernel_matmul"));
    assert!(html.contains("NVIDIA A100"));
    assert!(html.contains("HORIZONTAL EXECUTION TIMELINE"));
    assert!(html.contains("flame-controls"));
    assert!(html.contains("breakdown-table"));
    assert!(html.contains("filterSpans"));
}

#[test]
fn test_render_timeline_svg_u64_overflow_safety() {
    let mut session = TraceSession::new();
    session.add_span(Span::new(
        "overflow_span",
        "infer",
        u64::MAX - 10,
        20,
        "gpu:0",
        100,
    ));
    session.add_span(Span::new("normal_span", "infer", 100, 200, "gpu:0", 100));
    let svg = crate::flamegraph::timeline_svg::render_timeline_svg(&session);
    assert!(svg.contains("overflow_span"));
}

#[test]
fn test_compute_stage_breakdown_overflow_safety() {
    let mut session = TraceSession::new();
    session.add_span(Span::new("s1", "stage_a", 0, u64::MAX - 5, "cpu", 0));
    session.add_span(Span::new("s2", "stage_a", 10, 20, "cpu", 0));
    let (stats, total) = crate::flamegraph::breakdown::compute_stage_breakdown(&session);
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].total_us, u64::MAX);
    assert_eq!(total, u64::MAX);
}
