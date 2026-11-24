//! Unit tests for NDJSON span ingestion.

use crate::ndjson::parse_ndjson_spans;

#[test]
fn test_parse_valid_ndjson_spans() {
    let input = "{\"name\":\"matmul\",\"stage\":\"infer\",\"start_us\":100,\"duration_us\":420,\"device\":\"gpu0\",\"rss_kb\":64000}\n\n{\"name\":\"relu\",\"stage\":\"infer\",\"start_us\":520,\"duration_us\":50,\"device\":\"gpu0\",\"rss_kb\":64100}\n";
    let session = parse_ndjson_spans(input).expect("parsed successfully");
    assert_eq!(session.len(), 2);
    let spans = session.spans();
    assert_eq!(spans[0].name, "matmul");
    assert_eq!(spans[0].duration_us, 420);
    assert_eq!(spans[0].rss_kb, 64000);
    assert_eq!(spans[1].name, "relu");
    assert_eq!(spans[1].start_us, 520);
}

#[test]
fn test_parse_escaped_strings_and_empty() {
    let input = "{\"name\":\"op\\\"quote\\\"\",\"stage\":\"prep\\nline\",\"start_us\":1,\"duration_us\":2}\n";
    let session = parse_ndjson_spans(input).expect("parsed escaped");
    assert_eq!(session.len(), 1);
    let span = &session.spans()[0];
    assert_eq!(span.name, "op\"quote\"");
    assert_eq!(span.stage, "prep\nline");
    assert_eq!(span.device, "cpu");
    assert_eq!(span.rss_kb, 0);
}

#[test]
fn test_parse_missing_fields_and_malformed() {
    let input = "{\"stage\":\"infer\"}";
    assert!(parse_ndjson_spans(input).is_err());

    let invalid = "not a json";
    assert!(parse_ndjson_spans(invalid).is_err());
}

#[test]
fn test_parse_trace_json_bracket_and_backslash_and_whitespace_defects() {
    let bad_bracket = "{\"spans\":][}";
    assert!(crate::ndjson::parse_trace_json_or_ndjson(bad_bracket).is_err());

    let win_path = "{\"spans\":[{\"name\":\"path C:\\\\\",\"stage\":\"infer\",\"start_us\":10,\"duration_us\":20}]}";
    let session =
        crate::ndjson::parse_trace_json_or_ndjson(win_path).expect("parse win path");
    assert_eq!(session.len(), 1);
    assert_eq!(session.spans()[0].name, "path C:\\");

    let formatted = "{\n  \"spans\" : [\n    {\"name\":\"op1\",\"stage\":\"infer\",\"start_us\":1,\"duration_us\":2}\n  ]\n}";
    let session =
        crate::ndjson::parse_trace_json_or_ndjson(formatted).expect("parse formatted json");
    assert_eq!(session.len(), 1);

    let bad_inner = "{\"spans\":[123, true, \"hello\"]}";
    assert!(crate::ndjson::parse_trace_json_or_ndjson(bad_inner).is_err());
}

#[test]
fn test_parse_chrome_trace_events() {
    let chrome_trace = r#"{"traceEvents":[{"name":"matmul","cat":"infer","ph":"X","ts":100,"dur":420,"pid":1,"tid":1,"args":{"device":"gpu0","rss_kb":64000}}]}"#;
    let session =
        crate::ndjson::parse_trace_json_or_ndjson(chrome_trace).expect("parse chrome trace");
    assert_eq!(session.len(), 1);
    let span = &session.spans()[0];
    assert_eq!(span.name, "matmul");
    assert_eq!(span.stage, "infer");
    assert_eq!(span.start_us, 100);
    assert_eq!(span.duration_us, 420);
    assert_eq!(span.device, "gpu0");
    assert_eq!(span.rss_kb, 64000);
}
