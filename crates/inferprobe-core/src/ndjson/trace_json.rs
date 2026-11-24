//! JSON Trace session parser (`{"spans":[...]}`).

use crate::ndjson::parse_span_line;
use crate::trace::TraceSession;

/// Parses a trace JSON string (`{"spans":[...]}` or `{"traceEvents":[...]}`) or NDJSON spans into a `TraceSession`.
pub fn parse_trace_json_or_ndjson(content: &str) -> Result<TraceSession, String> {
    let trimmed = content.trim();
    if is_spans_json(trimmed) || is_trace_events_json(trimmed) {
        parse_trace_json_object(trimmed)
    } else {
        crate::ndjson::parse_ndjson_spans(trimmed)
    }
}

fn is_spans_json(content: &str) -> bool {
    let mut chars = content.chars().filter(|c| !c.is_whitespace());
    chars.next() == Some('{')
        && chars.by_ref().take(7).collect::<String>() == "\"spans\""
        && chars.next() == Some(':')
}

fn is_trace_events_json(content: &str) -> bool {
    let mut chars = content.chars().filter(|c| !c.is_whitespace());
    chars.next() == Some('{')
        && chars.by_ref().take(13).collect::<String>() == "\"traceEvents\""
        && chars.next() == Some(':')
}

fn parse_trace_json_object(json: &str) -> Result<TraceSession, String> {
    let open_bracket = json
        .find('[')
        .ok_or_else(|| "missing '[' in spans array".to_string())?;
    let close_bracket = json
        .rfind(']')
        .ok_or_else(|| "missing ']' in spans array".to_string())?;

    if open_bracket >= close_bracket {
        return Err("invalid brackets in spans array".into());
    }

    let inner = &json[open_bracket + 1..close_bracket];
    let mut session = TraceSession::new();

    let mut in_quote = false;
    let mut escaped = false;
    let mut depth = 0;
    let mut span_start = None;

    for (idx, b) in inner.bytes().enumerate() {
        if in_quote {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_quote = false;
            }
        } else if b == b'"' {
            in_quote = true;
        } else if b == b'{' {
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
        } else if depth == 0 && !b.is_ascii_whitespace() && b != b',' {
            return Err(format!(
                "unexpected character in spans array: '{}'",
                b as char
            ));
        }
    }

    Ok(session)
}
