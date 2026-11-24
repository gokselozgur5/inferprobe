//! Terminal HUD formatting with ANSI/ASCII sparklines and latency jitter bars.

use inferprobe_core::LatencySummary;

const SPARK_CHARS: [char; 8] = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// Generates an ASCII/Unicode sparkline string from a series of values.
pub fn generate_sparkline(values: &[f64], width: usize) -> String {
    if values.is_empty() || width == 0 {
        return String::new();
    }
    let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;

    let sampled: Vec<f64> = if values.len() <= width {
        values.to_vec()
    } else {
        (0..width)
            .map(|i| {
                let idx = (i * values.len()) / width;
                values[idx]
            })
            .collect()
    };

    sampled
        .into_iter()
        .map(|v| {
            if range <= 1e-9 {
                SPARK_CHARS[0]
            } else {
                let norm = ((v - min) / range).clamp(0.0, 1.0);
                let idx = ((norm * 7.0).round() as usize).min(7);
                SPARK_CHARS[idx]
            }
        })
        .collect()
}

/// Generates a terminal jitter distribution bar.
pub fn generate_jitter_bar(jitter_us: f64, mean_us: f64, width: usize) -> String {
    let ratio = if mean_us > 1e-9 {
        (jitter_us / mean_us).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let filled = ((ratio * width as f64).round() as usize).min(width);
    let mut bar = String::with_capacity(width);
    for _ in 0..filled {
        bar.push('■');
    }
    for _ in filled..width {
        bar.push('·');
    }
    bar
}

/// Prints a high-density, professional ANSI telemetry HUD.
pub fn print_bench_hud(summary: &LatencySummary, samples: &[f64], iterations: usize) {
    let use_color = std::env::var("NO_COLOR").is_err();

    let c_cyan = if use_color { "\x1b[38;5;39m" } else { "" };
    let c_green = if use_color { "\x1b[38;5;82m" } else { "" };
    let c_yellow = if use_color { "\x1b[38;5;214m" } else { "" };
    let c_dim = if use_color { "\x1b[38;5;244m" } else { "" };
    let c_bold = if use_color { "\x1b[1m" } else { "" };
    let c_reset = if use_color { "\x1b[0m" } else { "" };

    println!("Profile complete ({} iterations):", iterations);
    println!(
        "  p50: {:.2} us | p90: {:.2} us | p99: {:.2} us",
        summary.p50_us, summary.p90_us, summary.p99_us
    );
    println!(
        "  mean: {:.2} us (stddev: {:.2} us) | throughput: {:.2} ops/sec",
        summary.mean_us, summary.stddev_us, summary.throughput_ops_sec
    );

    let spark = generate_sparkline(samples, 32);
    let jitter_bar = generate_jitter_bar(summary.jitter_us, summary.mean_us, 20);

    println!(
        "{}+------------------------------------------------------------------------+{}",
        c_dim, c_reset
    );
    println!(
        "{}|{}  {}INFERPROBE // BENCHMARK TELEMETRY HUD{}                                  {}|{}",
        c_dim, c_reset, c_bold, c_reset, c_dim, c_reset
    );
    println!(
        "{}+------------------------------------------------------------------------+{}",
        c_dim, c_reset
    );
    println!(
        "{}|{}  Iterations:  {:>8}    {}[{:<32}]{}       {}|{}",
        c_dim, c_reset, iterations, c_cyan, spark, c_reset, c_dim, c_reset
    );
    println!(
        "{}|{}  Throughput:  {:>8.1} ops/s  Min / Max:  {:.1} us / {:.1} us           {}|{}",
        c_dim, c_reset, summary.throughput_ops_sec, summary.min_us, summary.max_us, c_dim, c_reset
    );
    println!(
        "{}+------------------------------------------------------------------------+{}",
        c_dim, c_reset
    );
    println!(
        "{}|{}  PERCENTILES:                                                              {}|{}",
        c_dim, c_reset, c_dim, c_reset
    );
    println!(
        "{}|{}    p50:   {}{:>8.2} us{}    p95:   {}{:>8.2} us{}                           {}|{}",
        c_dim,
        c_reset,
        c_green,
        summary.p50_us,
        c_reset,
        c_yellow,
        summary.p95_us,
        c_reset,
        c_dim,
        c_reset
    );
    println!(
        "{}|{}    p90:   {}{:>8.2} us{}    p99:   {}{:>8.2} us{}                           {}|{}",
        c_dim,
        c_reset,
        c_green,
        summary.p90_us,
        c_reset,
        c_yellow,
        summary.p99_us,
        c_reset,
        c_dim,
        c_reset
    );
    println!(
        "{}|{}    Mean:  {}{:>8.2} us{}    p99.9: {}{:>8.2} us{}                           {}|{}",
        c_dim,
        c_reset,
        c_cyan,
        summary.mean_us,
        c_reset,
        c_yellow,
        summary.p999_us,
        c_reset,
        c_dim,
        c_reset
    );
    println!(
        "{}+------------------------------------------------------------------------+{}",
        c_dim, c_reset
    );
    println!(
        "{}|{}  JITTER & NOISE:                                                           {}|{}",
        c_dim, c_reset, c_dim, c_reset
    );
    println!(
        "{}|{}    StdDev: {:>8.2} us   Jitter: {:>8.2} us [{:<20}]      {}|{}",
        c_dim, c_reset, summary.stddev_us, summary.jitter_us, jitter_bar, c_dim, c_reset
    );
    println!(
        "{}+------------------------------------------------------------------------+{}",
        c_dim, c_reset
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sparkline_empty() {
        assert_eq!(generate_sparkline(&[], 10), "");
    }

    #[test]
    fn test_generate_sparkline_uniform() {
        let values = vec![5.0; 10];
        let spark = generate_sparkline(&values, 5);
        assert_eq!(spark.chars().count(), 5);
        assert!(spark.chars().all(|c| c == ' '));
    }

    #[test]
    fn test_generate_sparkline_ramp() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let spark = generate_sparkline(&values, 8);
        assert_eq!(spark.chars().count(), 8);
        assert_eq!(spark.chars().next().unwrap(), ' ');
        assert_eq!(spark.chars().last().unwrap(), '█');
    }

    #[test]
    fn test_generate_jitter_bar() {
        let bar = generate_jitter_bar(5.0, 10.0, 10);
        assert_eq!(bar.chars().count(), 10);
        assert_eq!(bar, "■■■■■·····");
    }
}
