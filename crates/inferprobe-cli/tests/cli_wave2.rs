//! End-to-end integration tests for Wave 2 CLI subcommands.

use std::fs;
use std::process::Command;

fn get_bin_path() -> &'static str {
    env!("CARGO_BIN_EXE_inferprobe")
}

#[test]
fn test_cli_wave2_ingest_and_report_flow() {
    let tmp = std::env::temp_dir();
    let ndjson_path = tmp.join("wave2_test_spans.ndjson");
    let trace_path = tmp.join("wave2_test_trace.json");
    let report_path = tmp.join("wave2_test_report.html");

    let ndjson_content = "{\"name\":\"conv1\",\"stage\":\"backbone\",\"start_us\":0,\"duration_us\":500,\"device\":\"gpu:0\",\"rss_kb\":32000}\n{\"name\":\"linear\",\"stage\":\"head\",\"start_us\":500,\"duration_us\":200,\"device\":\"gpu:0\",\"rss_kb\":32100}\n";
    fs::write(&ndjson_path, ndjson_content).expect("write ndjson");

    let bin = get_bin_path();

    // 1. Ingest NDJSON to trace JSON
    let status_ingest = Command::new(bin)
        .args([
            "ingest",
            ndjson_path.to_str().unwrap(),
            "--out",
            trace_path.to_str().unwrap(),
        ])
        .status()
        .expect("exec ingest");
    assert!(status_ingest.success());
    assert!(trace_path.exists());

    // 2. Report HTML generation from trace
    let status_report = Command::new(bin)
        .args([
            "report",
            trace_path.to_str().unwrap(),
            "--out",
            report_path.to_str().unwrap(),
        ])
        .status()
        .expect("exec report");
    assert!(status_report.success());
    assert!(report_path.exists());

    let html = fs::read_to_string(&report_path).expect("read report");
    assert!(html.contains("INFERPROBE // EXECUTION REPORT"));
    assert!(html.contains("conv1"));
    assert!(html.contains("linear"));
}

#[test]
fn test_cli_wave2_hw_and_run_flow() {
    let tmp = std::env::temp_dir();
    let run_trace = tmp.join("wave2_run_trace.json");
    let run_html = tmp.join("wave2_run_report.html");

    let bin = get_bin_path();

    // 1. Hardware snapshot
    let output_hw = Command::new(bin).arg("hw").output().expect("exec hw");
    assert!(output_hw.status.success());
    let hw_stdout = String::from_utf8_lossy(&output_hw.stdout);
    assert!(hw_stdout.contains("platform"));
    assert!(hw_stdout.contains("cpu_model"));

    // 2. Run subprocess benchmark with portable command
    let status_run = Command::new(bin)
        .args([
            "run",
            "--iterations",
            "3",
            "--out",
            run_trace.to_str().unwrap(),
            "--html",
            run_html.to_str().unwrap(),
            "--",
            "echo",
            "cli_test",
        ])
        .status()
        .expect("exec run");
    assert!(status_run.success());
    assert!(run_trace.exists());
    assert!(run_html.exists());

    let trace_content = fs::read_to_string(&run_trace).expect("read run trace");
    assert!(trace_content.contains("subprocess"));
}
