//! Run subcommand implementation.

use std::fs;
use std::process;

use inferprobe_core::{collect_host_hardware, render_html_report, run_subprocess_benchmark};

/// Executes the run subcommand.
pub fn exec_run(
    cmd: &str,
    args: &[String],
    iterations: usize,
    out_file: Option<String>,
    html_file: Option<String>,
) {
    let (session, exit_code) =
        run_subprocess_benchmark(cmd, args, iterations).unwrap_or_else(|e| {
            eprintln!("Error executing subprocess benchmark: {}", e);
            process::exit(1);
        });

    if let Some(ref path) = out_file {
        if let Err(e) = fs::write(path, session.to_json()) {
            eprintln!("Error saving trace JSON to '{}': {}", path, e);
        }
    }

    if let Some(ref path) = html_file {
        let hw = collect_host_hardware();
        let html = render_html_report(&session, Some(&hw));
        if let Err(e) = fs::write(path, html) {
            eprintln!("Error saving HTML report to '{}': {}", path, e);
        }
    }

    if exit_code != 0 {
        process::exit(exit_code);
    }
}
