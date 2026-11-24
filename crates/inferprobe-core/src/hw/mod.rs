//! Hardware telemetry module.

pub mod collector;
pub mod model;
pub mod nvidia_smi;
#[cfg(test)]
mod tests;

pub use collector::collect_host_hardware;
pub use model::HardwareSnapshot;
pub use nvidia_smi::{parse_nvidia_smi_csv, GpuTelemetry};
