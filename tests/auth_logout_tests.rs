use linear_cli::auth::commands::logout::handle_logout;
use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::auth::UserInfo;
use linear_cli::error::CliError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock storage that captures deletions
struct CapturingMockStorage {
    token: Arc<Mutex<Option<String>>>,
    deleted: Arc<Mutex<bool>>,
}

impl CapturingMockStorage {
    fn new_with_token(token: String) -> Self {
        Self {
            token: Arc::new(Mutex::new(Some(token))),
            deleted: Arc::new(Mutex::new(false)),
        }
    }

    fn was_deleted(&self) -> bool {
        *self.deleted.lock().unwrap()
    }
}

impl TokenStorage for CapturingMockStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.token.lock().unwrap().clone())
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(None)
    }

    fn store_auth(&self, token: &str, _user_info: &UserInfo) -> Result<(), CliError> {
        *self.token.lock().unwrap() = Some(token.to_string());
        Ok(())
    }

    fn delete(&self) -> Result<(), CliError> {
        *self.token.lock().unwrap() = None;
        *self.deleted.lock().unwrap() = true;
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
fn test_logout_removes_token_from_keyring() {
    let storage = CapturingMockStorage::new_with_token("lin_api_test".to_string());
    let io = CapturingIo::new();

    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let result = handle_logout(&storage, &config, &io, None);

    assert!(result.is_ok());
    assert!(storage.was_deleted());
}

#[test]
fn test_logout_shows_success_message() {
    let storage = CapturingMockStorage::new_with_token("lin_api_test".to_string());
    let io = CapturingIo::new();

    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let result = handle_logout(&storage, &config, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert!(output.iter().any(|line| line.contains("Logged out")));
}

#[test]
fn test_logout_when_not_logged_in_succeeds() {
    let storage = CapturingMockStorage::new_with_token("".to_string());
    let io = CapturingIo::new();

    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let result = handle_logout(&storage, &config, &io, None);

    // Should succeed even if no token was present
    assert!(result.is_ok());
}
