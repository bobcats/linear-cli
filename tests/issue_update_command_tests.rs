use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{IssueClient, IssueFieldPatch, UpdateIssueInput};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::update::handle_update;
use linear_cli::issues::resolver::IssueReferenceLookup;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::milestones::resolver::MilestoneReferenceLookup;
use linear_cli::milestones::types::{Milestone, MilestoneProject};
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

struct PassthroughLookup;

impl MilestoneReferenceLookup for PassthroughLookup {
    fn get_milestone_by_id(&self, _token: &str, id: &str) -> Result<Option<Milestone>, CliError> {
        Ok(Some(test_milestone(id, "Beta", "project-from-slug")))
    }

    fn find_milestones_by_name(
        &self,
        _token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<Milestone>, CliError> {
        Ok(vec![test_milestone(
            "milestone-1",
            name,
            project_id.unwrap_or("project-from-slug"),
        )])
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        if slug == "app" {
            Ok(Some("project-from-slug".to_string()))
        } else {
            Ok(Some("project-from-slug".to_string()))
        }
    }
}

impl IssueReferenceLookup for PassthroughLookup {
    fn resolve_viewer_id(&self, _token: &str) -> Result<String, CliError> {
        Ok("viewer-123".to_string())
    }

    fn resolve_user_id_by_email(
        &self,
        _token: &str,
        _email: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("user-from-email".to_string()))
    }

    fn resolve_team_id_by_key(&self, _token: &str, _key: &str) -> Result<Option<String>, CliError> {
        Ok(Some("team-from-key".to_string()))
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        _slug: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("project-from-slug".to_string()))
    }

    fn resolve_state_id_by_name(
        &self,
        _token: &str,
        _name: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("state-from-name".to_string()))
    }

    fn resolve_issue_id_by_identifier(
        &self,
        _token: &str,
        _identifier: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some("issue-from-identifier".to_string()))
    }
}

fn test_milestone(id: &str, name: &str, project_id: &str) -> Milestone {
    Milestone {
        id: id.to_string(),
        name: name.to_string(),
        description: None,
        status: "next".to_string(),
        progress: 0.0,
        sort_order: 0.0,
        target_date: None,
        project: MilestoneProject {
            id: project_id.to_string(),
            name: "App".to_string(),
            slug_id: "app".to_string(),
        },
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-01T00:00:00Z".to_string(),
        archived_at: None,
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
            state_type: "started".to_string(),
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
        milestone: None,
        created_at: "2026-02-23T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        parent: None,
        children: None,
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
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
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
        "ENG-123",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
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
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
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
fn test_update_with_parent_passes_through_to_client() {
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
        None,
        None,
        None,
        None,
        None,
        Some("ENG-100".to_string()),
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123"));
}

#[test]
fn test_update_with_only_parent_does_not_require_other_fields() {
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
        None,
        None,
        None,
        None,
        None,
        Some("ENG-100".to_string()),
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
}

#[test]
fn test_update_resolves_at_me_to_viewer_uuid_before_sending_to_client() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let captured_input: Arc<Mutex<Option<UpdateIssueInput>>> = Arc::new(Mutex::new(None));
    let captured = captured_input.clone();

    struct CapturingClient {
        captured: Arc<Mutex<Option<UpdateIssueInput>>>,
        result: Issue,
    }

    impl IssueClient for CapturingClient {
        fn get_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
            unreachable!()
        }
        fn list_issues(
            &self,
            _token: &str,
            _assignee: Option<String>,
            _project: Option<String>,
            _limit: usize,
        ) -> Result<Vec<Issue>, CliError> {
            unreachable!()
        }
        fn update_issue(
            &self,
            _token: &str,
            _id: &str,
            input: UpdateIssueInput,
        ) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(self.result.clone())
        }
    }

    let client = CapturingClient {
        captured,
        result: sample_issue(),
    };

    let result = handle_update(
        "ENG-123",
        None,
        None,
        Some("@me".to_string()),
        None,
        None,
        None,
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let input = captured_input.lock().unwrap();
    let input = input
        .as_ref()
        .expect("update_issue should have been called");
    assert_eq!(
        input.assignee_id.as_deref(),
        Some("viewer-123"),
        "@me should be resolved to viewer UUID, not passed as raw string"
    );
}

