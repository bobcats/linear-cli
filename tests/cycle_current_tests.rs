use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::MockTokenStorage;
use linear_cli::client::cycles::CycleClient;
use linear_cli::cycles::commands::handle_current;
use linear_cli::cycles::types::Cycle;
use linear_cli::error::CliError;
use linear_cli::io::MockIo;
use linear_cli::output::OutputFormat;
use std::collections::HashMap;

/// Mock client for cycle current tests
struct MockCycleClient {
    cycles: Vec<Cycle>,
}

impl CycleClient for MockCycleClient {
    fn get_cycle(&self, _token: &str, _id: &str) -> Result<Cycle, CliError> {
        unimplemented!("Not needed for current tests")
    }

    fn list_cycles(&self, _token: &str, _limit: usize) -> Result<Vec<Cycle>, CliError> {
        Ok(self.cycles.clone())
    }
}

/// Helper to create a test cycle
fn create_test_cycle(id: usize, is_active: bool) -> Cycle {
    Cycle {
        id: format!("cycle-{}", id),
        name: format!("Sprint {}", id),
        number: id as f64,
        description: Some(format!("Sprint {} description", id)),
        starts_at: "2024-01-15T00:00:00Z".to_string(),
        ends_at: "2024-01-29T23:59:59Z".to_string(),
        created_at: "2024-01-10T10:00:00Z".to_string(),
        completed_at: if is_active {
            None
        } else {
            Some("2024-01-29T18:00:00Z".to_string())
        },
        progress: 0.75,
        is_active,
        is_future: false,
        is_next: false,
        is_past: !is_active,
        is_previous: false,
        team_name: "Engineering".to_string(),
        team_key: "ENG".to_string(),
    }
}

#[test]
fn test_current_finds_active_cycle() {
    // Setup: Three cycles, middle one is active
    let cycles = vec![
        create_test_cycle(1, false),
        create_test_cycle(2, true), // This is active
        create_test_cycle(3, false),
    ];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Json));

    // Assert: Success
    assert!(result.is_ok());

    // Assert: Output contains the active cycle (Sprint 2)
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);
    assert!(output[0].contains("Sprint 2"));
    assert!(output[0].contains("\"is_active\":true"));
}

#[test]
fn test_current_returns_error_when_no_active_cycle() {
    // Setup: All cycles are inactive
    let cycles = vec![
        create_test_cycle(1, false),
        create_test_cycle(2, false),
        create_test_cycle(3, false),
    ];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute
    let result = handle_current(&client, &config, &storage, &io, None);

    // Assert: Returns NotFound error
    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => {
            assert_eq!(msg, "No active cycle found");
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[test]
fn test_current_with_empty_cycle_list() {
    // Setup: No cycles at all
    let cycles = vec![];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute
    let result = handle_current(&client, &config, &storage, &io, None);

    // Assert: Returns NotFound error
    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => {
            assert_eq!(msg, "No active cycle found");
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[test]
fn test_current_json_format() {
    // Setup: One active cycle
    let cycles = vec![create_test_cycle(5, true)];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute with JSON format
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Json));

    // Assert: Success and valid JSON
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&output[0]).unwrap();
    assert_eq!(json["id"], "cycle-5");
    assert_eq!(json["name"], "Sprint 5");
    assert_eq!(json["is_active"], true);
}

#[test]
fn test_current_csv_format() {
    // Setup: One active cycle
    let cycles = vec![create_test_cycle(10, true)];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute with CSV format
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Csv));

    // Assert: Success and valid CSV
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);

    // Verify CSV structure (header + data row)
    let lines: Vec<&str> = output[0].lines().collect();
    assert_eq!(lines.len(), 2); // Header + 1 data row

    // Check header
    assert!(lines[0].contains("id"));
    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("number"));

    // Check data
    assert!(lines[1].contains("cycle-10"));
    assert!(lines[1].contains("Sprint 10"));
}

#[test]
fn test_current_markdown_format() {
    // Setup: One active cycle
    let cycles = vec![create_test_cycle(7, true)];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute with Markdown format
    let result = handle_current(
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Markdown),
    );

    // Assert: Success and valid markdown
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);

    // Verify markdown structure (card format for single item)
    assert!(output[0].contains("# Sprint 7 (Cycle #7)"));
    assert!(output[0].contains("**Status:**"));
    assert!(output[0].contains("**Progress:**"));
    assert!(output[0].contains("**Team:**"));
}

#[test]
fn test_current_table_format() {
    // Setup: One active cycle
    let cycles = vec![create_test_cycle(8, true)];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute with Table format
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Table));

    // Assert: Success and valid table
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);

    // Verify table structure (vertical layout for single item)
    assert!(output[0].contains("Name"));
    assert!(output[0].contains("Sprint 8"));
    assert!(output[0].contains("Number"));
    assert!(output[0].contains("#8"));
    assert!(output[0].contains("Status"));
    assert!(output[0].contains("Progress"));
}

#[test]
fn test_current_respects_format_precedence() {
    // Setup: One active cycle, config with LINEAR_CLI_FORMAT set
    let cycles = vec![create_test_cycle(9, true)];

    let client = MockCycleClient { cycles };
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_CLI_FORMAT".to_string(), "csv".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute with explicit JSON flag (should override env var)
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Json));

    // Assert: Success and JSON format (flag overrides env var)
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);

    // Verify it's JSON, not CSV
    assert!(output[0].trim().starts_with('{'));
    let json: serde_json::Value = serde_json::from_str(&output[0]).unwrap();
    assert_eq!(json["id"], "cycle-9");
}

#[test]
fn test_current_handles_multiple_active_cycles_gracefully() {
    // Setup: Multiple cycles marked as active (edge case, shouldn't happen in practice)
    // Should return the first one found
    let cycles = vec![
        create_test_cycle(1, false),
        create_test_cycle(2, true), // First active
        create_test_cycle(3, true), // Second active (shouldn't happen)
    ];

    let client = MockCycleClient { cycles };
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("test-token".to_string());
    let io = MockIo::new();

    // Execute
    let result = handle_current(&client, &config, &storage, &io, Some(OutputFormat::Json));

    // Assert: Success, returns first active cycle found
    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);
    assert!(output[0].contains("Sprint 2")); // First active one
}
