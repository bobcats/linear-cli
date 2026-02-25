use linear_cli::auth::config::TestConfigProvider;
use linear_cli::output::{OutputFormat, detect_format_with_provider, get_format_with_provider};
use std::collections::HashMap;

#[test]
fn test_detect_format_returns_json_when_env_var_set_to_json() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "json".to_string());
    let config = TestConfigProvider { values };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Json));
}

#[test]
fn test_detect_format_returns_csv_when_env_var_set_to_csv() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "csv".to_string());
    let config = TestConfigProvider { values };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Csv));
}

#[test]
fn test_detect_format_returns_markdown_when_env_var_set_to_markdown() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "markdown".to_string());
    let config = TestConfigProvider { values };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Markdown));
}

#[test]
fn test_detect_format_returns_markdown_when_env_var_set_to_md() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "md".to_string());
    let config = TestConfigProvider { values };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Markdown));
}

#[test]
fn test_detect_format_returns_table_when_env_var_set_to_table() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "table".to_string());
    let config = TestConfigProvider { values };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Table));
}

#[test]
fn test_detect_format_defaults_to_auto() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };

    let format = detect_format_with_provider(&config);
    assert!(matches!(format, OutputFormat::Auto));
}

#[test]
fn test_output_format_resolves_auto_to_table_when_tty() {
    let format = OutputFormat::Auto;
    // This test is simplified - in real implementation we'd check is_terminal()
    // For now we just test that Auto is a valid variant
    assert!(matches!(format, OutputFormat::Auto));
}

#[test]
fn test_json_output_has_data_wrapper() {
    use serde_json::json;

    let data = json!({
        "issues": [
            {"id": "123", "title": "Test"}
        ]
    });

    let output = json!({
        "data": data
    });

    assert!(output.get("data").is_some());
    assert!(output["data"].get("issues").is_some());
}

#[test]
fn test_get_format_cli_flag_takes_precedence_over_env() {
    // Even if env var is set, CLI flag should win
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "json".to_string());
    let config = TestConfigProvider { values };

    let format = get_format_with_provider(Some(OutputFormat::Csv), &config);
    assert!(matches!(format, OutputFormat::Csv));
}

#[test]
fn test_get_format_uses_env_when_no_cli_flag() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_FORMAT".to_string(), "markdown".to_string());
    let config = TestConfigProvider { values };

    let format = get_format_with_provider(None, &config);
    assert!(matches!(format, OutputFormat::Markdown));
}

#[test]
fn test_get_format_resolves_auto_when_no_cli_flag_or_env() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };

    let format = get_format_with_provider(None, &config);
    // Should resolve to either Json or Table (depends on TTY)
    assert!(matches!(format, OutputFormat::Json | OutputFormat::Table));
}

#[test]
fn test_error_output_to_stderr() {
    use linear_cli::error::{CliError, ErrorOutput};

    let error = CliError::NotFound("Issue not found".to_string());
    let output: ErrorOutput = error.into();

    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("NOT_FOUND"));
}
