//! Unit tests for flamegraph report rendering.

#[cfg(test)]
mod tests {
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
    }
}
