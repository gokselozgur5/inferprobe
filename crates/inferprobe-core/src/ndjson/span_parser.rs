//! NDJSON line parser for execution spans.

use super::parser_helpers::{parse_json_str, skip_whitespace};
use super::skip::{parse_json_u64, skip_value};
use crate::trace::Span;

/// Parses a line of NDJSON into a `Span`.
pub fn parse_span_line(line: &str) -> Result<Option<Span>, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(format!("line is not a valid JSON object: {}", trimmed));
    }
    let inner = &trimmed[1..trimmed.len() - 1];

    let mut name = None;
    let mut stage = None;
    let mut start_us = None;
    let mut duration_us = None;
    let mut device = None;
    let mut rss_kb = None;

    let mut chars = inner.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == ',' {
            chars.next();
            continue;
        }
        if c == '"' {
            let key = parse_json_str(&mut chars)?;
            skip_whitespace(&mut chars);
            if chars.next() != Some(':') {
                return Err(format!("expected ':' after key '{}'", key));
            }
            skip_whitespace(&mut chars);
            match key.as_str() {
                "name" => name = Some(parse_json_str(&mut chars)?),
                "stage" | "cat" => stage = Some(parse_json_str(&mut chars)?),
                "start_us" | "ts" => start_us = Some(parse_json_u64(&mut chars)?),
                "duration_us" | "dur" => duration_us = Some(parse_json_u64(&mut chars)?),
                "device" => device = Some(parse_json_str(&mut chars)?),
                "rss_kb" => rss_kb = Some(parse_json_u64(&mut chars)?),
                "args" => parse_args_obj(&mut chars, &mut device, &mut rss_kb)?,
                _ => skip_value(&mut chars)?,
            }
        } else {
            return Err("expected key in JSON object".into());
        }
    }

    let span = Span::new(
        name.ok_or_else(|| "missing required field 'name'".to_string())?,
        stage.unwrap_or_else(|| "default".to_string()),
        start_us.ok_or_else(|| "missing required field 'start_us'".to_string())?,
        duration_us.ok_or_else(|| "missing required field 'duration_us'".to_string())?,
        device.unwrap_or_else(|| "cpu".to_string()),
        rss_kb.unwrap_or(0),
    );

    Ok(Some(span))
}

fn parse_args_obj(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    device: &mut Option<String>,
    rss_kb: &mut Option<u64>,
) -> Result<(), String> {
    skip_whitespace(chars);
    if chars.next() != Some('{') {
        return Err("expected '{' for args object".into());
    }
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == ',' {
            chars.next();
            continue;
        }
        if c == '}' {
            chars.next();
            return Ok(());
        }
        if c == '"' {
            let key = parse_json_str(chars)?;
            skip_whitespace(chars);
            if chars.next() != Some(':') {
                return Err("expected ':' in args object".into());
            }
            skip_whitespace(chars);
            match key.as_str() {
                "device" => *device = Some(parse_json_str(chars)?),
                "rss_kb" => *rss_kb = Some(parse_json_u64(chars)?),
                _ => skip_value(chars)?,
            }
        } else {
            return Err("expected key in args object".into());
        }
    }
    Err("unterminated args object".into())
}
