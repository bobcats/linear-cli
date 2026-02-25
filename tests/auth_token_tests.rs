use linear_cli::auth::commands::token::handle_token;
use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::auth::UserInfo;
use linear_cli::error::CliError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock storage
struct MockStorage {
    token: Option<String>,
}

impl TokenStorage for MockStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.token.clone())
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(None)
    }

    fn store_auth(&self, _token: &str, _user_info: &UserInfo) -> Result<(), CliError> {
        Ok(())
    }

    fn delete(&self) -> Result<(), CliError> {
        Ok(())
    }
}

// Capturing IO
struct CapturingIo {
    stdout: Arc<Mutex<Vec<String>>>,
}

impl CapturingIo {
    fn new() -> Self {
        Self {
            stdout: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn stdout_lines(&self) -> Vec<String> {
        self.stdout.lock().unwrap().clone()
    }
}

impl linear_cli::io::Io for CapturingIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok(String::new())
    }

    fn print(&self, message: &str) {
        self.stdout.lock().unwrap().push(message.to_string());
    }

    fn print_error(&self, message: &str) {
        let _ = message;
    }
}

#[test]
fn test_token_prints_raw_token() {
    let mut config_values = HashMap::new();
    config_values.insert(
        "LINEAR_TOKEN".to_string(),
        "lin_api_raw_token_123".to_string(),
    );
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let result = handle_token(&config, &storage, &io);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], "lin_api_raw_token_123");
}

#[test]
fn test_token_with_no_token_returns_error() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let result = handle_token(&config, &storage, &io);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::AuthError(_)));
}

#[test]
fn test_token_output_is_pure() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "lin_api_pure_123".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let result = handle_token(&config, &storage, &io);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    // Should be exactly one line with just the token
    assert_eq!(output.len(), 1);
    assert!(!output[0].contains("Token:"));
    assert!(!output[0].contains("✓"));
}

#[test]
fn test_token_respects_env_precedence() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "env_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage {
        token: Some("keyring_token".to_string()),
    };
    let io = CapturingIo::new();

    let result = handle_token(&config, &storage, &io);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert_eq!(output[0], "env_token");
}
