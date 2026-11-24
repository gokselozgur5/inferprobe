//! NDJSON multi-line ingestion module.

pub mod parser_helpers;
pub mod skip;
pub mod span_parser;
#[cfg(test)]
mod tests;
pub mod trace_json;

pub use parser_helpers::skip_whitespace;
pub use span_parser::parse_span_line;
pub use trace_json::parse_trace_json_or_ndjson;

use crate::trace::TraceSession;

/// Parses multi-line NDJSON into a `TraceSession`.
pub fn parse_ndjson_spans(input: &str) -> Result<TraceSession, String> {
    let mut session = TraceSession::new();
    for (line_idx, line) in input.lines().enumerate() {
        match parse_span_line(line) {
            Ok(Some(span)) => session.add_span(span),
            Ok(None) => {}
            Err(e) => return Err(format!("line {}: {}", line_idx + 1, e)),
        }
    }
    Ok(session)
}
