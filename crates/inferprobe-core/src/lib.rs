//! Deterministic profiling primitives for Edge AI and pipeline execution.

pub mod histogram;
pub mod trace;
pub use histogram::{LatencyHistogram, LatencySummary};
pub use trace::{Span, TraceSession};

/// Returns the inferprobe core version string.
pub fn version() -> &'static str {
    "0.1.0"
}
