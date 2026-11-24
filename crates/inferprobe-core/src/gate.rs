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
