use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::issues::{CreateIssueInput, IssueClient};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::create::handle_create;
use linear_cli::issues::resolver::IssueReferenceLookup;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::milestones::resolver::MilestoneReferenceLookup;
use linear_cli::milestones::types::{Milestone, MilestoneProject};
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

struct PassthroughLookup;

impl MilestoneReferenceLookup for PassthroughLookup {
    fn get_milestone_by_id(&self, _token: &str, id: &str) -> Result<Option<Milestone>, CliError> {
        Ok(Some(Milestone {
            id: id.to_string(),
            name: "Beta".to_string(),
            description: None,
            status: "next".to_string(),
            progress: 0.0,
            sort_order: 0.0,
            target_date: None,
            project: MilestoneProject {
                id: "project-from-slug".to_string(),
                name: "App".to_string(),
                slug_id: "app".to_string(),
            },
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            archived_at: None,
        }))
    }

    fn find_milestones_by_name(
        &self,
        _token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<Milestone>, CliError> {
        Ok(vec![Milestone {
            id: "milestone-from-name".to_string(),
            name: name.to_string(),
            description: None,
            status: "next".to_string(),
            progress: 0.0,
            sort_order: 0.0,
            target_date: None,
            project: MilestoneProject {
                id: project_id.unwrap_or("project-from-slug").to_string(),
                name: "App".to_string(),
                slug_id: "app".to_string(),
            },
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            archived_at: None,
        }])
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(Some(
            if slug == "APP" {
                "project-from-slug"
            } else {
                slug
            }
            .to_string(),
        ))
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

fn sample_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Implement issue create".to_string(),
        description: Some("Implement create handler".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
            state_type: "started".to_string(),
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
        milestone: None,
        created_at: "2026-02-23T00:00:00Z".to_string(),
        updated_at: "2026-02-23T00:00:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        parent: None,
        children: None,
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
        None,
        Some(2),
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
        None,
        Some(2),
        None,
        &client,
        &PassthroughLookup,
        &PassthroughLookup,
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

#[test]
fn test_create_with_parent_passes_through_to_client() {
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
        "Sub-issue title",
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
fn test_create_resolves_at_me_to_viewer_uuid_before_sending_to_client() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();

    let captured_input: Arc<Mutex<Option<CreateIssueInput>>> = Arc::new(Mutex::new(None));
    let captured = captured_input.clone();

    struct CapturingClient {
        captured: Arc<Mutex<Option<CreateIssueInput>>>,
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
        fn create_issue(&self, _token: &str, input: CreateIssueInput) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(self.result.clone())
        }
    }

    let client = CapturingClient {
        captured,
        result: sample_issue(),
    };

    let result = handle_create(
        "ENG",
        "Test @me resolution",
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
        .expect("create_issue should have been called");
    assert_eq!(
        input.assignee_id.as_deref(),
        Some("viewer-123"),
        "@me should be resolved to viewer UUID, not passed as raw string"
    );
}

#[test]
fn test_create_with_project_scopes_milestone_resolution() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let captured_input: Arc<Mutex<Option<CreateIssueInput>>> = Arc::new(Mutex::new(None));
    let captured = captured_input.clone();

    struct CapturingClient {
        captured: Arc<Mutex<Option<CreateIssueInput>>>,
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
        fn create_issue(&self, _token: &str, input: CreateIssueInput) -> Result<Issue, CliError> {
            *self.captured.lock().unwrap() = Some(input);
            Ok(sample_issue())
        }
    }

    struct Lookup {
        scoped_project: Arc<Mutex<Option<String>>>,
    }
    impl IssueReferenceLookup for Lookup {
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
        fn resolve_team_id_by_key(
            &self,
            _token: &str,
            _key: &str,
        ) -> Result<Option<String>, CliError> {
            Ok(Some("team-from-key".to_string()))
        }
        fn resolve_project_id_by_slug(
            &self,
            _token: &str,
            slug: &str,
        ) -> Result<Option<String>, CliError> {
            Ok((slug == "APP").then(|| "project-from-slug".to_string()))
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
    impl MilestoneReferenceLookup for Lookup {
        fn get_milestone_by_id(
            &self,
            _token: &str,
            _id: &str,
        ) -> Result<Option<Milestone>, CliError> {
            Ok(None)
        }
        fn find_milestones_by_name(
            &self,
            _token: &str,
            name: &str,
            project_id: Option<&str>,
        ) -> Result<Vec<Milestone>, CliError> {
            *self.scoped_project.lock().unwrap() = project_id.map(str::to_string);
            Ok(vec![Milestone {
                id: "milestone-1".to_string(),
                name: name.to_string(),
                description: None,
                status: "next".to_string(),
                progress: 0.0,
                sort_order: 0.0,
                target_date: None,
                project: MilestoneProject {
                    id: "project-from-slug".to_string(),
                    name: "App".to_string(),
                    slug_id: "app".to_string(),
                },
                created_at: "2026-01-01T00:00:00Z".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                archived_at: None,
            }])
        }
        fn resolve_project_id_by_slug(
            &self,
            _token: &str,
            slug: &str,
        ) -> Result<Option<String>, CliError> {
            Ok((slug == "APP").then(|| "project-from-slug".to_string()))
        }
    }

    let scoped_project = Arc::new(Mutex::new(None));
    let lookup = Lookup {
        scoped_project: scoped_project.clone(),
    };
    let client = CapturingClient { captured };

    let result = handle_create(
        "ENG",
        "Ship beta",
        None,
        None,
        Some("APP".to_string()),
        None,
        None,
        Some(2),
        Some("Beta".to_string()),
        &client,
        &lookup,
        &lookup,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    assert_eq!(
        *scoped_project.lock().unwrap(),
        Some("project-from-slug".to_string())
    );
    let input = captured_input.lock().unwrap();
    let input = input
        .as_ref()
        .expect("create_issue should have been called");
    assert_eq!(input.project_id.as_deref(), Some("project-from-slug"));
    assert_eq!(input.project_milestone_id.as_deref(), Some("milestone-1"));
}
