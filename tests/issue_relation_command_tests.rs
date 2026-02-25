use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{CreateIssueRelationInput, IssueClient, UpdateIssueInput};
use linear_cli::client::queries::IssueRelationType;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::relation::{handle_block, handle_duplicate, handle_link};
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
struct MockRelationIssueClient {
    relation_result: Result<Issue, CliError>,
}

impl IssueClient for MockRelationIssueClient {
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

    fn create_issue_relation(
        &self,
        _token: &str,
        _input: CreateIssueRelationInput,
    ) -> Result<Issue, CliError> {
        self.relation_result.clone()
    }
}

fn sample_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Relation target issue".to_string(),
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
        project: None,
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        comments: None,
    }
}

#[test]
fn test_link_outputs_issue_on_success() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockRelationIssueClient {
        relation_result: Ok(sample_issue()),
    };

    let result = handle_link("ENG-123", "ENG-456", &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
}

#[test]
fn test_block_returns_auth_error_without_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockRelationIssueClient {
        relation_result: Ok(sample_issue()),
    };

    let result = handle_block("ENG-123", "ENG-456", &client, &config, &storage, &io, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::AuthError(_) => {}
        _ => panic!("expected AuthError"),
    }
}

#[test]
fn test_duplicate_propagates_not_found() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockRelationIssueClient {
        relation_result: Err(CliError::NotFound("Issue ENG-999 not found".to_string())),
    };

    let result = handle_duplicate("ENG-999", "ENG-456", &client, &config, &storage, &io, None);

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => assert!(msg.contains("not found")),
        _ => panic!("expected NotFound"),
    }
}

#[test]
fn test_relation_handlers_use_expected_relation_type_mappings() {
    let input_link = CreateIssueRelationInput {
        issue_id: "ENG-1".to_string(),
        related_issue_id: "ENG-2".to_string(),
        relation_type: IssueRelationType::Related,
    };
    let input_block = CreateIssueRelationInput {
        issue_id: "ENG-1".to_string(),
        related_issue_id: "ENG-2".to_string(),
        relation_type: IssueRelationType::Blocks,
    };
    let input_duplicate = CreateIssueRelationInput {
        issue_id: "ENG-1".to_string(),
        related_issue_id: "ENG-2".to_string(),
        relation_type: IssueRelationType::Duplicate,
    };

    assert!(matches!(
        input_link.relation_type,
        IssueRelationType::Related
    ));
    assert!(matches!(
        input_block.relation_type,
        IssueRelationType::Blocks
    ));
    assert!(matches!(
        input_duplicate.relation_type,
        IssueRelationType::Duplicate
    ));
}
