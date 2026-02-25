use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::states::MockStateClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::states::commands::list::handle_list;
use linear_cli::states::types::WorkflowState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

fn config_with_token() -> TestConfigProvider {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    TestConfigProvider { values }
}

fn make_state(name: &str, state_type: &str) -> WorkflowState {
    WorkflowState {
        id: format!("state-{}", name.to_lowercase().replace(' ', "-")),
        name: name.to_string(),
        state_type: state_type.to_string(),
        color: "#bec2c8".to_string(),
        position: 1.0,
        description: None,
        team_name: None,
    }
}

#[test]
fn test_state_list_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockStateClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_state_list_returns_empty_list() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockStateClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
}

#[test]
fn test_state_list_returns_states_on_success() {
    let states = vec![
        make_state("Backlog", "backlog"),
        make_state("In Progress", "started"),
    ];
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockStateClient {
        list_result: Ok(states),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("Backlog"));
    assert!(output.contains("In Progress"));
}

#[test]
fn test_state_list_propagates_client_error() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockStateClient {
        list_result: Err(CliError::NetworkError("connection failed".to_string())),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}
