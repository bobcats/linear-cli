use linear_cli::perf::targets::{
    LatencyTarget, LatencyTargetsDocument, parse_latency_targets_json, validate_latency_targets,
};

fn valid_targets_document() -> LatencyTargetsDocument {
    LatencyTargetsDocument {
        generated_from: "docs/perf/baseline-hotspots.md".to_string(),
        generated_at: "2026-02-24T21:00:00Z".to_string(),
        targets: vec![
            LatencyTarget {
                benchmark_id: "project_handlers_json/list".to_string(),
                baseline_ns: 1330.0,
                target_ns: 1200.0,
                tolerance_percent: 5.0,
            },
            LatencyTarget {
                benchmark_id: "project_handlers_json/view".to_string(),
                baseline_ns: 1320.0,
                target_ns: 1210.0,
                tolerance_percent: 5.0,
            },
        ],
    }
}

#[test]
fn test_validate_latency_targets_accepts_well_formed_document() {
    let doc = valid_targets_document();

    let result = validate_latency_targets(&doc);

    assert!(result.is_ok());
}

#[test]
fn test_validate_latency_targets_rejects_target_above_baseline() {
    let mut doc = valid_targets_document();
    doc.targets[0].target_ns = 1400.0;

    let result = validate_latency_targets(&doc);

    assert!(result.is_err());
}

#[test]
fn test_validate_latency_targets_rejects_duplicate_benchmark_ids() {
    let mut doc = valid_targets_document();
    doc.targets[1].benchmark_id = doc.targets[0].benchmark_id.clone();

    let result = validate_latency_targets(&doc);

    assert!(result.is_err());
}

#[test]
fn test_parse_latency_targets_json_parses_valid_schema() {
    let input = r#"
    {
      "generated_from": "docs/perf/baseline-hotspots.md",
      "generated_at": "2026-02-24T21:00:00Z",
      "targets": [
        {
          "benchmark_id": "project_handlers_json/list",
          "baseline_ns": 1330.0,
          "target_ns": 1200.0,
          "tolerance_percent": 5.0
        }
      ]
    }
    "#;

    let parsed = parse_latency_targets_json(input).expect("valid target schema should parse");

    assert_eq!(parsed.targets.len(), 1);
    assert_eq!(parsed.targets[0].benchmark_id, "project_handlers_json/list");
}
