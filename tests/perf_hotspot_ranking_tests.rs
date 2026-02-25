use linear_cli::perf::hotspot_ranking::{HotspotEntry, rank_hotspots_from_criterion_index};

#[test]
fn test_rank_hotspots_sorts_by_median_latency_descending() {
    let criterion_index = r#"
    {
      "benchmarks": [
        {
          "id": "issue_handlers_json/create",
          "typical": { "estimate": 840.0, "unit": "ns" }
        },
        {
          "id": "project_handlers_json/view",
          "typical": { "estimate": 718.0, "unit": "ns" }
        },
        {
          "id": "cli_parse_issue_paths/issue_update_rich",
          "typical": { "estimate": 31593.0, "unit": "ns" }
        }
      ]
    }
    "#;

    let ranked = rank_hotspots_from_criterion_index(criterion_index).expect("should parse ranking");

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0].id, "cli_parse_issue_paths/issue_update_rich");
    assert_eq!(ranked[1].id, "issue_handlers_json/create");
    assert_eq!(ranked[2].id, "project_handlers_json/view");
}

#[test]
fn test_rank_hotspots_is_deterministic_for_equal_estimates() {
    let criterion_index = r#"
    {
      "benchmarks": [
        {
          "id": "cycle_handlers_json/current",
          "typical": { "estimate": 500.0, "unit": "ns" }
        },
        {
          "id": "cycle_handlers_json/list",
          "typical": { "estimate": 500.0, "unit": "ns" }
        }
      ]
    }
    "#;

    let ranked = rank_hotspots_from_criterion_index(criterion_index).expect("should parse ranking");

    assert_eq!(
        ranked,
        vec![
            HotspotEntry {
                id: "cycle_handlers_json/current".to_string(),
                estimate_ns: 500.0,
            },
            HotspotEntry {
                id: "cycle_handlers_json/list".to_string(),
                estimate_ns: 500.0,
            },
        ]
    );
}
