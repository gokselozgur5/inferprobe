//! Stage breakdown SVG progress bars and percentile breakdown tables.

use super::breakdown::StageStat;
use super::escape::escape_html;

/// Renders stage breakdown bars and percentile breakdown table.
pub fn render_breakdown_bars(breakdown: &[StageStat]) -> String {
    let mut out = String::from(
        "<div class=\"section-title\">STAGE TELEMETRY &amp; LATENCY PERCENTILES</div>\n",
    );

    out.push_str("<div class=\"breakdown-container\">\n");
    for item in breakdown {
        let label = escape_html(&item.name);
        out.push_str(&format!(
            "<div class=\"breakdown-row\">\n  <div class=\"breakdown-label\">{}</div>\n  <div class=\"bar-track\"><div class=\"bar-fill\" style=\"width:{:.1}%\"></div></div>\n  <div class=\"breakdown-val\">{:.1}% ({} us, {} calls)</div>\n</div>\n",
            label, item.pct_of_total, item.pct_of_total, item.total_us, item.count
        ));
    }
    out.push_str("</div>\n");

    out.push_str("<table class=\"breakdown-table\">\n");
    out.push_str("<thead><tr><th>Stage</th><th>Calls</th><th>Total (us)</th><th>Share</th><th>p50 (us)</th><th>p90 (us)</th><th>p99 (us)</th></tr></thead>\n<tbody>\n");
    for item in breakdown {
        out.push_str(&format!(
            "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{:.1}%</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>\n",
            escape_html(&item.name),
            item.count,
            item.total_us,
            item.pct_of_total,
            item.p50_us,
            item.p90_us,
            item.p99_us
        ));
    }
    out.push_str("</tbody></table>\n");

    out
}
