//! Single-file HTML + inline SVG flamegraph report generator.

pub mod breakdown;
pub mod breakdown_svg;
pub mod css;
pub mod escape;
pub mod hw_table;
pub mod kpi;
#[cfg(test)]
mod tests;

pub use escape::escape_html;

use crate::hw::HardwareSnapshot;
use crate::trace::TraceSession;

/// Renders a full self-contained HTML + SVG flamegraph report.
pub fn render_html_report(session: &TraceSession, hw: Option<&HardwareSnapshot>) -> String {
    let css = css::get_report_css();
    let kpi = kpi::render_kpi_cards(session);
    let (breakdown, _) = breakdown::compute_stage_breakdown(session);
    let breakdown_bars = breakdown_svg::render_breakdown_bars(&breakdown);
    let timeline_svg = timeline_svg::render_timeline_svg(session);
    let hw_table = hw_table::render_hw_table(hw);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>InferProbe Execution Report</title>
<style>{}</style>
</head>
<body>
<div class="container">
  <div class="header">
    <div class="title">INFERPROBE // EXECUTION REPORT</div>
    <div class="subtitle">Deterministic latency percentiles, stage telemetry, and hardware snapshot</div>
  </div>
  {}
  {}
  {}
  {}
</div>
</body>
</html>"#,
        css, kpi, breakdown_bars, timeline_svg, hw_table
    )
}

mod timeline_svg;
