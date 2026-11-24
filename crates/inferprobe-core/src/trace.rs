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
