//! Helper parsing primitives for JSON strings and numbers.

use std::iter::Peekable;
use std::str::Chars;

/// Skips whitespace characters in a peekable char iterator.
pub fn skip_whitespace(chars: &mut Peekable<Chars<'_>>) {
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

/// Parses an escaped JSON string without leading/trailing quotes.
pub fn parse_json_str(chars: &mut Peekable<Chars<'_>>) -> Result<String, String> {
    if chars.next() != Some('"') {
        return Err("expected '\"'".into());
    }
    let mut s = String::new();
    let mut escaped = false;
    while let Some(c) = chars.next() {
        if escaped {
            match c {
                '"' => s.push('"'),
                '\\' => s.push('\\'),
                '/' => s.push('/'),
                'b' => s.push('\x08'),
                'f' => s.push('\x0c'),
                'n' => s.push('\n'),
                'r' => s.push('\r'),
                't' => s.push('\t'),
                other => s.push(other),
            }
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            return Ok(s);
        } else {
            s.push(c);
        }
    }
    Err("unterminated string literal".into())
}
