//! Ingest subcommand implementation.

use std::fs;
use std::process;

use inferprobe_core::parse_ndjson_spans;

/// Executes the ingest subcommand.
pub fn exec_ingest(ndjson_path: &str, out_path: &str) {
    let content = fs::read_to_string(ndjson_path).unwrap_or_else(|e| {
        eprintln!("Error: failed to read ndjson '{}': {}", ndjson_path, e);
        process::exit(1);
    });

    let session = parse_ndjson_spans(&content).unwrap_or_else(|e| {
        eprintln!("Error: failed to parse ndjson spans: {}", e);
        process::exit(1);
    });

    let json = session.to_json();
    fs::write(out_path, json).unwrap_or_else(|e| {
        eprintln!("Error: failed to write trace '{}': {}", out_path, e);
        process::exit(1);
    });

    println!(
        "Ingested {} spans successfully: {}",
        session.len(),
        out_path
    );
}
