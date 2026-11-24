//! Deterministic profiling primitives for Edge AI and pipeline execution.

pub mod histogram;
pub use histogram::{LatencyHistogram, LatencySummary};

/// Returns the inferprobe core version string.
pub fn version() -> &'static str {
    "0.1.0"
}
