//! Host hardware telemetry collector.

use super::model::HardwareSnapshot;
use super::nvidia_smi::parse_nvidia_smi_csv;
use std::process::Command;

/// Collects hardware snapshot safely from host OS.
pub fn collect_host_hardware() -> HardwareSnapshot {
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);
    let cpu_model = detect_cpu_model();
    let gpu_telem = detect_nvidia_gpu();

    if let Some(gpu) = gpu_telem {
        HardwareSnapshot::new(
            platform,
            cpu_model,
            Some(gpu.name),
            gpu.util_pct,
            gpu.memory_used_mb,
            gpu.memory_total_mb,
            gpu.temp_c,
            gpu.power_w,
        )
    } else {
        HardwareSnapshot::new(platform, cpu_model, None, None, None, None, None, None)
    }
}

fn detect_cpu_model() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
        {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    format!("Generic {} CPU", std::env::consts::ARCH)
}

fn detect_nvidia_gpu() -> Option<super::nvidia_smi::GpuTelemetry> {
    let output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }
    let csv = String::from_utf8_lossy(&output.stdout);
    let gpus = parse_nvidia_smi_csv(&csv);
    gpus.into_iter().next()
}
