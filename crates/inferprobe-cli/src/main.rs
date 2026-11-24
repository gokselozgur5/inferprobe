mod args;
mod cmd_compare;

use std::env;
use std::fs;
use std::process;

use args::{parse_cli_args, CliCommand};
use cmd_compare::{exec_compare, parse_key};
use inferprobe_core::{run_synthetic_pipeline, PipelineStageSpec};

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    match parse_cli_args(&raw_args) {
        Ok(CliCommand::Bench {
            iterations,
            out_file,
            chrome_trace_file,
        }) => {
            exec_bench(iterations, out_file, chrome_trace_file);
        }
        Ok(CliCommand::Compare {
            baseline_file,
            current_file,
            max_p99_pct,
            max_mean_pct,
        }) => {
            exec_compare(&baseline_file, &current_file, max_p99_pct, max_mean_pct);
        }
        Ok(CliCommand::Inspect { file }) => exec_inspect(&file),
        Ok(CliCommand::Help) => print_usage(),
        Err(err) => {
            eprintln!("Error: {}", err);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("inferprobe - Edge AI, GPU & Pipeline Profiler");
    println!("Usage:");
    println!("  inferprobe bench [--iterations N] [--out FILE] [--chrome-trace FILE]");
    println!(
        "  inferprobe compare <baseline.json> <current.json> [--max-p99-pct F] [--max-mean-pct F]"
    );
    println!("  inferprobe inspect <trace.json>");
}

fn exec_bench(iterations: usize, out: Option<String>, chrome: Option<String>) {
    let stages = vec![
        PipelineStageSpec::new("preprocess", 1_000, "cpu"),
        PipelineStageSpec::new("inference", 4_000, "gpu:0"),
        PipelineStageSpec::new("postprocess", 1_000, "cpu"),
    ];
    let (histogram, session) = run_synthetic_pipeline(&stages, iterations);
    let summary = histogram.summarize(None).expect("summary");
    println!("Profile complete ({} iterations):", iterations);
    println!(
        "  p50: {:.2} us | p90: {:.2} us | p99: {:.2} us",
        summary.p50_us, summary.p90_us, summary.p99_us
    );
    println!(
        "  mean: {:.2} us (stddev: {:.2} us) | throughput: {:.2} ops/sec",
        summary.mean_us, summary.stddev_us, summary.throughput_ops_sec
    );
    if let Some(path) = out {
        fs::write(&path, summary.to_json()).expect("write summary");
        println!("Summary saved to {}", path);
    }
    if let Some(path) = chrome {
        fs::write(&path, session.to_chrome_trace()).expect("write trace");
        println!("Chrome trace saved to {}", path);
    }
}

fn exec_inspect(path: &str) {
    let content = fs::read_to_string(path).expect("read file");
    println!("Inspect Summary for {}", path);
    if let (Some(p50), Some(p90), Some(p99)) = (
        parse_key(&content, "p50_us"),
        parse_key(&content, "p90_us"),
        parse_key(&content, "p99_us"),
    ) {
        println!(
            "  p50: {:.2} us | p90: {:.2} us | p99: {:.2} us",
            p50, p90, p99
        );
    } else {
        println!("  Trace file bytes: {}", content.len());
    }
}
