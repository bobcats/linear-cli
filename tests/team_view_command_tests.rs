use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::teams::MockTeamClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::teams::commands::view::handle_view;
use linear_cli::teams::types::Team;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Mock storage for testing
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

// Capturing IO for testing
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

    fn print_error(&self, _message: &str) {
        // Ignore for now
    }
}

#[test]
fn test_view_returns_team_details_for_valid_id() {
    let team = Team {
        id: "team-123".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    };

    let mut config_values = HashMap::new();
    config_values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    let config = TestConfigProvider {
        values: config_values,
    };

    let storage = MockStorage { token: None };

    let client = MockTeamClient {
        result: Ok(team.clone()),
        list_result: Ok(vec![]),
    };

    let io = CapturingIo::new();

    let result = handle_view("team-123", &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines();

    // Verify team details are in output
    assert!(!output.is_empty());
    let full_output = output.join("\n");
    assert!(full_output.contains("ENG"));
    assert!(full_output.contains("Engineering"));
}
