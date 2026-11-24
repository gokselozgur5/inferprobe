//! Horizontal SVG flamegraph / timeline generator.

use super::escape::escape_html;
use crate::trace::TraceSession;

const PALETTE: [&str; 6] = [
    "#38bdf8", "#818cf8", "#34d399", "#f472b6", "#fbbf24", "#a78bfa",
];

/// Renders horizontal SVG timeline for spans in a trace session.
pub fn render_timeline_svg(session: &TraceSession) -> String {
    if session.is_empty() {
        return "<div class=\"muted\">No spans recorded in trace.</div>".into();
    }

    let min_start = session
        .spans()
        .iter()
        .map(|s| s.start_us)
        .min()
        .unwrap_or(0);
    let max_end = session
        .spans()
        .iter()
        .map(|s| s.start_us.saturating_add(s.duration_us))
        .max()
        .unwrap_or(1);
    let total_span_us = max_end.saturating_sub(min_start).max(1);

    let svg_w = 960;
    let row_h = 28;
    let svg_h = (session.len() * row_h + 40).max(120);

    let mut svg = format!(
        "<div class=\"section-title\">HORIZONTAL EXECUTION TIMELINE</div>\n<svg class=\"flame-svg\" viewBox=\"0 0 {} {}\" width=\"100%\" height=\"{}\">\n",
        svg_w, svg_h, svg_h
    );

    for (i, span) in session.spans().iter().enumerate() {
        let y = 20 + i * row_h;
        let rel_start = span.start_us.saturating_sub(min_start);
        let x = (rel_start as f64 / total_span_us as f64) * (svg_w - 40) as f64 + 20.0;
        let w = ((span.duration_us as f64 / total_span_us as f64) * (svg_w - 40) as f64).max(3.0);
        let color = PALETTE[i % PALETTE.len()];
        let name_esc = escape_html(&span.name);
        let stage_esc = escape_html(&span.stage);
        let dev_esc = escape_html(&span.device);

        svg.push_str(&format!(
            "  <g class=\"flame-rect\"><rect x=\"{:.1}\" y=\"{}\" width=\"{:.1}\" height=\"22\" rx=\"3\" fill=\"{}\"><title>{} [{} | {}] - start: {} us, dur: {} us, rss: {} KB</title></rect><text x=\"{:.1}\" y=\"{}\" fill=\"#0f172a\" font-size=\"11\" font-weight=\"600\" font-family=\"monospace\">{}</text></g>\n",
            x, y, w, color, name_esc, stage_esc, dev_esc, span.start_us, span.duration_us, span.rss_kb, x + 6.0, y + 15, name_esc
        ));
    }

    svg.push_str("</svg>\n");
    svg
}
