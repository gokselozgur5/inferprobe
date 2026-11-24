//! Horizontal interactive SVG flamegraph / timeline generator.

use super::escape::escape_html;
use crate::trace::TraceSession;

const PALETTE: [&str; 8] = [
    "#38bdf8", "#818cf8", "#34d399", "#f472b6", "#fbbf24", "#a78bfa", "#f97316", "#2dd4bf",
];

/// Renders horizontal interactive SVG timeline for spans in a trace session.
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

    let svg_w = 1040;
    let row_h = 28;
    let svg_h = (session.len() * row_h + 50).max(140);

    let mut out = String::new();
    out.push_str(
        "<div class=\"section-title\">HORIZONTAL EXECUTION TIMELINE &amp; FLAMEGRAPH</div>\n",
    );

    // Controls: search filter and zoom controls
    out.push_str(r#"<div class="flame-controls">
  <input type="text" id="flame-search" placeholder="Filter spans (e.g. matmul, infer, gpu:0)..." oninput="filterSpans(this.value)" />
  <div class="flame-btn-group">
    <button class="flame-btn" onclick="zoomFlame(1.25)">Zoom +</button>
    <button class="flame-btn" onclick="zoomFlame(0.8)">Zoom -</button>
    <button class="flame-btn" onclick="resetFlame()">Reset</button>
  </div>
  <div id="flame-details" class="flame-details">Hover or click a span to inspect trace attributes</div>
</div>
"#);

    out.push_str(&format!(
        "<div class=\"flame-viewport\"><svg id=\"flamegraph-svg\" class=\"flame-svg\" viewBox=\"0 0 {} {}\" width=\"100%\" height=\"{}\">\n",
        svg_w, svg_h, svg_h
    ));

    // Time axis ticks
    out.push_str("  <g class=\"axis-group\">\n");
    for i in 0..=5 {
        let pct = i as f64 / 5.0;
        let tick_x = 20.0 + pct * (svg_w - 40) as f64;
        let tick_time = (pct * total_span_us as f64) as u64;
        out.push_str(&format!(
            "    <line x1=\"{:.1}\" y1=\"18\" x2=\"{:.1}\" y2=\"{}\" stroke=\"#1e293b\" stroke-width=\"1\" stroke-dasharray=\"3 3\" />\n",
            tick_x, tick_x, svg_h - 10
        ));
        out.push_str(&format!(
            "    <text x=\"{:.1}\" y=\"14\" fill=\"#64748b\" font-size=\"10\" font-family=\"monospace\">+{} us</text>\n",
            tick_x, tick_time
        ));
    }
    out.push_str("  </g>\n");

    out.push_str("  <g id=\"flame-nodes\">\n");
    for (i, span) in session.spans().iter().enumerate() {
        let y = 26 + i * row_h;
        let rel_start = span.start_us.saturating_sub(min_start);
        let x = (rel_start as f64 / total_span_us as f64) * (svg_w - 40) as f64 + 20.0;
        let w = ((span.duration_us as f64 / total_span_us as f64) * (svg_w - 40) as f64).max(4.0);
        let color = PALETTE[i % PALETTE.len()];
        let name_esc = escape_html(&span.name);
        let stage_esc = escape_html(&span.stage);
        let dev_esc = escape_html(&span.device);

        out.push_str(&format!(
            "    <g class=\"flame-rect\" data-name=\"{}\" data-stage=\"{}\" data-device=\"{}\" data-start=\"{}\" data-dur=\"{}\" data-rss=\"{}\" onclick=\"selectSpan(this)\">\n",
            name_esc, stage_esc, dev_esc, span.start_us, span.duration_us, span.rss_kb
        ));
        out.push_str(&format!(
            "      <rect x=\"{:.1}\" y=\"{}\" width=\"{:.1}\" height=\"22\" rx=\"3\" fill=\"{}\"><title>{} [{} | {}] - start: {} us, dur: {} us, rss: {} KB</title></rect>\n",
            x, y, w, color, name_esc, stage_esc, dev_esc, span.start_us, span.duration_us, span.rss_kb
        ));
        out.push_str(&format!(
            "      <text x=\"{:.1}\" y=\"{}\" fill=\"#0f172a\" font-size=\"11\" font-weight=\"600\" font-family=\"monospace\" clip-path=\"url(#clip-{})\" pointer-events=\"none\">{}</text>\n",
            x + 6.0, y + 15, i, name_esc
        ));
        out.push_str("    </g>\n");
    }
    out.push_str("  </g>\n");

    out.push_str("</svg></div>\n");
    out
}
