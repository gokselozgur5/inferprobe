//! Subprocess profiling module.

pub mod runner;
#[cfg(test)]
mod tests;

pub use runner::run_subprocess_benchmark;
