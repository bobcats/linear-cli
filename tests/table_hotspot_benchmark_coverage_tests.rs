const COMMAND_PATHS_BENCH: &str = include_str!("../benches/command_paths.rs");
const FORMATTERS_BENCH: &str = include_str!("../benches/formatters.rs");

#[test]
fn test_formatters_bench_includes_large_table_hotspot_scenarios() {
    assert!(
        FORMATTERS_BENCH.contains("table_hotspot_large_rows"),
        "expected a large-row table hotspot benchmark scenario"
    );
    assert!(
        FORMATTERS_BENCH.contains("table_hotspot_wide_cells"),
        "expected a wide-cell table hotspot benchmark scenario"
    );
}

#[test]
fn test_command_paths_bench_includes_table_format_compare_for_all_non_auth_families() {
    assert!(
        COMMAND_PATHS_BENCH.contains("issue_handlers_format_compare"),
        "issue handler table format compare group should exist"
    );
    assert!(
        COMMAND_PATHS_BENCH.contains("project_handlers_format_compare"),
        "project handler table format compare group should exist"
    );
    assert!(
        COMMAND_PATHS_BENCH.contains("team_handlers_format_compare"),
        "team handler table format compare group should exist"
    );
    assert!(
        COMMAND_PATHS_BENCH.contains("cycle_handlers_format_compare"),
        "cycle handler table format compare group should exist"
    );
}
