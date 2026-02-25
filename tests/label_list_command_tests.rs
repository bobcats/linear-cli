use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::labels::MockLabelClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::labels::commands::list::handle_list;
use linear_cli::labels::types::IssueLabel;
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

#[test]
fn test_label_list_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockLabelClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_label_list_returns_labels_on_success() {
    let labels = vec![
        IssueLabel {
            id: "label-1".to_string(),
            name: "Bug".to_string(),
            color: "#eb5757".to_string(),
            description: None,
            is_group: false,
            parent_name: None,
        },
        IssueLabel {
            id: "label-2".to_string(),
            name: "Feature".to_string(),
            color: "#4ea7fc".to_string(),
            description: None,
            is_group: false,
            parent_name: None,
        },
    ];
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockLabelClient {
        list_result: Ok(labels),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("Bug"));
    assert!(output.contains("Feature"));
}

#[test]
fn test_label_list_returns_empty_list() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockLabelClient {
        list_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_list(50, None, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
}
