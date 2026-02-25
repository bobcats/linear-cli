use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{CreateIssueInput, IssueClient};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::create::handle_create;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::output::OutputFormat;
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
struct MockCreateIssueClient {
    create_result: Result<Issue, CliError>,
}

impl IssueClient for MockCreateIssueClient {
    fn get_issue(&self, _token: &str, _identifier: &str) -> Result<Issue, CliError> {
        unreachable!("not used in create handler tests")
    }

    fn list_issues(
        &self,
        _token: &str,
        _assignee: Option<String>,
        _project: Option<String>,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        unreachable!("not used in create handler tests")
    }

    fn create_issue(&self, _token: &str, _input: CreateIssueInput) -> Result<Issue, CliError> {
        self.create_result.clone()
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Implement issue create".to_string(),
        description: Some("Implement create handler".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::High,
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
        updated_at: "2026-02-23T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        comments: None,
    }
}

#[test]
fn test_create_outputs_full_issue_object_on_success() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateIssueClient {
        create_result: Ok(sample_issue()),
    };

    let result = handle_create(
        "ENG",
        "Implement issue create",
        Some("Implement create handler".to_string()),
        Some("@me".to_string()),
        None,
        None,
        Some(2),
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
    assert!(output.contains("Implement issue create"));
}

#[test]
fn test_create_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateIssueClient {
        create_result: Ok(sample_issue()),
    };

    let result = handle_create(
        "ENG",
        "Implement issue create",
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

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::AuthError(_) => {}
        _ => panic!("expected AuthError"),
    }
}

#[test]
fn test_create_propagates_not_found_for_unresolved_reference() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateIssueClient {
        create_result: Err(CliError::NotFound(
            "project not found for slug: unknown-project".to_string(),
        )),
    };

    let result = handle_create(
        "ENG",
        "Implement issue create",
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

#[test]
fn test_create_uses_config_provider_json_style_override() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    values.insert("LINEAR_CLI_JSON_STYLE".to_string(), "pretty".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateIssueClient {
        create_result: Ok(sample_issue()),
    };

    let result = handle_create(
        "ENG",
        "Implement issue create",
        Some("Implement create handler".to_string()),
        Some("@me".to_string()),
        None,
        None,
        Some(2),
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(
        output.contains('\n'),
        "issue create JSON should be pretty when LINEAR_CLI_JSON_STYLE=pretty is provided by config"
    );
    assert!(output.contains("\"identifier\": \"ENG-123\""));
}
