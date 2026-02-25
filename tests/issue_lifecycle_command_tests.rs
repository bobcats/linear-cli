use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{IssueClient, UpdateIssueInput};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::lifecycle::{handle_archive, handle_unarchive};
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
struct MockLifecycleIssueClient {
    archive_result: Result<Issue, CliError>,
    unarchive_result: Result<Issue, CliError>,
}

impl IssueClient for MockLifecycleIssueClient {
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
        unreachable!("not used")
    }

    fn archive_issue(&self, _token: &str, _id: &str, _trash: bool) -> Result<Issue, CliError> {
        self.archive_result.clone()
    }

    fn unarchive_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
        self.unarchive_result.clone()
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Lifecycle target issue".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Done".to_string(),
        },
        priority: Priority::Low,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        project: None,
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        comments: None,
    }
}

#[test]
fn test_archive_outputs_issue_on_success() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockLifecycleIssueClient {
        archive_result: Ok(sample_issue()),
        unarchive_result: Ok(sample_issue()),
    };

    let result = handle_archive("ENG-123", false, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
}

#[test]
fn test_unarchive_returns_auth_error_without_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockLifecycleIssueClient {
        archive_result: Ok(sample_issue()),
        unarchive_result: Ok(sample_issue()),
    };

    let result = handle_unarchive("ENG-123", &client, &config, &storage, &io, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::AuthError(_) => {}
        _ => panic!("expected AuthError"),
    }
}

#[test]
fn test_archive_propagates_not_found() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockLifecycleIssueClient {
        archive_result: Err(CliError::NotFound("Issue ENG-999 not found".to_string())),
        unarchive_result: Ok(sample_issue()),
    };

    let result = handle_archive("ENG-999", false, &client, &config, &storage, &io, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => assert!(msg.contains("not found")),
        _ => panic!("expected NotFound"),
    }
}
