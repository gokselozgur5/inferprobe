//! Subprocess execution profiling runner.

use crate::telemetry::ProcessTelemetry;
use crate::trace::{Span, TraceSession};
use std::process::Command;
use std::time::Instant;

/// Runs a child process command repeatedly, recording duration spans and peak RSS.
pub fn run_subprocess_benchmark(
    cmd: &str,
    args: &[String],
    iterations: usize,
) -> Result<(TraceSession, i32), String> {
    if cmd.is_empty() {
        return Err("command cannot be empty".into());
    }

    let iters = iterations.max(1);
    let mut session = TraceSession::new();
    let mut last_exit_code = 0;
    let base_time = Instant::now();

    for i in 0..iters {
        let span_start_us = base_time.elapsed().as_micros() as u64;
        let run_start = Instant::now();

        let status = Command::new(cmd)
            .args(args)
            .status()
            .map_err(|e| format!("failed to spawn '{}': {}", cmd, e))?;

        let duration_us = run_start.elapsed().as_micros() as u64;
        let rss_kb = ProcessTelemetry::current_rss_kb().unwrap_or(0);
        let code = status.code().unwrap_or(1);
        last_exit_code = code;

        session.add_span(Span::new(
            format!("run_{}", i),
            "subprocess",
            span_start_us,
            duration_us,
            "host",
            rss_kb,
        ));

        if code != 0 {
            break;
        }
    }

    Ok((session, last_exit_code))
}
