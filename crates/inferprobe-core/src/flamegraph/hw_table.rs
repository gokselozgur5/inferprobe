//! Hardware snapshot HTML table renderer.

use super::escape::escape_html;
use crate::hw::HardwareSnapshot;

/// Renders hardware snapshot HTML table.
pub fn render_hw_table(hw: Option<&HardwareSnapshot>) -> String {
    let mut out = String::from("<div class=\"section-title\">HARDWARE SNAPSHOT</div>\n");
    if let Some(h) = hw {
        out.push_str("<table class=\"hw-table\">\n");
        out.push_str(&format!(
            "<tr><th>Platform</th><td>{}</td></tr>\n",
            escape_html(&h.platform)
        ));
        out.push_str(&format!(
            "<tr><th>CPU Model</th><td>{}</td></tr>\n",
            escape_html(&h.cpu_model)
        ));
        out.push_str(&format!(
            "<tr><th>GPU Model</th><td>{}</td></tr>\n",
            h.gpu_name
                .as_deref()
                .map(escape_html)
                .unwrap_or_else(|| "None detected".into())
        ));
        if let Some(util) = h.gpu_util_pct {
            out.push_str(&format!("<tr><th>GPU Utilization</th><td>{:.1}%</td></tr>\n", util));
        }
        if let (Some(used), Some(total)) = (h.vram_used_mb, h.vram_total_mb) {
            out.push_str(&format!(
                "<tr><th>VRAM Usage</th><td>{} / {} MB ({:.1}%)</td></tr>\n",
                used,
                total,
                (used as f64 / total.max(1) as f64) * 100.0
            ));
        }
        if let Some(temp) = h.temp_c {
            out.push_str(&format!("<tr><th>GPU Temperature</th><td>{:.1} °C</td></tr>\n", temp));
        }
        if let Some(pwr) = h.power_w {
            out.push_str(&format!("<tr><th>GPU Power Draw</th><td>{:.1} W</td></tr>\n", pwr));
        }
        out.push_str("</table>\n");
    } else {
        out.push_str("<div class=\"card\">No hardware snapshot available.</div>\n");
    }
    out
}
