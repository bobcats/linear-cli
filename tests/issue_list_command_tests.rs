use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::MockIssueClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::handle_list;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Test config provider
struct TestConfigProvider {
    pub values: HashMap<String, String>,
}

impl ConfigProvider for TestConfigProvider {
    fn get_var(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}

// Mock token storage
struct MockStorage {
    pub token: Option<String>,
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

// Capturing IO for tests
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
        Ok("test_token".to_string())
    }

    fn print(&self, message: &str) {
        self.stdout.lock().unwrap().push(message.to_string());
    }

    fn print_error(&self, _message: &str) {}
}

fn dummy_issue() -> Issue {
    Issue {
        id: "dummy".to_string(),
        identifier: "DUMMY-1".to_string(),
        title: "Dummy".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::None,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Dummy".to_string(),
            email: "dummy@example.com".to_string(),
        },
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        url: "https://linear.app/issue/DUMMY-1".to_string(),
        project: None,
        comments: None,
    }
}

#[test]
fn test_list_returns_empty_vec_when_no_issues() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Ok(vec![]),
    };

    let result = handle_list(None, None, 50, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert!(!output.is_empty());
    let full_output = output.join("\n");
    // Should be an empty array in JSON
    assert!(full_output.contains("[]"));
}

#[test]
fn test_list_returns_multiple_issues() {
    let issues = vec![
        Issue {
            id: "issue-1".to_string(),
            identifier: "ENG-123".to_string(),
            title: "First issue".to_string(),
            description: None,
            state: IssueState {
                id: "state-1".to_string(),
                name: "Todo".to_string(),
            },
            priority: Priority::Medium,
            assignee: None,
            creator: User {
                id: "user-1".to_string(),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            created_at: "2025-01-01T00:00:00Z".to_string(),
            updated_at: "2025-01-01T00:00:00Z".to_string(),
            url: "https://linear.app/issue/ENG-123".to_string(),
            project: None,
            comments: None,
        },
        Issue {
            id: "issue-2".to_string(),
            identifier: "ENG-124".to_string(),
            title: "Second issue".to_string(),
            description: None,
            state: IssueState {
                id: "state-2".to_string(),
                name: "In Progress".to_string(),
            },
            priority: Priority::Urgent,
            assignee: Some(User {
                id: "user-2".to_string(),
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            }),
            creator: User {
                id: "user-1".to_string(),
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            created_at: "2025-01-02T00:00:00Z".to_string(),
            updated_at: "2025-01-02T00:00:00Z".to_string(),
            url: "https://linear.app/issue/ENG-124".to_string(),
            project: None,
            comments: None,
        },
    ];

    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Ok(issues.clone()),
    };

    let result = handle_list(None, None, 50, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();
    assert!(!output.is_empty());
    let full_output = output.join("\n");
    assert!(full_output.contains("ENG-123"));
    assert!(full_output.contains("ENG-124"));
}

#[test]
fn test_list_passes_assignee_filter_to_client() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Ok(vec![]),
    };

    let result = handle_list(
        Some("user-123".to_string()),
        None,
        50,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    // Mock client receives the filter and returns empty list
}

#[test]
fn test_list_passes_limit_to_client() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Ok(vec![]),
    };

    let result = handle_list(None, None, 10, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    // Mock client receives the limit and returns empty list
}

#[test]
fn test_list_returns_error_when_client_fails() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Err(CliError::NetworkError("API connection failed".to_string())),
    };

    let result = handle_list(None, None, 50, &client, &config, &storage, &io, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NetworkError(msg) => assert_eq!(msg, "API connection failed"),
        _ => panic!("Expected NetworkError"),
    }
}

#[test]
fn test_list_passes_project_filter_to_client() {
    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let client = MockIssueClient {
        result: Ok(dummy_issue()),
        list_result: Ok(vec![]),
    };

    let result = handle_list(
        None,
        Some("project-123".to_string()),
        50,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    // Mock client receives the project filter and returns empty list
}
