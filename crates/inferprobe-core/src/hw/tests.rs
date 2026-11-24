//! Unit tests for hardware telemetry and nvidia-smi parsing.

#[cfg(test)]
mod tests {
    use crate::hw::{collect_host_hardware, parse_nvidia_smi_csv, HardwareSnapshot};

    #[test]
    fn test_parse_nvidia_smi_csv_single_gpu() {
        let csv = "NVIDIA RTX 4090, 85, 12288, 24576, 68.0, 310.5\n";
        let gpus = parse_nvidia_smi_csv(csv);
        assert_eq!(gpus.len(), 1);
        let gpu = &gpus[0];
        assert_eq!(gpu.name, "NVIDIA RTX 4090");
        assert_eq!(gpu.util_pct, Some(85.0));
        assert_eq!(gpu.memory_used_mb, Some(12288));
        assert_eq!(gpu.memory_total_mb, Some(24576));
        assert_eq!(gpu.temp_c, Some(68.0));
        assert_eq!(gpu.power_w, Some(310.5));
    }

    #[test]
    fn test_parse_nvidia_smi_csv_multi_gpu_and_na() {
        let csv = "NVIDIA A100-SXM4-80GB, 99, 78000, 81920, 55.0, 395.0\nNVIDIA L4, [N/A], 1024, 24576, [N/A], 45.2\n";
        let gpus = parse_nvidia_smi_csv(csv);
        assert_eq!(gpus.len(), 2);
        assert_eq!(gpus[0].name, "NVIDIA A100-SXM4-80GB");
        assert_eq!(gpus[1].name, "NVIDIA L4");
        assert_eq!(gpus[1].util_pct, None);
        assert_eq!(gpus[1].memory_used_mb, Some(1024));
        assert_eq!(gpus[1].temp_c, None);
        assert_eq!(gpus[1].power_w, Some(45.2));
    }

    #[test]
    fn test_parse_nvidia_smi_csv_malformed() {
        let csv = "\n   \n,,,,\n";
        let gpus = parse_nvidia_smi_csv(csv);
        assert!(gpus.is_empty());
    }

    #[test]
    fn test_hardware_snapshot_serialization_and_collect() {
        let snap = HardwareSnapshot::new(
            "linux-x86_64",
            "AMD EPYC 7763",
            Some("NVIDIA H100".into()),
            Some(95.0),
            Some(40000),
            Some(81920),
            Some(60.0),
            Some(650.0),
        );
        let json = snap.to_json();
        assert!(json.contains("\"platform\":\"linux-x86_64\""));
        assert!(json.contains("\"gpu_name\":\"NVIDIA H100\""));
        assert!(json.contains("\"power_w\":650"));

        let host_snap = collect_host_hardware();
        assert!(!host_snap.platform.is_empty());
        assert!(!host_snap.cpu_model.is_empty());
    }
}
