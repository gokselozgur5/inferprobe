//! JSON Trace session parser (`{"spans":[...]}`).

use crate::ndjson::parse_span_line;
use crate::trace::TraceSession;

/// Parses a trace JSON string (`{"spans":[...]}`) or NDJSON spans into a `TraceSession`.
pub fn parse_trace_json_or_ndjson(content: &str) -> Result<TraceSession, String> {
    let trimmed = content.trim();
    if trimmed.starts_with("{\"spans\":") {
        parse_trace_json_object(trimmed)
    } else {
        crate::ndjson::parse_ndjson_spans(trimmed)
    }
}

fn parse_trace_json_object(json: &str) -> Result<TraceSession, String> {
    let open_bracket = json
        .find('[')
        .ok_or_else(|| "missing '[' in spans array".to_string())?;
    let close_bracket = json
        .rfind(']')
        .ok_or_else(|| "missing ']' in spans array".to_string())?;

    let inner = &json[open_bracket + 1..close_bracket];
    let mut session = TraceSession::new();

    let mut in_quote = false;
    let mut depth = 0;
    let mut span_start = None;

    for (idx, b) in inner.bytes().enumerate() {
        if b == b'"' && (idx == 0 || inner.as_bytes()[idx - 1] != b'\\') {
            in_quote = !in_quote;
        } else if !in_quote {
            if b == b'{' {
                if depth == 0 {
                    span_start = Some(idx);
                }
                depth += 1;
            } else if b == b'}' {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = span_start {
                        let span_str = &inner[start..=idx];
                        if let Some(span) = parse_span_line(span_str)? {
                            session.add_span(span);
                        }
                    }
                    span_start = None;
                }
            }
        }
    }

    Ok(session)
}
