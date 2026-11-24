//! Compare subcommand logic.

use std::fs;
use std::process;

use inferprobe_core::{
    evaluate_regression, parse_trace_json_or_ndjson, LatencyHistogram, LatencySummary,
    RegressionPolicy,
};

pub fn exec_compare(base_p: &str, curr_p: &str, p99_th: Option<f64>, mean_th: Option<f64>) {
    let mut policy = RegressionPolicy::default();
    if let Some(v) = p99_th {
        policy.max_p99_regression_pct = v;
    }
    if let Some(v) = mean_th {
        policy.max_mean_regression_pct = v;
    }
    let b_str = fs::read_to_string(base_p).unwrap_or_else(|e| {
        eprintln!("Error: failed to read baseline '{}': {}", base_p, e);
        process::exit(1);
    });
    let c_str = fs::read_to_string(curr_p).unwrap_or_else(|e| {
        eprintln!("Error: failed to read current '{}': {}", curr_p, e);
        process::exit(1);
    });

    let (b_mean, b_p99) = extract_metrics(&b_str).unwrap_or_else(|e| {
        eprintln!("Error: baseline '{}' invalid: {}", base_p, e);
        process::exit(1);
    });
    let (c_mean, c_p99) = extract_metrics(&c_str).unwrap_or_else(|e| {
        eprintln!("Error: current '{}' invalid: {}", curr_p, e);
        process::exit(1);
    });

    let d_base = dummy_sum(b_mean, b_p99);
    let d_curr = dummy_sum(c_mean, c_p99);
    let rep = evaluate_regression(&d_base, &d_curr, 1024, 1024, &policy);
    println!(
        "p99 change: {:+.2}% | mean change: {:+.2}%",
        rep.p99_change_pct, rep.mean_change_pct
    );
    if rep.passed {
        println!("PASSED: No regression detected.");
    } else {
        eprintln!("FAILED: Regression gate violated: {:?}", rep.violations);
        process::exit(2);
    }
}

fn extract_metrics(content: &str) -> Result<(f64, f64), String> {
    if let (Some(mean), Some(p99)) = (parse_key(content, "mean_us"), parse_key(content, "p99_us")) {
        return Ok((mean, p99));
    }
    // Attempt summarizing spans from TraceSession
    if let Ok(session) = parse_trace_json_or_ndjson(content) {
        if !session.is_empty() {
            let mut hist = LatencyHistogram::new();
            for span in session.spans() {
                hist.record(span.duration_us as f64);
            }
            if let Some(sum) = hist.summarize(None) {
                return Ok((sum.mean_us, sum.p99_us));
            }
        }
    }
    Err("missing required metrics 'mean_us' and 'p99_us'".into())
}

pub fn parse_key(json: &str, key: &str) -> Option<f64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)?;
    let rem = json[pos + needle.len()..].trim_start();
    let num: String = rem
        .chars()
        .take_while(|c| {
            c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+' || *c == 'e' || *c == 'E'
        })
        .collect();
    num.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn dummy_sum(mean: f64, p99: f64) -> LatencySummary {
    LatencySummary {
        count: 1,
        min_us: 0.0,
        max_us: p99,
        mean_us: mean,
        stddev_us: 0.0,
        jitter_us: 0.0,
        p50_us: mean,
        p90_us: p99,
        p95_us: p99,
        p99_us: p99,
        p999_us: p99,
        throughput_ops_sec: 1.0,
    }
}
