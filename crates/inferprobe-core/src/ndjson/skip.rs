//! Numeric and compound JSON value skipping helpers.

use super::parser_helpers::{parse_json_str, skip_whitespace};
use std::iter::Peekable;
use std::str::Chars;

/// Parses an unsigned 64-bit integer from JSON characters.
pub fn parse_json_u64(chars: &mut Peekable<Chars<'_>>) -> Result<u64, String> {
    skip_whitespace(chars);
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }
    if num_str.is_empty() {
        return Err("expected number".into());
    }
    num_str
        .parse::<u64>()
        .map_err(|e| format!("invalid number '{}': {}", num_str, e))
}

/// Skips an arbitrary JSON value (string, object, array, or scalar).
pub fn skip_value(chars: &mut Peekable<Chars<'_>>) -> Result<(), String> {
    skip_whitespace(chars);
    match chars.peek() {
        Some('"') => {
            parse_json_str(chars)?;
        }
        Some('{') => skip_nested(chars, '{', '}')?,
        Some('[') => skip_nested(chars, '[', ']')?,
        _ => skip_scalar(chars),
    }
    Ok(())
}

fn skip_nested(chars: &mut Peekable<Chars<'_>>, open: char, close: char) -> Result<(), String> {
    chars.next();
    let mut depth = 1;
    while depth > 0 {
        match chars.next() {
            Some(c) if c == open => depth += 1,
            Some(c) if c == close => depth -= 1,
            Some('"') => {
                parse_json_str(chars)?;
            }
            None => return Err(format!("unterminated container '{}'", open)),
            _ => {}
        }
    }
    Ok(())
}

fn skip_scalar(chars: &mut Peekable<Chars<'_>>) {
    while let Some(&c) = chars.peek() {
        if c == ',' || c == '}' || c == ']' || c.is_whitespace() {
            break;
        }
        chars.next();
    }
}
