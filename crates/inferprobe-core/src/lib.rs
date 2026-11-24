//! Deterministic profiling primitives for Edge AI and pipeline execution.

pub mod telemetry;
pub use telemetry::{run_synthetic_pipeline, PipelineStageSpec, ProcessTelemetry};
pub mod gate;
pub mod histogram;
pub use gate::{evaluate_regression, RegressionPolicy, RegressionReport};
pub mod trace;
pub use histogram::{LatencyHistogram, LatencySummary};
pub use trace::{Span, TraceSession};

/// Returns the inferprobe core version string.
pub fn version() -> &'static str {
    "0.1.0"
}
