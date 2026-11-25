//! Live command benchmark runner with p50/p95/p99 latency and CI gate.

use std::process::{self, Command, Stdio};
use std::time::Instant;

use inferprobe_core::{LatencyHistogram, ProcessTelemetry};

/// Runs a command `runs` times, prints p50/p95/p99 ms, and gates on `max_p99`.
pub fn exec_bench_cmd(cmd: &str, args: &[String], runs: usize, max_p99: Option<f64>, json: bool) {
    if runs == 0 || cmd.is_empty() {
        eprintln!("Error: runs must be >= 1 and command cannot be empty");
        process::exit(1);
    }
    let mut c = Command::new(cmd);
    c.args(args).stdout(Stdio::null()).stderr(Stdio::null());
    let mut hist = LatencyHistogram::new();
    for _ in 0..runs {
        let t0 = Instant::now();
        let st = c.status().unwrap_or_else(|e| {
            eprintln!("Error spawning '{}': {}", cmd, e);
            process::exit(1);
        });
        hist.record(t0.elapsed().as_secs_f64() * 1e6);
        if !st.success() {
            eprintln!("Error: command exited with {:?}", st.code());
            process::exit(st.code().unwrap_or(1));
        }
    }
    let sum = hist.summarize(None).expect("samples");
    let (p50, p95, p99) = (sum.p50_us / 1e3, sum.p95_us / 1e3, sum.p99_us / 1e3);
    let rss = ProcessTelemetry::sample().rss_kb;
    let passed = max_p99.is_none_or(|limit| p99 <= limit);
    if json {
        print!("{{\"runs\":{},\"p50_ms\":{:.4},", runs, p50);
        print!("\"p95_ms\":{:.4},\"p99_ms\":{:.4},", p95, p99);
        println!("\"rss_kb\":{},\"passed\":{}}}", rss, passed);
    } else {
        print!("runs={} p50={:.3}ms p95={:.3}ms ", runs, p50, p95);
        println!("p99={:.3}ms rss={}kb passed={}", p99, rss, passed);
    }
    if !passed {
        eprintln!("Error: p99 {:.4}ms exceeded --max-p99-ms", p99);
        process::exit(1);
    }
}
