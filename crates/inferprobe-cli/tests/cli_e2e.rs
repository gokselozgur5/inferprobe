use std::process::Command;

#[test]
fn test_cli_bench_compare_flow() {
    let bin = env!("CARGO_BIN_EXE_inferprobe");
    let tmp_dir = std::env::temp_dir();
    let base_out = tmp_dir.join("inferprobe_test_base.json");
    let curr_out = tmp_dir.join("inferprobe_test_curr.json");
    let trace_out = tmp_dir.join("inferprobe_test_trace.json");

    let status = Command::new(bin)
        .args([
            "bench",
            "--iterations",
            "10",
            "--out",
            base_out.to_str().unwrap(),
            "--chrome-trace",
            trace_out.to_str().unwrap(),
        ])
        .status()
        .expect("bench baseline");
    assert!(status.success());
    assert!(base_out.exists());
    assert!(trace_out.exists());

    let status = Command::new(bin)
        .args([
            "bench",
            "--iterations",
            "10",
            "--out",
            curr_out.to_str().unwrap(),
        ])
        .status()
        .expect("bench current");
    assert!(status.success());

    let status = Command::new(bin)
        .args([
            "compare",
            base_out.to_str().unwrap(),
            curr_out.to_str().unwrap(),
            "--max-p99-pct",
            "1000.0",
            "--max-mean-pct",
            "1000.0",
        ])
        .status()
        .expect("compare ok");
    assert!(status.success());

    let status = Command::new(bin)
        .args(["inspect", base_out.to_str().unwrap()])
        .status()
        .expect("inspect");
    assert!(status.success());
}
