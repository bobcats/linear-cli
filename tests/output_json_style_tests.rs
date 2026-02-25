use linear_cli::auth::config::TestConfigProvider;
use linear_cli::output::{JsonStyle, resolve_json_style_for_tty_with_provider};
use std::collections::HashMap;

#[test]
fn test_resolve_json_style_defaults_to_compact_when_not_tty() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };

    let style = resolve_json_style_for_tty_with_provider(&config, false);

    assert!(matches!(style, JsonStyle::Compact));
}

#[test]
fn test_resolve_json_style_defaults_to_pretty_when_tty() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };

    let style = resolve_json_style_for_tty_with_provider(&config, true);

    assert!(matches!(style, JsonStyle::Pretty));
}

#[test]
fn test_resolve_json_style_supports_pretty_override() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_JSON_STYLE".to_string(), "pretty".to_string());
    let config = TestConfigProvider { values };

    let style = resolve_json_style_for_tty_with_provider(&config, false);

    assert!(matches!(style, JsonStyle::Pretty));
}

#[test]
fn test_resolve_json_style_supports_compact_override() {
    let mut values = HashMap::new();
    values.insert("LINEAR_CLI_JSON_STYLE".to_string(), "compact".to_string());
    let config = TestConfigProvider { values };

    let style = resolve_json_style_for_tty_with_provider(&config, true);

    assert!(matches!(style, JsonStyle::Compact));
}
