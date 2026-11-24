//! CSS styling for self-contained dark tactical engineering reports.

/// Returns embedded CSS styles for the HTML report.
pub fn get_report_css() -> &'static str {
    r#"
:root {
  --bg: #090d16;
  --surface: #111827;
  --border: #1f2937;
  --text: #e2e8f0;
  --muted: #94a3b8;
  --accent: #38bdf8;
  --accent2: #818cf8;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
  background: var(--bg);
  color: var(--text);
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace;
  padding: 32px 24px;
  line-height: 1.5;
}
.container { max-width: 1100px; margin: 0 auto; }
.header { border-bottom: 1px solid var(--border); padding-bottom: 16px; margin-bottom: 24px; }
.title { font-size: 22px; font-weight: 700; letter-spacing: 0.5px; color: var(--accent); }
.subtitle { color: var(--muted); font-size: 13px; margin-top: 4px; }
.section-title {
  font-size: 13px; font-weight: 700; letter-spacing: 1px; color: var(--muted);
  text-transform: uppercase; margin: 24px 0 12px 0;
}
.cards-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(130px, 1fr)); gap: 12px; margin-bottom: 24px; }
.card { background: var(--surface); border: 1px solid var(--border); border-radius: 6px; padding: 14px 12px; text-align: center; }
.card-val { font-size: 18px; font-weight: 700; color: #f8fafc; font-family: monospace; }
.card-lbl { font-size: 11px; text-transform: uppercase; color: var(--muted); margin-top: 4px; }
.breakdown-container { background: var(--surface); border: 1px solid var(--border); border-radius: 6px; padding: 16px; margin-bottom: 24px; }
.breakdown-row { display: flex; align-items: center; margin-bottom: 10px; font-size: 13px; }
.breakdown-label { width: 140px; font-weight: 600; color: #f8fafc; font-family: monospace; }
.bar-track { flex: 1; height: 12px; background: #1e293b; border-radius: 6px; overflow: hidden; margin: 0 16px; }
.bar-fill { height: 100%; background: linear-gradient(90deg, var(--accent), var(--accent2)); border-radius: 6px; }
.breakdown-val { width: 180px; text-align: right; color: var(--muted); font-family: monospace; }
.flame-svg { background: var(--surface); border: 1px solid var(--border); border-radius: 6px; margin-bottom: 24px; }
.flame-rect rect { transition: opacity 0.15s ease; cursor: pointer; }
.flame-rect:hover rect { opacity: 0.85; }
.hw-table { width: 100%; border-collapse: collapse; background: var(--surface); border: 1px solid var(--border); border-radius: 6px; overflow: hidden; font-size: 13px; }
.hw-table td, .hw-table th { padding: 10px 14px; border-bottom: 1px solid var(--border); }
.hw-table th { background: #1a2234; text-align: left; color: var(--muted); font-weight: 600; width: 30%; }
.hw-table td { color: #f1f5f9; font-family: monospace; }
"#
}
