//! CLI command enumeration definitions.

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    Bench {
        iterations: usize,
        out_file: Option<String>,
        chrome_trace_file: Option<String>,
    },
    Compare {
        baseline_file: String,
        current_file: String,
        max_p99_pct: Option<f64>,
        max_mean_pct: Option<f64>,
    },
    Inspect {
        file: String,
    },
    Report {
        trace_file: String,
        out_file: String,
    },
    Ingest {
        ndjson_file: String,
        out_file: String,
    },
    Run {
        cmd: String,
        args: Vec<String>,
        iterations: usize,
        out_file: Option<String>,
        html_file: Option<String>,
    },
    Hw,
    Help,
}
