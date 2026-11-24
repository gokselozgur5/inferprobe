//! Stage breakdown SVG progress bars.

use super::breakdown::StageStat;
use super::escape::escape_html;

/// Renders stage breakdown bars in SVG.
pub fn render_breakdown_bars(breakdown: &[StageStat]) -> String {
    let mut svg = String::from(
        "<div class=\"section-title\">STAGE DURATION BREAKDOWN</div>\n<div class=\"breakdown-container\">\n",
    );
    for item in breakdown {
        let label = escape_html(&item.name);
        svg.push_str(&format!(
            "<div class=\"breakdown-row\">\n  <div class=\"breakdown-label\">{}</div>\n  <div class=\"bar-track\"><div class=\"bar-fill\" style=\"width:{:.1}%\"></div></div>\n  <div class=\"breakdown-val\">{:.1}% ({} us, {} calls)</div>\n</div>\n",
            label, item.pct_of_total, item.pct_of_total, item.total_us, item.count
        ));
    }
    svg.push_str("</div>\n");
    svg
}
