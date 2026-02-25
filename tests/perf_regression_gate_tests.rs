use linear_cli::perf::regression::{BenchmarkSample, evaluate_regression_against_targets};
use linear_cli::perf::targets::{LatencyTarget, LatencyTargetsDocument};

fn targets_doc() -> LatencyTargetsDocument {
    LatencyTargetsDocument {
        generated_from: "docs/perf/baseline-hotspots.md".to_string(),
        generated_at: "2026-02-24T21:20:00Z".to_string(),
        targets: vec![
            LatencyTarget {
                benchmark_id: "project_handlers_json/list".to_string(),
                baseline_ns: 1335.0,
                target_ns: 1250.0,
                tolerance_percent: 5.0,
            },
            LatencyTarget {
                benchmark_id: "project_handlers_json/view".to_string(),
                baseline_ns: 1322.0,
                target_ns: 1240.0,
                tolerance_percent: 5.0,
            },
        ],
    }
}

#[test]
fn test_regression_evaluation_passes_within_tolerance() {
    let targets = targets_doc();
    let samples = vec![
        BenchmarkSample {
            benchmark_id: "project_handlers_json/list".to_string(),
            estimate_ns: 1300.0,
        },
        BenchmarkSample {
            benchmark_id: "project_handlers_json/view".to_string(),
            estimate_ns: 1290.0,
        },
    ];

    let result =
        evaluate_regression_against_targets(&targets, &samples).expect("evaluation should succeed");

    assert!(result.passed);
    assert!(result.failures.is_empty());
}

#[test]
fn test_regression_evaluation_fails_when_above_tolerance() {
    let targets = targets_doc();
    let samples = vec![
        BenchmarkSample {
            benchmark_id: "project_handlers_json/list".to_string(),
            estimate_ns: 1500.0,
        },
        BenchmarkSample {
            benchmark_id: "project_handlers_json/view".to_string(),
            estimate_ns: 1290.0,
        },
    ];

    let result =
        evaluate_regression_against_targets(&targets, &samples).expect("evaluation should succeed");

    assert!(!result.passed);
    assert_eq!(result.failures.len(), 1);
    assert_eq!(
        result.failures[0].benchmark_id,
        "project_handlers_json/list"
    );
}

#[test]
fn test_regression_evaluation_fails_on_missing_sample() {
    let targets = targets_doc();
    let samples = vec![BenchmarkSample {
        benchmark_id: "project_handlers_json/list".to_string(),
        estimate_ns: 1300.0,
    }];

    let result =
        evaluate_regression_against_targets(&targets, &samples).expect("evaluation should succeed");

    assert!(!result.passed);
    assert_eq!(result.failures.len(), 1);
    assert_eq!(
        result.failures[0].benchmark_id,
        "project_handlers_json/view"
    );
}
