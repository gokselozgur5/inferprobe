//! Structured pipeline execution trace and Chrome Trace exporter.

/// Individual profiled execution span.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub name: String,
    pub stage: String,
    pub start_us: u64,
    pub duration_us: u64,
    pub device: String,
    pub rss_kb: u64,
}

impl Span {
    pub fn new(
        name: impl Into<String>,
        stage: impl Into<String>,
        start_us: u64,
        duration_us: u64,
        device: impl Into<String>,
        rss_kb: u64,
    ) -> Self {
        Self {
            name: name.into(),
            stage: stage.into(),
            start_us,
            duration_us,
            device: device.into(),
            rss_kb,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            "{{\"name\":\"{}\",\"stage\":\"{}\",\"start_us\":{},\"duration_us\":{},\"device\":\"{}\",\"rss_kb\":{}}}",
            escape_json(&self.name),
            escape_json(&self.stage),
            self.start_us,
            self.duration_us,
            escape_json(&self.device),
            self.rss_kb
        )
    }
}

pub(crate) fn escape_json(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}

/// Collection of spans representing a full tracing session.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TraceSession {
    spans: Vec<Span>,
}

impl TraceSession {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    pub fn add_span(&mut self, span: Span) {
        self.spans.push(span);
    }

    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    pub fn len(&self) -> usize {
        self.spans.len()
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    pub fn to_json(&self) -> String {
        let mut out = String::from("{\"spans\":[");
        for (i, span) in self.spans.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&span.to_json());
        }
        out.push_str("]}");
        out
    }

    pub fn to_chrome_trace(&self) -> String {
        let mut out = String::from("{\"traceEvents\":[");
        for (i, span) in self.spans.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let event = format!(
                "{{\"name\":\"{}\",\"cat\":\"{}\",\"ph\":\"X\",\"ts\":{},\"dur\":{},\"pid\":1,\"tid\":1,\"args\":{{\"device\":\"{}\",\"rss_kb\":{}}}}}",
                escape_json(&span.name),
                escape_json(&span.stage),
                span.start_us,
                span.duration_us,
                escape_json(&span.device),
                span.rss_kb
            );
            out.push_str(&event);
        }
        out.push_str("]}");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_session_chrome_format() {
        let mut session = TraceSession::new();
        session.add_span(Span::new("preprocess", "input", 0, 150, "cpu", 4096));
        session.add_span(Span::new("forward", "model", 150, 850, "gpu:0", 8192));

        let json = session.to_json();
        assert!(json.contains("\"preprocess\""));
        assert!(json.contains("\"forward\""));

        let chrome = session.to_chrome_trace();
        assert!(chrome.starts_with("{\"traceEvents\":["));
        assert!(chrome.contains("\"ph\":\"X\""));
        assert!(chrome.contains("\"ts\":150"));
        assert!(chrome.contains("\"dur\":850"));
    }
}
