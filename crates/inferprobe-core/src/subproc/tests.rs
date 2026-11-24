//! Unit tests for subprocess benchmark runner.

use crate::subproc::run_subprocess_benchmark;

#[test]
fn test_subprocess_benchmark_success() {
    let (session, code) =
        run_subprocess_benchmark("echo", &["test_probe".into()], 3).expect("runs echo");
    assert_eq!(code, 0);
    assert_eq!(session.len(), 3);
    for span in session.spans() {
        assert_eq!(span.stage, "subprocess");
        assert_eq!(span.device, "host");
    }
}

#[test]
fn test_subprocess_benchmark_failure_code() {
    let res = run_subprocess_benchmark("false", &[], 3);
    if let Ok((session, code)) = res {
        assert_ne!(code, 0);
        assert_eq!(session.len(), 1);
    }
}
