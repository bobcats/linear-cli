use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::search::MockSearchClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::search::handle_search;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
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

fn make_issue(identifier: &str, title: &str) -> Issue {
    Issue {
        id: format!("id-{identifier}"),
        identifier: identifier.to_string(),
        title: title.to_string(),
        description: None,
        priority: Priority::from_i32(0),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
        },
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        url: format!("https://linear.app/issue/{identifier}"),
        project: None,
        comments: None,
    }
}

#[test]
fn test_search_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockSearchClient {
        search_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_search(
        "test", None, false, 50, &client, &config, &storage, &io, None,
    );

    assert!(result.is_err());
}

#[test]
fn test_search_returns_results_on_success() {
    let issues = vec![
        make_issue("ENG-123", "Fix token refresh"),
        make_issue("ENG-456", "Token expiry handling"),
    ];
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockSearchClient {
        search_result: Ok(issues),
    };
    let io = CapturingIo::new();

    let result = handle_search(
        "token", None, false, 50, &client, &config, &storage, &io, None,
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
    assert!(output.contains("ENG-456"));
}

#[test]
fn test_search_returns_empty_results() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockSearchClient {
        search_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_search(
        "nonexistent",
        None,
        false,
        50,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
}

#[test]
fn test_search_propagates_client_error() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockSearchClient {
        search_result: Err(CliError::NetworkError("connection failed".to_string())),
    };
    let io = CapturingIo::new();

    let result = handle_search(
        "test", None, false, 50, &client, &config, &storage, &io, None,
    );

    assert!(result.is_err());
}