#[test]
fn test_update_with_project_scopes_milestone_resolution() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let captured_input: Arc<Mutex<Option<UpdateIssueInput>>> = Arc::new(Mutex::new(None));

    struct CapturingClient {
        captured: Arc<Mutex<Option<UpdateIssueInput>>>,
    }

    impl IssueClient for CapturingClient {
        fn get_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
            unreachable!()
        }
        fn list_issues(
            &self,
            _token: &str,
            _assignee: Option<String>,
            _project: Option<String>,
            _limit: usize,
        ) -> Result<Vec<Issue>, CliError> {
            unreachable!()
        }
        fn update_issue(
            &self,
            _token: &str,
            _id: &str,
            input: UpdateIssueInput,
        ) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(sample_issue())
        }
    }

    let client = CapturingClient {
        captured: captured_input.clone(),
    };

    let result = handle_update(
        "ENG-123",
        None,
        None,
        None,
        Some("app".to_string()),
        None,
        None,
        None,
        Some("Beta".to_string()),
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let input = captured_input.lock().unwrap();
    assert_eq!(
        input.as_ref().map(|input| &input.project_milestone_id),
        Some(&IssueFieldPatch::Set("milestone-1".to_string()))
    );
}

#[test]
fn test_update_with_milestone_null_clears_and_counts_as_patch() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let captured_input: Arc<Mutex<Option<UpdateIssueInput>>> = Arc::new(Mutex::new(None));

    struct CapturingClient {
        captured: Arc<Mutex<Option<UpdateIssueInput>>>,
    }

    impl IssueClient for CapturingClient {
        fn get_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
            unreachable!()
        }
        fn list_issues(
            &self,
            _token: &str,
            _assignee: Option<String>,
            _project: Option<String>,
            _limit: usize,
        ) -> Result<Vec<Issue>, CliError> {
            unreachable!()
        }
        fn update_issue(
            &self,
            _token: &str,
            _id: &str,
            input: UpdateIssueInput,
        ) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(sample_issue())
        }
    }

    let client = CapturingClient {
        captured: captured_input.clone(),
    };

    let result = handle_update(
        "ENG-123",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("null".to_string()),
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let input = captured_input.lock().unwrap();
    assert_eq!(
        input.as_ref().map(|input| &input.project_milestone_id),
        Some(&IssueFieldPatch::Clear)
    );
}

#[test]
fn test_update_without_milestone_leaves_milestone_unchanged() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let captured_input: Arc<Mutex<Option<UpdateIssueInput>>> = Arc::new(Mutex::new(None));

    struct CapturingClient {
        captured: Arc<Mutex<Option<UpdateIssueInput>>>,
    }

    impl IssueClient for CapturingClient {
        fn get_issue(&self, _token: &str, _id: &str) -> Result<Issue, CliError> {
            unreachable!()
        }
        fn list_issues(
            &self,
            _token: &str,
            _assignee: Option<String>,
            _project: Option<String>,
            _limit: usize,
        ) -> Result<Vec<Issue>, CliError> {
            unreachable!()
        }
        fn update_issue(
            &self,
            _token: &str,
            _id: &str,
            input: UpdateIssueInput,
        ) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(sample_issue())
        }
    }

    let client = CapturingClient {
        captured: captured_input.clone(),
    };

    let result = handle_update(
        "ENG-123",
        Some("Rename".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let input = captured_input.lock().unwrap();
    assert_eq!(
        input.as_ref().map(|input| &input.project_milestone_id),
        Some(&IssueFieldPatch::Unchanged)
    );
}
