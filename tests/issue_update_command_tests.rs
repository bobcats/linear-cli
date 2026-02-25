use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{IssueClient, UpdateIssueInput};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::update::handle_update;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct TestConfigProvider {
    values: HashMap<String, String>,
}

impl ConfigProvider for TestConfigProvider {
    fn get_var(&self, key: &str) -> Option<String> {
        self.values.get(key).cloned()
    }
}

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

#[derive(Clone)]
struct MockUpdateIssueClient {
    update_result: Result<Issue, CliError>,
}

impl IssueClient for MockUpdateIssueClient {
    fn get_issue(&self, _token: &str, _identifier: &str) -> Result<Issue, CliError> {
        unreachable!("not used")
    }

    fn list_issues(
        &self,
        _token: &str,
        _assignee: Option<String>,
        _project: Option<String>,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        unreachable!("not used")
    }

    fn update_issue(
        &self,
        _token: &str,
        _id: &str,
        _input: UpdateIssueInput,
    ) -> Result<Issue, CliError> {
        self.update_result.clone()
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Updated issue title".to_string(),
        description: Some("Updated description".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::Medium,
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
        project: None,
        created_at: "2026-02-23T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        comments: None,
    }
}

#[test]
fn test_update_outputs_full_issue_object_on_success() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockUpdateIssueClient {
        update_result: Ok(sample_issue()),
    };

    let result = handle_update(
        "ENG-123",
        Some("Updated issue title".to_string()),
        None,
        None,
        None,
        None,
        None,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
    assert!(output.contains("Updated issue title"));
}

#[test]
fn test_update_returns_invalid_args_when_no_patch_fields() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockUpdateIssueClient {
        update_result: Ok(sample_issue()),
    };

    let result = handle_update(
        "ENG-123", None, None, None, None, None, None, &client, &config, &storage, &io, None,
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::InvalidArgs(msg) => assert!(msg.contains("at least one")),
        _ => panic!("expected InvalidArgs error"),
    }
}

#[test]
fn test_update_propagates_not_found_for_unresolved_reference() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockUpdateIssueClient {
        update_result: Err(CliError::NotFound(
            "project not found for slug: unknown-project".to_string(),
        )),
    };

    let result = handle_update(
        "ENG-123",
        None,
        None,
        None,
        Some("unknown-project".to_string()),
        None,
        None,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => assert!(msg.contains("project")),
        _ => panic!("expected NotFound error"),
    }
}
