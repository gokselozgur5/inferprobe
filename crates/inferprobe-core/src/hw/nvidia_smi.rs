//! Nvidia SMI CSV parser.

/// Single GPU telemetry snapshot parsed from nvidia-smi CSV.
#[derive(Debug, Clone, PartialEq)]
pub struct GpuTelemetry {
    pub name: String,
    pub util_pct: Option<f64>,
    pub memory_used_mb: Option<u64>,
    pub memory_total_mb: Option<u64>,
    pub temp_c: Option<f64>,
    pub power_w: Option<f64>,
}

/// Parses output of `nvidia-smi --query-gpu=... --format=csv,noheader,nounits`.
pub fn parse_nvidia_smi_csv(csv: &str) -> Vec<GpuTelemetry> {
    let mut gpus = Vec::new();
    for line in csv.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let cols: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
        if cols.is_empty() || cols[0].is_empty() {
            continue;
        }
        let name = cols[0].to_string();
        let util_pct = cols.get(1).and_then(|v| parse_f64_clean(v));
        let memory_used_mb = cols.get(2).and_then(|v| parse_u64_clean(v));
        let memory_total_mb = cols.get(3).and_then(|v| parse_u64_clean(v));
        let temp_c = cols.get(4).and_then(|v| parse_f64_clean(v));
        let power_w = cols.get(5).and_then(|v| parse_f64_clean(v));

        gpus.push(GpuTelemetry {
            name,
            util_pct,
            memory_used_mb,
            memory_total_mb,
            temp_c,
            power_w,
        });
    }
    gpus
}

fn parse_f64_clean(s: &str) -> Option<f64> {
    if s.is_empty() || s.contains("[N/A]") || s == "N/A" {
        return None;
    }
    s.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_u64_clean(s: &str) -> Option<u64> {
    if s.is_empty() || s.contains("[N/A]") || s == "N/A" {
        return None;
    }
    s.parse::<u64>().ok()
}
