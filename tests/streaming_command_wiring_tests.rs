use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::teams::MockTeamClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::output::OutputFormat;
use linear_cli::teams::commands::{
    handle_list as handle_team_list, handle_view as handle_team_view,
};
use linear_cli::teams::types::Team;
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

struct StreamingExpectationIo {
    print_calls: Arc<Mutex<usize>>,
    stream_calls: Arc<Mutex<usize>>,
}

impl StreamingExpectationIo {
    fn new() -> Self {
        Self {
            print_calls: Arc::new(Mutex::new(0)),
            stream_calls: Arc::new(Mutex::new(0)),
        }
    }

    fn print_calls(&self) -> usize {
        *self.print_calls.lock().expect("mutex poisoned")
    }

    fn stream_calls(&self) -> usize {
        *self.stream_calls.lock().expect("mutex poisoned")
    }
}

impl Io for StreamingExpectationIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok("ignored".to_string())
    }

    fn print(&self, _message: &str) {
        let mut calls = self.print_calls.lock().expect("mutex poisoned");
        *calls += 1;
    }

    fn print_bytes(&self, _bytes: &[u8]) {
        let mut calls = self.stream_calls.lock().expect("mutex poisoned");
        *calls += 1;
    }

    fn print_error(&self, _message: &str) {}
}

fn sample_team() -> Team {
    Team {
        id: "team-1".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#123456".to_string()),
        icon: Some("⚙️".to_string()),
        private: false,
        created_at: "2026-02-24T00:00:00Z".to_string(),
    }
}

fn test_config() -> TestConfigProvider {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());
    TestConfigProvider { values }
}

#[test]
fn test_team_list_uses_streaming_output_path_instead_of_print_string() {
    let config = test_config();
    let storage = TestStorage;
    let io = StreamingExpectationIo::new();
    let team = sample_team();

    let client = MockTeamClient {
        result: Ok(team.clone()),
        list_result: Ok(vec![team]),
    };

    let result = handle_team_list(
        10,
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(result.is_ok());
    assert_eq!(
        io.print_calls(),
        0,
        "team list should use streaming writer path instead of io.print(string)"
    );
    assert_eq!(
        io.stream_calls(),
        1,
        "team list should invoke streaming byte output exactly once"
    );
}

#[test]
fn test_team_view_uses_streaming_output_path_instead_of_print_string() {
    let config = test_config();
    let storage = TestStorage;
    let io = StreamingExpectationIo::new();
    let team = sample_team();

    let client = MockTeamClient {
        result: Ok(team),
        list_result: Ok(vec![]),
    };

    let result = handle_team_view(
        "team-1",
        &client,
        &config,
        &storage,
        &io,
        Some(OutputFormat::Json),
    );

    assert!(result.is_ok());
    assert_eq!(
        io.print_calls(),
        0,
        "team view should use streaming writer path instead of io.print(string)"
    );
    assert_eq!(
        io.stream_calls(),
        1,
        "team view should invoke streaming byte output exactly once"
    );
}
