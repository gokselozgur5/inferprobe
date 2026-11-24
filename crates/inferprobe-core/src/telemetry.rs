//! Portable host telemetry sampler and synthetic inference pipeline runner.

use std::fs;
use std::time::Instant;

use crate::histogram::LatencyHistogram;
use crate::trace::{Span, TraceSession};

/// Snapshot of host process resources.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessTelemetry {
    pub pid: u32,
    pub rss_kb: u64,
}

impl ProcessTelemetry {
    pub fn sample() -> Self {
        let pid = std::process::id();
        let rss_kb = read_rss_kb().unwrap_or(0);
        Self { pid, rss_kb }
    }
}

/// Reads resident set size (RSS) in KB.
/// Checks /proc/self/status on Linux; falls back gracefully on other platforms.
fn read_rss_kb() -> Option<u64> {
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        return Some(kb);
                    }
                }
            }
        }
    }
    None
}
