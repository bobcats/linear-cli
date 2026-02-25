use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::users::MockUserClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::users::commands::list::handle_list;
use linear_cli::users::types::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct MockStorage {
    token: Option<String>,
}

impl TokenStorage for MockStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.token.clone())
    }
    fn get_user_info(&self) -> Result<Option<linear_cli::client::auth::UserInfo>, CliError> {
        Ok(None)
    }
    fn store_auth(
        &self,
        _token: &str,
        _user_info: &linear_cli::client::auth::UserInfo,
    ) -> Result<(), CliError> {
        Ok(())
    }
    fn delete(&self) -> Result<(), CliError> {
        Ok(())
    }
}

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

impl Io for CapturingIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok(String::new())
    }
    fn print(&self, message: &str) {
        self.stdout.lock().unwrap().push(message.to_string());
    }
    fn print_error(&self, _message: &str) {}
}

fn config_with_token() -> TestConfigProvider {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    TestConfigProvider { values }
}

#[test]
fn test_user_list_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockUserClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_user_list_returns_users_on_success() {
    let users = vec![
        User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            display_name: "alice".to_string(),
            email: "alice@example.com".to_string(),
            active: true,
            admin: true,
            guest: false,
        },
        User {
            id: "user-2".to_string(),
            name: "Bob".to_string(),
            display_name: "bob".to_string(),
            email: "bob@example.com".to_string(),
            active: true,
            admin: false,
            guest: false,
        },
    ];
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockUserClient {
        list_result: Ok(users),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));
}

#[test]
fn test_user_list_returns_empty_list() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockUserClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
}
