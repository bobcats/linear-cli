use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::comments::MockCommentClient;
use linear_cli::comments::types::Comment;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::comment_delete::handle_comment_delete;
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

fn dummy_comment() -> Comment {
    Comment {
        id: "comment-1".to_string(),
        body: "dummy".to_string(),
        user_name: "Test".to_string(),
        user_email: "test@test.com".to_string(),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        edited_at: None,
        issue_identifier: None,
    }
}

#[test]
fn test_comment_delete_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Ok(dummy_comment()),
        delete_result: Ok(()),
    };
    let io = CapturingIo::new();

    let result = handle_comment_delete("comment-123", &client, &config, &storage, &io, None);

    assert!(result.is_err());
}

#[test]
fn test_comment_delete_succeeds() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Ok(dummy_comment()),
        delete_result: Ok(()),
    };
    let io = CapturingIo::new();

    let result = handle_comment_delete("comment-123", &client, &config, &storage, &io, None);

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("deleted") || output.contains("comment-123"));
}

#[test]
fn test_comment_delete_propagates_error() {
    let config = config_with_token();
    let storage = MockStorage { token: None };
    let client = MockCommentClient {
        list_result: Ok(vec![]),
        create_result: Ok(dummy_comment()),
        delete_result: Err(CliError::NotFound("Comment not found".to_string())),
    };
    let io = CapturingIo::new();

    let result = handle_comment_delete("comment-999", &client, &config, &storage, &io, None);

    assert!(result.is_err());
}
