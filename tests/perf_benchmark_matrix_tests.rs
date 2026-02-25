const CARGO_TOML: &str = include_str!("../Cargo.toml");
const COMMAND_PATHS_BENCH: &str = include_str!("../benches/command_paths.rs");
const WRITE_OPS_BENCH: &str = include_str!("../benches/write_operations.rs");
const FORMATTERS_BENCH: &str = include_str!("../benches/formatters.rs");

#[test]
fn test_benchmark_targets_include_required_perf_suites() {
    assert!(CARGO_TOML.contains("name = \"formatters\""));
    assert!(CARGO_TOML.contains("name = \"write_operations\""));
    assert!(CARGO_TOML.contains("name = \"command_paths\""));
}

#[test]
fn test_matrix_includes_parse_bench_groups_for_all_non_auth_command_families() {
    assert!(COMMAND_PATHS_BENCH.contains("cli_parse_issue_paths"));
    assert!(COMMAND_PATHS_BENCH.contains("cli_parse_project_paths"));
    assert!(COMMAND_PATHS_BENCH.contains("cli_parse_team_paths"));
    assert!(COMMAND_PATHS_BENCH.contains("cli_parse_cycle_paths"));
}

#[test]
fn test_matrix_includes_handler_bench_groups_for_all_non_auth_command_families() {
    assert!(COMMAND_PATHS_BENCH.contains("issue_handlers_json"));
    assert!(COMMAND_PATHS_BENCH.contains("project_handlers_json"));
    assert!(COMMAND_PATHS_BENCH.contains("team_handlers_json"));
    assert!(COMMAND_PATHS_BENCH.contains("cycle_handlers_json"));
}

#[test]
fn test_matrix_includes_format_bench_groups_for_all_non_auth_command_families() {
    assert!(FORMATTERS_BENCH.contains("bench_issue_list"));
    assert!(FORMATTERS_BENCH.contains("bench_project_list"));
    assert!(FORMATTERS_BENCH.contains("bench_team_list"));
    assert!(FORMATTERS_BENCH.contains("bench_cycle_list"));
}

#[test]
fn test_matrix_includes_write_path_bench_groups() {
    assert!(WRITE_OPS_BENCH.contains("write_mutation_build"));
    assert!(WRITE_OPS_BENCH.contains("write_mutation_build_and_serialize"));
    assert!(WRITE_OPS_BENCH.contains("issue_reference_resolver"));
}
