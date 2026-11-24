//! Deterministic latency percentiles and statistical summaries.

/// Statistical summary of observed latency samples.
#[derive(Debug, Clone, PartialEq)]
pub struct LatencySummary {
    pub count: usize,
    pub min_us: f64,
    pub max_us: f64,
    pub mean_us: f64,
    pub stddev_us: f64,
    pub jitter_us: f64,
    pub p50_us: f64,
    pub p90_us: f64,
    pub p95_us: f64,
    pub p99_us: f64,
    pub p999_us: f64,
    pub throughput_ops_sec: f64,
}

impl LatencySummary {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"count\":{},\"min_us\":{:.3},\"max_us\":{:.3},\"mean_us\":{:.3},\"stddev_us\":{:.3},\"jitter_us\":{:.3},\"p50_us\":{:.3},\"p90_us\":{:.3},\"p95_us\":{:.3},\"p99_us\":{:.3},\"p999_us\":{:.3},\"throughput_ops_sec\":{:.3}}}",
            self.count, self.min_us, self.max_us, self.mean_us, self.stddev_us,
            self.jitter_us, self.p50_us, self.p90_us, self.p95_us, self.p99_us,
            self.p999_us, self.throughput_ops_sec
        )
    }
}
