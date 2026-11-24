# inferprobe

Deterministic Edge AI, GPU & Pipeline Profiler with Latency Percentiles, Memory Telemetry, Chrome Trace Export, and CI Regression Gates in pure Rust.

## Features

- **Zero External Dependencies**: Pure Rust `std` implementation for instant builds and auditable profiling binaries.
- **Deterministic Latency Percentiles**: High-resolution latency summary (`p50`, `p90`, `p95`, `p99`, `p999`, jitter, stddev, ops/sec).
- **Structured Tracing**: Pipeline stage spans with Chrome Trace Event format export (`chrome://tracing` or Perfetto).
- **CI Regression Gate**: Automated pass/fail policy evaluation comparing baseline and current run metrics.
- **Wave 2 Telemetry & Reporting**: Zero-dep NDJSON span ingestion, HTML/SVG flamegraphs & latency dashboards, host/GPU telemetry, and subprocess benchmarking.
- **CLI Suite**: Easy-to-use subcommands: `bench`, `compare`, `inspect`, `report`, `ingest`, `run`, and `hw`.

## Workspace Crates

- `crates/inferprobe-core`: Core statistical algorithms, trace structures, telemetry samplers, and regression policies.
- `crates/inferprobe-cli`: Command-line executable frontend.

## Quickstart

```bash
# Run benchmark and save summary and trace
cargo run -p inferprobe-cli -- bench --iterations 50 --out baseline.json --chrome-trace trace.json

# Ingest raw NDJSON spans into trace JSON
cargo run -p inferprobe-cli -- ingest spans.ndjson --out trace.json

# Generate self-contained HTML/SVG flamegraph report
cargo run -p inferprobe-cli -- report trace.json --out report.html

# Run subprocess benchmark repeatedly and capture trace + HTML dashboard
cargo run -p inferprobe-cli -- run --iterations 10 --out run_trace.json --html run_report.html -- echo "infer"

# Print host and GPU hardware snapshot
cargo run -p inferprobe-cli -- hw

# Inspect trace or metrics
cargo run -p inferprobe-cli -- inspect baseline.json

# Evaluate regression between runs
cargo run -p inferprobe-cli -- compare baseline.json current.json --max-p99-pct 10.0 --max-mean-pct 5.0
```

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
