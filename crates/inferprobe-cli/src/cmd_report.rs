//! Report subcommand implementation.

use std::fs;
use std::process;

use inferprobe_core::{
    collect_host_hardware, parse_trace_json_or_ndjson, render_html_report, TraceSession,
};

/// Executes the report subcommand.
pub fn exec_report(trace_path: &str, out_path: &str) {
    let content = fs::read_to_string(trace_path).unwrap_or_else(|e| {
        eprintln!("Error: failed to read trace '{}': {}", trace_path, e);
        process::exit(1);
    });

    let session = parse_trace_content(&content).unwrap_or_else(|e| {
        eprintln!("Error: failed to parse trace '{}': {}", trace_path, e);
        process::exit(1);
    });

    let hw = collect_host_hardware();
    let html = render_html_report(&session, Some(&hw));

    fs::write(out_path, html).unwrap_or_else(|e| {
        eprintln!("Error: failed to write report '{}': {}", out_path, e);
        process::exit(1);
    });

    println!("Report generated successfully: {}", out_path);
}

fn parse_trace_content(content: &str) -> Result<TraceSession, String> {
    parse_trace_json_or_ndjson(content)
}
