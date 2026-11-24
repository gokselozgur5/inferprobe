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

/// Accumulator for latency measurements.
#[derive(Debug, Default, Clone)]
pub struct LatencyHistogram {
    samples: Vec<f64>,
}

impl LatencyHistogram {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn record(&mut self, latency_us: f64) {
        if latency_us >= 0.0 && !latency_us.is_nan() && !latency_us.is_infinite() {
            self.samples.push(latency_us);
        }
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn summarize(&self, total_wall_time_us: Option<f64>) -> Option<LatencySummary> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let count = sorted.len();
        let (min_us, max_us) = (sorted[0], sorted[count - 1]);
        let sum: f64 = sorted.iter().copied().sum();
        let mean_us = sum / (count as f64);
        let variance = sorted.iter().map(|&v| (v - mean_us).powi(2)).sum::<f64>() / (count as f64);
        let stddev_us = variance.sqrt();
        let jitter_us = if count > 1 {
            (1..count)
                .map(|i| (sorted[i] - sorted[i - 1]).abs())
                .sum::<f64>()
                / ((count - 1) as f64)
        } else {
            0.0
        };
        let p50_us = percentile(&sorted, 50.0);
        let p90_us = percentile(&sorted, 90.0);
        let p95_us = percentile(&sorted, 95.0);
        let p99_us = percentile(&sorted, 99.0);
        let p999_us = percentile(&sorted, 99.9);
        let wall_sec = total_wall_time_us
            .filter(|&w| w > 0.0)
            .map(|w| w / 1e6)
            .unwrap_or(if sum > 0.0 { sum / 1e6 } else { 1.0 });
        let throughput_ops_sec = if wall_sec > 0.0 {
            (count as f64) / wall_sec
        } else {
            0.0
        };
        Some(LatencySummary {
            count,
            min_us,
            max_us,
            mean_us,
            stddev_us,
            jitter_us,
            p50_us,
            p90_us,
            p95_us,
            p99_us,
            p999_us,
            throughput_ops_sec,
        })
    }
}

fn percentile(sorted: &[f64], pct: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let rank = (pct.clamp(0.0, 100.0) / 100.0) * ((sorted.len() - 1) as f64);
    let (l, u) = (rank.floor() as usize, rank.ceil() as usize);
    if l == u {
        sorted[l]
    } else {
        sorted[l] + (rank - l as f64) * (sorted[u] - sorted[l])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_histogram_metrics() {
        let mut h = LatencyHistogram::new();
        for i in 1..=100 {
            h.record(i as f64);
        }
        let s = h.summarize(Some(100_000.0)).expect("summary");
        assert_eq!(s.count, 100);
        assert!((s.mean_us - 50.5).abs() < 1e-6);
        assert!((s.p50_us - 50.5).abs() < 1e-6);
        assert!((s.throughput_ops_sec - 1000.0).abs() < 1e-3);
    }

    #[test]
    fn test_chronological_jitter() {
        let mut h = LatencyHistogram::new();
        for _ in 0..5 {
            h.record(10.0);
            h.record(110.0);
        }
        let s = h.summarize(None).expect("summary");
        assert!((s.jitter_us - 100.0).abs() < 1e-6);
    }
}
