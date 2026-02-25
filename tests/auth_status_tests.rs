use linear_cli::auth::commands::status::handle_status;
use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::auth::{MockAuthClient, UserInfo};
use linear_cli::error::CliError;
use linear_cli::output::OutputFormat;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Stored auth data for testing
#[derive(Clone)]
struct StoredAuth {
    token: Option<String>,
    user_info: Option<UserInfo>,
}

// Mock storage for testing
struct MockStorage {
    data: Arc<Mutex<StoredAuth>>,
}

impl MockStorage {
    fn new(token: Option<String>) -> Self {
        Self {
            data: Arc::new(Mutex::new(StoredAuth {
                token,
                user_info: None,
            })),
        }
    }
}

impl TokenStorage for MockStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.data.lock().unwrap().token.clone())
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(self.data.lock().unwrap().user_info.clone())
    }

    fn store_auth(&self, token: &str, user_info: &UserInfo) -> Result<(), CliError> {
        // Single lock ensures atomicity - both fields updated together
        let mut data = self.data.lock().unwrap();
        data.token = Some(token.to_string());
        data.user_info = Some(user_info.clone());
        Ok(())
    }

    fn delete(&self) -> Result<(), CliError> {
        // Single lock ensures atomicity - both fields deleted together
        let mut data = self.data.lock().unwrap();
        data.token = None;
        data.user_info = None;
        Ok(())
    }
}

// Capturing IO to verify output
struct CapturingIo {
    stdout: Arc<Mutex<Vec<String>>>,
    stderr: Arc<Mutex<Vec<String>>>,
}

impl CapturingIo {
    fn new() -> Self {
        Self {
            stdout: Arc::new(Mutex::new(Vec::new())),
            stderr: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn stdout_lines(&self) -> Vec<String> {
        self.stdout.lock().unwrap().clone()
    }

    fn _stderr_lines(&self) -> Vec<String> {
        self.stderr.lock().unwrap().clone()
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
        self.stderr.lock().unwrap().push(message.to_string());
    }
}

#[test]
fn test_status_with_valid_token_shows_user_info() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "lin_api_test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage::new(None);
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let io = CapturingIo::new();

    let result = handle_status(&config, &storage, &api_client, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert!(output.iter().any(|line| line.contains("Alice")));
    assert!(output.iter().any(|line| line.contains("alice@example.com")));
}

#[test]
fn test_status_shows_token_source_env() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "lin_api_test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage::new(None);
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let io = CapturingIo::new();

    let result = handle_status(
        &config,
        &storage,
        &api_client,
        &io,
        Some(OutputFormat::Table),
    );

    assert!(result.is_ok());
    let output = io.stdout_lines();
    // Check for "environment variable (LINEAR_TOKEN)" which is the full string in table format
    assert!(output.iter().any(|line| line.contains("LINEAR_TOKEN")));
}

#[test]
fn test_status_shows_token_source_keyring() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage::new(Some("lin_api_keyring_token".to_string()));
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        }),
    };
    let io = CapturingIo::new();

    let result = handle_status(&config, &storage, &api_client, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert!(output.iter().any(|line| line.contains("keyring")));
}

#[test]
fn test_status_redacts_token() {
    let mut config_values = HashMap::new();
    config_values.insert(
        "LINEAR_TOKEN".to_string(),
        "lin_api_secret_token".to_string(),
    );
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage::new(None);
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let io = CapturingIo::new();

    let result = handle_status(&config, &storage, &api_client, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    // Should show redacted version
    assert!(output.iter().any(|line| line.contains("lin_api_se***")));
    // Should NOT show full token
    assert!(
        !output
            .iter()
            .any(|line| line.contains("lin_api_secret_token"))
    );
}

#[test]
fn test_status_with_no_token_shows_error() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage::new(None);
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let io = CapturingIo::new();

    let result = handle_status(&config, &storage, &api_client, &io, None);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::AuthError(_)));
}

#[test]
fn test_status_with_invalid_token_shows_error() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "invalid_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage::new(None);
    let api_client = MockAuthClient {
        result: Err(CliError::AuthError("Invalid token".to_string())),
    };
    let io = CapturingIo::new();

    let result = handle_status(&config, &storage, &api_client, &io, None);

    assert!(result.is_err());
}
