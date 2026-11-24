//! Compare subcommand logic.

use std::fs;
use std::process;

use inferprobe_core::{evaluate_regression, LatencySummary, RegressionPolicy};

pub fn exec_compare(base_p: &str, curr_p: &str, p99_th: Option<f64>, mean_th: Option<f64>) {
    let mut policy = RegressionPolicy::default();
    if let Some(v) = p99_th { policy.max_p99_regression_pct = v; }
    if let Some(v) = mean_th { policy.max_mean_regression_pct = v; }
    let b_str = fs::read_to_string(base_p).expect("read baseline");
    let c_str = fs::read_to_string(curr_p).expect("read current");
    let b_p99 = parse_key(&b_str, "p99_us").unwrap_or(0.0);
    let b_mean = parse_key(&b_str, "mean_us").unwrap_or(0.0);
    let c_p99 = parse_key(&c_str, "p99_us").unwrap_or(0.0);
    let c_mean = parse_key(&c_str, "mean_us").unwrap_or(0.0);
    let d_base = dummy_sum(b_mean, b_p99);
    let d_curr = dummy_sum(c_mean, c_p99);
    let rep = evaluate_regression(&d_base, &d_curr, 1024, 1024, &policy);
    println!("p99 change: {:+.2}% | mean change: {:+.2}%", rep.p99_change_pct, rep.mean_change_pct);
    if rep.passed {
        println!("PASSED: No regression detected.");
    } else {
        eprintln!("FAILED: Regression gate violated: {:?}", rep.violations);
        process::exit(2);
    }
}

pub fn parse_key(json: &str, key: &str) -> Option<f64> {
    let needle = format!("\"{}\":", key);
    let pos = json.find(&needle)?;
    let rem = &json[pos + needle.len()..];
    let num: String = rem.chars().take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+').collect();
    num.parse::<f64>().ok()
}

fn dummy_sum(mean: f64, p99: f64) -> LatencySummary {
    LatencySummary {
        count: 1, min_us: 0.0, max_us: p99, mean_us: mean, stddev_us: 0.0,
        jitter_us: 0.0, p50_us: mean, p90_us: p99, p95_us: p99, p99_us: p99,
        p999_us: p99, throughput_ops_sec: 1.0,
    }
}
