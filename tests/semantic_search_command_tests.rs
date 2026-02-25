use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::semantic_search::MockSemanticSearchClient;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::search::commands::search::handle_semantic_search;
use linear_cli::search::types::SemanticSearchResult;
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
fn test_semantic_search_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockSemanticSearchClient {
        search_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_semantic_search("test", None, None, &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_semantic_search_returns_results() {
    let results = vec![
        SemanticSearchResult {
            id: "r1".to_string(),
            result_type: "issue".to_string(),
            title: "Auth bug".to_string(),
            identifier: Some("ENG-123".to_string()),
            url: Some("https://linear.app/issue/ENG-123".to_string()),
        },
        SemanticSearchResult {
            id: "r2".to_string(),
            result_type: "project".to_string(),
            title: "Platform".to_string(),
            identifier: None,
            url: None,
        },
    ];
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockSemanticSearchClient {
        search_result: Ok(results),
    };
    let io = CapturingIo::new();

    let result = handle_semantic_search("auth", None, None, &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("ENG-123") || output.contains("Auth bug"));
}

#[test]
fn test_semantic_search_empty_results() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockSemanticSearchClient {
        search_result: Ok(vec![]),
    };
    let io = CapturingIo::new();

    let result = handle_semantic_search(
        "nonexistent",
        None,
        None,
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
}
