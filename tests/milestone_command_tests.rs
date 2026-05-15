use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::milestones::{CreateMilestoneInput, MilestoneClient, UpdateMilestoneInput};
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::milestones::commands::{
    handle_create, handle_delete, handle_list, handle_update, handle_view,
};
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

struct TestStorage;

impl TokenStorage for TestStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(None)
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
    fn output(&self) -> String {
        self.stdout.lock().expect("mutex poisoned").join("\n")
    }
}

impl Io for CapturingIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok("ignored".to_string())
    }
    fn print(&self, message: &str) {
        self.stdout
            .lock()
            .expect("mutex poisoned")
            .push(message.to_string());
    }
    fn print_bytes(&self, bytes: &[u8]) {
        self.stdout
            .lock()
            .expect("mutex poisoned")
            .push(String::from_utf8_lossy(bytes).to_string());
    }
    fn print_error(&self, _message: &str) {}
}

#[derive(Default)]
struct RecordingMilestoneClient {
    created: Arc<Mutex<Option<CreateMilestoneInput>>>,
    updated: Arc<Mutex<Option<(String, UpdateMilestoneInput)>>>,
    listed_project_id: Arc<Mutex<Option<String>>>,
}

impl MilestoneReferenceLookup for RecordingMilestoneClient {
    fn get_milestone_by_id(&self, _token: &str, id: &str) -> Result<Option<Milestone>, CliError> {
        Ok(Some(milestone(id, "Beta", "App", "project-1")))
    }

    fn find_milestones_by_name(
        &self,
        _token: &str,
        name: &str,
        project_id: Option<&str>,
    ) -> Result<Vec<Milestone>, CliError> {
        *self.listed_project_id.lock().expect("mutex poisoned") = project_id.map(str::to_string);
        if name == "Ambiguous" {
            return Ok(vec![
                milestone("milestone-1", "Ambiguous", "App", "project-1"),
                milestone("milestone-2", "Ambiguous", "Web", "project-2"),
            ]);
        }
        Ok(vec![milestone(
            "milestone-1",
            name,
            "App",
            project_id.unwrap_or("project-1"),
        )])
    }

    fn resolve_project_id_by_slug(
        &self,
        _token: &str,
        slug: &str,
    ) -> Result<Option<String>, CliError> {
        Ok(match slug {
            "APP" | "project-1" => Some("project-1".to_string()),
            "WEB" | "project-2" => Some("project-2".to_string()),
            _ => None,
        })
    }
}

impl MilestoneClient for RecordingMilestoneClient {
    fn get_milestone(&self, _token: &str, id: &str) -> Result<Milestone, CliError> {
        let mut milestone = sample_milestone();
        milestone.id = id.to_string();
        Ok(milestone)
    }

    fn list_milestones(
        &self,
        _token: &str,
        _project_id: Option<&str>,
        _name: Option<&str>,
        _limit: usize,
    ) -> Result<Vec<Milestone>, CliError> {
        Ok(vec![sample_milestone()])
    }

    fn create_milestone(
        &self,
        _token: &str,
        input: CreateMilestoneInput,
    ) -> Result<Milestone, CliError> {
        *self.created.lock().expect("mutex poisoned") = Some(input);
        Ok(sample_milestone())
    }

    fn update_milestone(
        &self,
        _token: &str,
        id: &str,
        input: UpdateMilestoneInput,
    ) -> Result<Milestone, CliError> {
        *self.updated.lock().expect("mutex poisoned") = Some((id.to_string(), input));
        Ok(sample_milestone())
    }

    fn delete_milestone(&self, _token: &str, _id: &str) -> Result<(), CliError> {
        Ok(())
    }
}

fn config_with_token() -> TestConfigProvider {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test-token".to_string());
    TestConfigProvider { values }
}

fn sample_milestone() -> Milestone {
    milestone("milestone-1", "Beta", "App", "project-1")
}

fn milestone(id: &str, name: &str, project_name: &str, project_id: &str) -> Milestone {
    Milestone {
        id: id.to_string(),
        name: name.to_string(),
        description: Some("Beta readiness".to_string()),
        status: "next".to_string(),
        progress: 0.5,
        sort_order: 1000.0,
        target_date: Some("2026-06-30".to_string()),
        project: MilestoneProject {
            id: project_id.to_string(),
            name: project_name.to_string(),
            slug_id: project_name.to_lowercase(),
        },
        created_at: "2026-05-01T00:00:00Z".to_string(),
        updated_at: "2026-05-02T00:00:00Z".to_string(),
        archived_at: None,
    }
}

#[test]
fn milestone_list_prints_json_from_client() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_list(
        None,
        10,
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    assert!(io.output().contains("Beta"));
}

#[test]
fn milestone_view_prints_one_milestone() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_view(
        "milestone-1",
        None,
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Table),
    )
    .unwrap();

    assert!(io.output().contains("Beta"));
}

#[test]
fn milestone_create_passes_input_to_client() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_create(
        "project-1",
        "Beta",
        Some("Beta readiness".to_string()),
        Some("2026-06-30".to_string()),
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    assert_eq!(
        *client.created.lock().expect("mutex poisoned"),
        Some(CreateMilestoneInput {
            project_id: "project-1".to_string(),
            name: "Beta".to_string(),
            description: Some("Beta readiness".to_string()),
            target_date: Some("2026-06-30".to_string())
        })
    );
}

#[test]
fn milestone_update_requires_patch_field() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    let result = handle_update(
        "milestone-1",
        None,
        UpdateMilestoneInput::default(),
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(
        matches!(result, Err(CliError::InvalidArgs(message)) if message.contains("at least one"))
    );
}

#[test]
fn milestone_delete_prints_json_success() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_delete(
        "milestone-1",
        None,
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&io.output()).unwrap();
    assert_eq!(parsed["deleted"], true);
    assert_eq!(parsed["id"], "milestone-1");
}

#[test]
fn milestone_view_resolves_scoped_name() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_view(
        "Beta",
        Some("APP"),
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    assert_eq!(
        *client.listed_project_id.lock().expect("mutex poisoned"),
        Some("project-1".to_string())
    );
}

#[test]
fn milestone_create_resolves_project_slug_before_create() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_create(
        "APP",
        "Beta",
        None,
        None,
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    assert_eq!(
        client
            .created
            .lock()
            .expect("mutex poisoned")
            .as_ref()
            .unwrap()
            .project_id,
        "project-1"
    );
}

#[test]
fn milestone_delete_resolves_scoped_name_before_delete() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    handle_delete(
        "Beta",
        Some("APP"),
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    )
    .unwrap();

    assert_eq!(
        *client.listed_project_id.lock().expect("mutex poisoned"),
        Some("project-1".to_string())
    );
}

#[test]
fn milestone_ambiguous_name_propagates_invalid_args() {
    let client = RecordingMilestoneClient::default();
    let io = CapturingIo::new();

    let result = handle_view(
        "Ambiguous",
        None,
        &client,
        &client,
        &config_with_token(),
        &TestStorage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(matches!(result, Err(CliError::InvalidArgs(message)) if message.contains("--project")));
}
