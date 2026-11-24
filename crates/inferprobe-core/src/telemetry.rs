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

/// Specification of a stage within a synthetic pipeline.
#[derive(Debug, Clone)]
pub struct PipelineStageSpec {
    pub name: String,
    pub simulated_work_ops: u64,
    pub simulated_device: String,
}

impl PipelineStageSpec {
    pub fn new(
        name: impl Into<String>,
        simulated_work_ops: u64,
        simulated_device: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            simulated_work_ops,
            simulated_device: simulated_device.into(),
        }
    }
}

/// Runs a deterministic synthetic inference pipeline across specified iterations.
pub fn run_synthetic_pipeline(
    stages: &[PipelineStageSpec],
    iterations: usize,
) -> (LatencyHistogram, TraceSession) {
    let mut histogram = LatencyHistogram::new();
    let mut session = TraceSession::new();
    let session_start = Instant::now();

    for _ in 0..iterations {
        let iter_start = Instant::now();
        for stage in stages {
            let stage_start_us = session_start.elapsed().as_micros() as u64;
            let stage_timer = Instant::now();

            let mut sink: u64 = 0;
            for i in 0..stage.simulated_work_ops {
                sink = sink.wrapping_add(i.wrapping_mul(31));
            }
            std::hint::black_box(sink);

            let duration_us = stage_timer.elapsed().as_micros().max(1) as u64;
            let telemetry = ProcessTelemetry::sample();
            session.add_span(Span::new(
                &stage.name,
                "inference",
                stage_start_us,
                duration_us,
                &stage.simulated_device,
                telemetry.rss_kb,
            ));
        }
        let total_iter_us = iter_start.elapsed().as_micros() as f64;
        histogram.record(total_iter_us);
    }

    (histogram, session)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_and_synthetic_pipeline() {
        let sample = ProcessTelemetry::sample();
        assert!(sample.pid > 0);

        let stages = vec![
            PipelineStageSpec::new("preprocess", 1_000, "cpu"),
            PipelineStageSpec::new("tensorrt_gemm", 5_000, "gpu:0"),
        ];
        let (hist, session) = run_synthetic_pipeline(&stages, 5);
        assert_eq!(hist.len(), 5);
        assert_eq!(session.len(), 10);
    }
}
