use inferprobe_core::{
    evaluate_regression, run_synthetic_pipeline, PipelineStageSpec, RegressionPolicy,
};

#[test]
fn test_end_to_end_synthetic_benchmark_and_gate() {
    let stages = vec![
        PipelineStageSpec::new("preprocess", 500, "cpu"),
        PipelineStageSpec::new("inference", 2_000, "npu:0"),
        PipelineStageSpec::new("postprocess", 300, "cpu"),
    ];

    let (baseline_hist, _) = run_synthetic_pipeline(&stages, 20);
    let baseline_summary = baseline_hist.summarize(None).expect("baseline summary");

    let (current_hist, current_trace) = run_synthetic_pipeline(&stages, 20);
    let current_summary = current_hist.summarize(None).expect("current summary");

    assert_eq!(current_trace.len(), 60);

    let policy = RegressionPolicy {
        max_p99_regression_pct: 1000.0,
        max_mean_regression_pct: 1000.0,
        max_rss_regression_pct: 1000.0,
    };

    let report = evaluate_regression(&baseline_summary, &current_summary, 2048, 2048, &policy);
    assert!(report.passed);
}
