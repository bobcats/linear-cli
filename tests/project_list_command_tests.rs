use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::projects::MockProjectClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::output::OutputFormat;
use linear_cli::projects::commands::{
    handle_list as handle_project_list, handle_view as handle_project_view,
};
use linear_cli::projects::types::Project;
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

fn sample_project() -> Project {
    Project {
        id: "project-1".to_string(),
        name: "Backend Services".to_string(),
        description: "Core API project".to_string(),
        content: None,
        slug_id: "backend-services".to_string(),
        url: "https://linear.app/project/backend-services".to_string(),
        color: "#2563eb".to_string(),
        icon: Some("⚙️".to_string()),
        status_name: "In Progress".to_string(),
        status_type: "started".to_string(),
        status_color: "#60a5fa".to_string(),
        progress: 0.42,
        priority: 2,
        priority_label: "High".to_string(),
        start_date: Some("2026-02-01".to_string()),
        target_date: Some("2026-03-01".to_string()),
        created_at: "2026-02-01T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        lead_name: Some("Alice".to_string()),
    }
}

#[test]
fn test_project_list_uses_config_provider_json_style_override() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test-token".to_string());
    values.insert("LINEAR_CLI_JSON_STYLE".to_string(), "pretty".to_string());

    let config = TestConfigProvider { values };
    let storage = TestStorage;
    let io = CapturingIo::new();

    let client = MockProjectClient {
        result: Ok(sample_project()),
        list_result: Ok(vec![sample_project()]),
    };

    let result = handle_project_list(
        10,
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(result.is_ok());

    let output = io.output();
    assert!(
        output.contains('\n'),
        "project list JSON should be pretty when LINEAR_CLI_JSON_STYLE=pretty is provided by config"
    );
    assert!(output.contains("\"name\": \"Backend Services\""));
}

#[test]
fn test_project_view_uses_config_provider_json_style_override() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test-token".to_string());
    values.insert("LINEAR_CLI_JSON_STYLE".to_string(), "pretty".to_string());

    let config = TestConfigProvider { values };
    let storage = TestStorage;
    let io = CapturingIo::new();

    let client = MockProjectClient {
        result: Ok(sample_project()),
        list_result: Ok(vec![]),
    };

    let result = handle_project_view(
        "project-1",
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(result.is_ok());

    let output = io.output();
    assert!(
        output.contains('\n'),
        "project view JSON should be pretty when LINEAR_CLI_JSON_STYLE=pretty is provided by config"
    );
    assert!(output.contains("\"name\": \"Backend Services\""));
}
