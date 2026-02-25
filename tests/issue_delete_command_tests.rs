use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::IssueClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::delete::handle_delete;
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

/// Mock that tracks delete calls
struct MockIssueClient {
    get_result: Result<Issue, CliError>,
    delete_result: Result<(), CliError>,
}

impl IssueClient for MockIssueClient {
    fn get_issue(&self, _token: &str, _identifier: &str) -> Result<Issue, CliError> {
        self.get_result.clone()
    }

    fn list_issues(
        &self,
        _token: &str,
        _assignee: Option<String>,
        _project: Option<String>,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        Ok(vec![])
    }

    fn delete_issue(&self, _token: &str, _id: &str, _permanently: bool) -> Result<(), CliError> {
        self.delete_result.clone()
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-uuid-123".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Test issue".to_string(),
        description: None,
        priority: Priority::from_i32(0),
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
        },
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        url: "https://linear.app/issue/ENG-123".to_string(),
        project: None,
        comments: None,
    }
}

#[test]
fn test_delete_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockIssueClient {
        get_result: Ok(sample_issue()),
        delete_result: Ok(()),
    };
    let io = CapturingIo::new();

    let result = handle_delete("ENG-123", false, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_delete_succeeds_and_outputs_confirmation() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockIssueClient {
        get_result: Ok(sample_issue()),
        delete_result: Ok(()),
    };
    let io = CapturingIo::new();

    let result = handle_delete("ENG-123", false, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123") || output.contains("deleted"));
}

#[test]
fn test_delete_propagates_not_found_error() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockIssueClient {
        get_result: Err(CliError::NotFound("Issue ENG-999 not found".to_string())),
        delete_result: Ok(()),
    };
    let io = CapturingIo::new();

    let result = handle_delete("ENG-999", false, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}
