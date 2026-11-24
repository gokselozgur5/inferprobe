//! CLI argument parsers for report, ingest, and hw subcommands.

use super::types::CliCommand;

/// Parses report subcommand arguments: `inferprobe report <trace.json> --out <report.html>`
pub fn parse_report(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("report requires <trace.json> and --out <report.html>".into());
    }
    let trace_file = args[0].clone();
    let mut out_file = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--out" && i + 1 < args.len() {
            out_file = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    let out_file = out_file.ok_or_else(|| "report requires --out <report.html>".to_string())?;
    Ok(CliCommand::Report {
        trace_file,
        out_file,
    })
}

/// Parses ingest subcommand arguments: `inferprobe ingest <spans.ndjson> --out <trace.json>`
pub fn parse_ingest(args: &[String]) -> Result<CliCommand, String> {
    if args.is_empty() {
        return Err("ingest requires <spans.ndjson> and --out <trace.json>".into());
    }
    let ndjson_file = args[0].clone();
    let mut out_file = None;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--out" && i + 1 < args.len() {
            out_file = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    let out_file = out_file.ok_or_else(|| "ingest requires --out <trace.json>".to_string())?;
    Ok(CliCommand::Ingest {
        ndjson_file,
        out_file,
    })
}
