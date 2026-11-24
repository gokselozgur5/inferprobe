//! Deterministic Edge AI, GPU & Pipeline Profiler Core.
//!
//! Provides zero-dependency latency percentiles, structured tracing,
//! Chrome Trace export, telemetry sampling, and regression gating.

pub mod flamegraph;
pub mod gate;
pub mod histogram;
pub mod hw;
pub mod ndjson;
pub mod subproc;
pub mod telemetry;
pub mod trace;

pub use flamegraph::{escape_html, render_html_report};
pub use gate::{evaluate_regression, RegressionPolicy, RegressionReport};
pub use histogram::{LatencyHistogram, LatencySummary};
pub use hw::{collect_host_hardware, parse_nvidia_smi_csv, HardwareSnapshot};
pub use ndjson::{parse_ndjson_spans, parse_span_line, parse_trace_json_or_ndjson};
pub use subproc::run_subprocess_benchmark;
pub use telemetry::{run_synthetic_pipeline, PipelineStageSpec, ProcessTelemetry};
pub use trace::{Span, TraceSession};

/// Returns the inferprobe core version string.
pub fn version() -> &'static str {
    "0.1.0"
}
