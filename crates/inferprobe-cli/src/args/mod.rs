//! Main CLI argument router.

pub mod parse_report_ingest;
pub mod parse_run;
pub mod types;

pub use types::CliCommand;

use parse_report_ingest::{parse_ingest, parse_report};
use parse_run::parse_run;

pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, String> {
    if args.len() < 2 {
        return Ok(CliCommand::Help);
    }
    match args[1].as_str() {
        "bench" => parse_bench(&args[2..]),
        "compare" => parse_compare(&args[2..]),
        "inspect" => parse_inspect(&args[2..]),
        "report" => parse_report(&args[2..]),
        "ingest" => parse_ingest(&args[2..]),
        "run" => parse_run(&args[2..]),
        "hw" => Ok(CliCommand::Hw),
        "-h" | "--help" | "help" => Ok(CliCommand::Help),
        other => Err(format!("unknown command '{}'", other)),
    }
}

fn parse_bench(args: &[String]) -> Result<CliCommand, String> {
    let mut iterations = 100;
    let mut out_file = None;
    let mut chrome_trace_file = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--iterations" if i + 1 < args.len() => {
                iterations = args[i + 1].parse().unwrap_or(100);
                i += 2;
            }
            "--out" if i + 1 < args.len() => {
                out_file = Some(args[i + 1].clone());
                i += 2;
            }
            "--chrome-trace" if i + 1 < args.len() => {
                chrome_trace_file = Some(args[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }
    Ok(CliCommand::Bench {
        iterations,
        out_file,
        chrome_trace_file,
    })
}

fn parse_compare(args: &[String]) -> Result<CliCommand, String> {
    if args.len() < 2 {
        return Err("compare requires <baseline.json> <current.json>".into());
    }
    let (baseline_file, current_file) = (args[0].clone(), args[1].clone());
    let mut max_p99_pct = None;
    let mut max_mean_pct = None;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--max-p99-pct" if i + 1 < args.len() => {
                max_p99_pct = args[i + 1].parse().ok();
                i += 2;
            }
            "--max-mean-pct" if i + 1 < args.len() => {
                max_mean_pct = args[i + 1].parse().ok();
                i += 2;
            }
            _ => i += 1,
        }
    }
    Ok(CliCommand::Compare {
        baseline_file,
        current_file,
        max_p99_pct,
        max_mean_pct,
    })
}

fn parse_inspect(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("inspect requires <trace.json>".into());
    }
    Ok(CliCommand::Inspect {
        file: args[0].clone(),
    })
}
