use linear_cli::auth::UserInfo;
use linear_cli::auth::config::ConfigProvider;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::comments::{CommentClient, CreateCommentInput};
use linear_cli::comments::types::Comment;
use linear_cli::error::CliError;
use linear_cli::io::Io;
use linear_cli::issues::commands::comment_add::handle_comment_add;
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
struct MockCreateCommentClient {
    create_result: Result<Comment, CliError>,
}

impl CommentClient for MockCreateCommentClient {
    fn list_comments(
        &self,
        _token: &str,
        _issue_id: &str,
        _limit: usize,
    ) -> Result<Vec<Comment>, CliError> {
        unreachable!("not used")
    }

    fn create_comment(
        &self,
        _token: &str,
        _input: CreateCommentInput,
    ) -> Result<Comment, CliError> {
        self.create_result.clone()
    }
}

fn sample_comment() -> Comment {
    Comment {
        id: "comment-1".to_string(),
        body: "Looks good to me".to_string(),
        user_name: "Alice".to_string(),
        user_email: "alice@example.com".to_string(),
        created_at: "2026-02-24T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        edited_at: None,
        issue_identifier: Some("ENG-123".to_string()),
    }
}

#[test]
fn test_comment_add_outputs_created_comment_on_success() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateCommentClient {
        create_result: Ok(sample_comment()),
    };

    let result = handle_comment_add(
        "ENG-123",
        "Looks good to me",
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_ok());
    let output = io.stdout_lines().join("\n");
    assert!(output.contains("Looks good to me"));
    assert!(output.contains("ENG-123"));
}

#[test]
fn test_comment_add_returns_auth_error_when_no_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateCommentClient {
        create_result: Ok(sample_comment()),
    };

    let result = handle_comment_add(
        "ENG-123",
        "Looks good",
        &client,
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
fn test_comment_add_propagates_not_found_errors() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "test_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockStorage { token: None };
    let io = CapturingIo::new();
    let client = MockCreateCommentClient {
        create_result: Err(CliError::NotFound("Issue ENG-999 not found".to_string())),
    };

    let result = handle_comment_add(
        "ENG-999",
        "Looks good",
        &client,
        &config,
        &storage,
        &io,
        None,
    );

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => assert!(msg.contains("not found")),
        _ => panic!("expected NotFound"),
    }
}
