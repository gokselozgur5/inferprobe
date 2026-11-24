//! Hardware telemetry snapshot data model.

use crate::trace::escape_json;

/// System and accelerator hardware snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareSnapshot {
    pub platform: String,
    pub cpu_model: String,
    pub gpu_name: Option<String>,
    pub gpu_util_pct: Option<f64>,
    pub vram_used_mb: Option<u64>,
    pub vram_total_mb: Option<u64>,
    pub temp_c: Option<f64>,
    pub power_w: Option<f64>,
}

impl HardwareSnapshot {
    /// Creates a new hardware snapshot with specified fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        platform: impl Into<String>,
        cpu_model: impl Into<String>,
        gpu_name: Option<String>,
        gpu_util_pct: Option<f64>,
        vram_used_mb: Option<u64>,
        vram_total_mb: Option<u64>,
        temp_c: Option<f64>,
        power_w: Option<f64>,
    ) -> Self {
        Self {
            platform: platform.into(),
            cpu_model: cpu_model.into(),
            gpu_name,
            gpu_util_pct,
            vram_used_mb,
            vram_total_mb,
            temp_c,
            power_w,
        }
    }

    /// Serializes snapshot to JSON string.
    pub fn to_json(&self) -> String {
        format!(
            "{{\"platform\":\"{}\",\"cpu_model\":\"{}\",\"gpu_name\":{},\"gpu_util_pct\":{},\"vram_used_mb\":{},\"vram_total_mb\":{},\"temp_c\":{},\"power_w\":{}}}",
            escape_json(&self.platform),
            escape_json(&self.cpu_model),
            opt_str(&self.gpu_name),
            opt_num(&self.gpu_util_pct),
            opt_num(&self.vram_used_mb),
            opt_num(&self.vram_total_mb),
            opt_num(&self.temp_c),
            opt_num(&self.power_w),
        )
    }
}

fn opt_str(opt: &Option<String>) -> String {
    match opt {
        Some(s) => format!("\"{}\"", escape_json(s)),
        None => "null".to_string(),
    }
}

fn opt_num<T: std::fmt::Display>(opt: &Option<T>) -> String {
    match opt {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}
