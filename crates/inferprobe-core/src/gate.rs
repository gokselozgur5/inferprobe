//! Regression gate policy evaluation between baseline and current profiles.

use crate::histogram::LatencySummary;

/// Threshold configuration for regression gate evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct RegressionPolicy {
    pub max_p99_regression_pct: f64,
    pub max_mean_regression_pct: f64,
    pub max_rss_regression_pct: f64,
}

impl Default for RegressionPolicy {
    fn default() -> Self {
        Self {
            max_p99_regression_pct: 10.0,
            max_mean_regression_pct: 5.0,
            max_rss_regression_pct: 15.0,
        }
    }
}

/// Structured outcome of a regression check.
#[derive(Debug, Clone, PartialEq)]
pub struct RegressionReport {
    pub passed: bool,
    pub p99_change_pct: f64,
    pub mean_change_pct: f64,
    pub rss_change_pct: f64,
    pub violations: Vec<String>,
}

impl RegressionReport {
    pub fn to_json(&self) -> String {
        let violations_json = self
            .violations
            .iter()
            .map(|v| format!("\"{}\"", v))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"passed\":{},\"p99_change_pct\":{:.2},\"mean_change_pct\":{:.2},\"rss_change_pct\":{:.2},\"violations\":[{}]}}",
            self.passed,
            self.p99_change_pct,
            self.mean_change_pct,
            self.rss_change_pct,
            violations_json
        )
    }
}

/// Evaluates latency and memory regression against a baseline profile.
pub fn evaluate_regression(
    baseline_lat: &LatencySummary,
    current_lat: &LatencySummary,
    baseline_rss_kb: u64,
    current_rss_kb: u64,
    policy: &RegressionPolicy,
) -> RegressionReport {
    let p99_change = pct_change(baseline_lat.p99_us, current_lat.p99_us);
    let mean_change = pct_change(baseline_lat.mean_us, current_lat.mean_us);
    let rss_change = pct_change(baseline_rss_kb as f64, current_rss_kb as f64);

    let mut violations = Vec::new();
    if p99_change > policy.max_p99_regression_pct {
        violations.push(format!(
            "p99 latency regression: +{:.2}% exceeds threshold +{:.2}%",
            p99_change, policy.max_p99_regression_pct
        ));
    }
    if mean_change > policy.max_mean_regression_pct {
        violations.push(format!(
            "mean latency regression: +{:.2}% exceeds threshold +{:.2}%",
            mean_change, policy.max_mean_regression_pct
        ));
    }
    if rss_change > policy.max_rss_regression_pct {
        violations.push(format!(
            "RSS memory regression: +{:.2}% exceeds threshold +{:.2}%",
            rss_change, policy.max_rss_regression_pct
        ));
    }

    RegressionReport {
        passed: violations.is_empty(),
        p99_change_pct: p99_change,
        mean_change_pct: mean_change,
        rss_change_pct: rss_change,
        violations,
    }
}

fn pct_change(base: f64, curr: f64) -> f64 {
    if base <= 0.0 {
        if curr <= 0.0 {
            0.0
        } else {
            100.0
        }
    } else {
        ((curr - base) / base) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_summary(mean: f64, p99: f64) -> LatencySummary {
        LatencySummary {
            count: 10,
            min_us: 1.0,
            max_us: p99,
            mean_us: mean,
            stddev_us: 1.0,
            jitter_us: 0.5,
            p50_us: mean,
            p90_us: p99 * 0.9,
            p95_us: p99 * 0.95,
            p99_us: p99,
            p999_us: p99,
            throughput_ops_sec: 100.0,
        }
    }

    #[test]
    fn test_gate_pass_and_violation() {
        let policy = RegressionPolicy::default();
        let base = dummy_summary(100.0, 200.0);
        let curr_ok = dummy_summary(102.0, 205.0);
        let report_ok = evaluate_regression(&base, &curr_ok, 1024, 1024, &policy);
        assert!(report_ok.passed);
        assert!(report_ok.violations.is_empty());

        let curr_fail = dummy_summary(120.0, 250.0);
        let report_fail = evaluate_regression(&base, &curr_fail, 1024, 2048, &policy);
        assert!(!report_fail.passed);
        assert_eq!(report_fail.violations.len(), 3);
    }
}
