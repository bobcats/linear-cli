use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::comments::MockCommentClient;
use linear_cli::client::issues::MockIssueClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::view::{ViewDeps, handle_view};
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock storage for testing
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

// Capturing IO for testing
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

    fn print_error(&self, _message: &str) {
        // Ignore for now
    }
}

#[test]
fn test_view_returns_issue_details_for_valid_identifier() {
    let issue = Issue {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Fix login bug".to_string(),
        description: Some("User cannot login with email".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::Urgent,
        assignee: Some(User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
        creator: User {
            id: "user-2".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-16T14:20:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        project: None,
        comments: None,
    };

    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };

    let storage = MockStorage { token: None };

    let issue_client = MockIssueClient {
        result: Ok(issue.clone()),
        list_result: Ok(vec![]),
    };

    let comment_client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Err(CliError::General("not used".to_string())),
        delete_result: Ok(()),
    };

    let io = CapturingIo::new();

    let deps = ViewDeps {
        issue_client: &issue_client,
        comment_client: &comment_client,
        config: &config,
        storage: &storage,
        io: &io,
    };

    let result = handle_view("ENG-123", false, 50, &deps, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();

    // Verify issue details are in output
    assert!(!output.is_empty());
    let full_output = output.join("\n");
    assert!(full_output.contains("ENG-123"));
    assert!(full_output.contains("Fix login bug"));
}

#[test]
fn test_view_returns_not_found_error_for_invalid_identifier() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };

    let storage = MockStorage { token: None };

    let issue_client = MockIssueClient {
        result: Err(CliError::NotFound(
            "Issue INVALID-123 not found".to_string(),
        )),
        list_result: Ok(vec![]),
    };

    let comment_client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Err(CliError::General("not used".to_string())),
        delete_result: Ok(()),
    };

    let io = CapturingIo::new();

    let deps = ViewDeps {
        issue_client: &issue_client,
        comment_client: &comment_client,
        config: &config,
        storage: &storage,
        io: &io,
    };

    let result = handle_view("INVALID-123", false, 50, &deps, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => {
            assert!(msg.contains("not found") || msg.contains("INVALID-123"));
        }
        _ => panic!("Expected NotFound error"),
    }
}

#[test]
fn test_view_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };

    let storage = MockStorage { token: None };

    let issue = Issue {
        id: "123".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Test".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::None,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        project: None,
        comments: None,
    };

    let issue_client = MockIssueClient {
        result: Ok(issue),
        list_result: Ok(vec![]),
    };

    let comment_client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Err(CliError::General("not used".to_string())),
        delete_result: Ok(()),
    };

    let io = CapturingIo::new();

    let deps = ViewDeps {
        issue_client: &issue_client,
        comment_client: &comment_client,
        config: &config,
        storage: &storage,
        io: &io,
    };

    let result = handle_view("ENG-123", false, 50, &deps, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::AuthError(_) => {}
        _ => panic!("Expected AuthError"),
    }
}

#[test]
fn test_view_returns_network_error_on_api_failure() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };

    let storage = MockStorage { token: None };

    let issue_client = MockIssueClient {
        result: Err(CliError::NetworkError("Connection failed".to_string())),
        list_result: Ok(vec![]),
    };

    let comment_client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Err(CliError::General("not used".to_string())),
        delete_result: Ok(()),
    };

    let io = CapturingIo::new();

    let deps = ViewDeps {
        issue_client: &issue_client,
        comment_client: &comment_client,
        config: &config,
        storage: &storage,
        io: &io,
    };

    let result = handle_view("ENG-123", false, 50, &deps, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NetworkError(msg) => {
            assert!(msg.contains("Connection failed"));
        }
        _ => panic!("Expected NetworkError"),
    }
}
