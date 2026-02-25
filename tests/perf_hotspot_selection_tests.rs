use linear_cli::perf::hotspot_ranking::HotspotEntry;
use linear_cli::perf::hotspot_selection::select_hotspots_for_optimization;

#[test]
fn test_select_hotspots_for_optimization_returns_top_n_entries() {
    let ranked = vec![
        HotspotEntry {
            id: "cli_parse_issue_paths/issue_update_rich".to_string(),
            estimate_ns: 32_000.0,
        },
        HotspotEntry {
            id: "issue_handlers_format_compare/create_table".to_string(),
            estimate_ns: 27_000.0,
        },
        HotspotEntry {
            id: "project_handlers_json/list".to_string(),
            estimate_ns: 800.0,
        },
    ];

    let selected = select_hotspots_for_optimization(&ranked, 2).expect("selection should work");

    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].id, "cli_parse_issue_paths/issue_update_rich");
    assert_eq!(selected[1].id, "issue_handlers_format_compare/create_table");
}

#[test]
fn test_select_hotspots_for_optimization_rejects_zero_top_n() {
    let ranked = vec![HotspotEntry {
        id: "issue_handlers_json/create".to_string(),
        estimate_ns: 840.0,
    }];

    let result = select_hotspots_for_optimization(&ranked, 0);

    assert!(result.is_err());
}
