//! Hw subcommand implementation.

use inferprobe_core::collect_host_hardware;

/// Executes the hw subcommand.
pub fn exec_hw() {
    let hw = collect_host_hardware();
    println!("{}", hw.to_json());
}
