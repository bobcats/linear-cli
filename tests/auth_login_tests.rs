use linear_cli::auth::commands::login::handle_login;
use linear_cli::auth::storage::TokenStorage;
use linear_cli::client::auth::{MockAuthClient, UserInfo};
use linear_cli::error::CliError;
use linear_cli::io::MockIo;
use secrecy::SecretString;
use std::sync::{Arc, Mutex};

// Stored auth data for testing
#[derive(Clone)]
struct StoredAuth {
    token: Option<String>,
    user_info: Option<UserInfo>,
}

// Helper to create a mock storage that captures stored tokens and user info
struct CapturingMockStorage {
    data: Arc<Mutex<StoredAuth>>,
    get_result: Option<String>,
}

impl CapturingMockStorage {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(StoredAuth {
                token: None,
                user_info: None,
            })),
            get_result: None,
        }
    }

    fn stored_token(&self) -> Option<String> {
        self.data.lock().unwrap().token.clone()
    }
}

impl TokenStorage for CapturingMockStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.get_result.clone())
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(self.data.lock().unwrap().user_info.clone())
    }

    fn store_auth(&self, token: &str, user_info: &UserInfo) -> Result<(), CliError> {
        // Single lock ensures atomicity - both fields updated together
        let mut data = self.data.lock().unwrap();
        data.token = Some(token.to_string());
        data.user_info = Some(user_info.clone());
        Ok(())
    }

    fn delete(&self) -> Result<(), CliError> {
        // Single lock ensures atomicity - both fields deleted together
        let mut data = self.data.lock().unwrap();
        data.token = None;
        data.user_info = None;
        Ok(())
    }
}

#[test]
fn test_login_with_valid_token_stores_in_keyring() {
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let storage = CapturingMockStorage::new();
    let io = MockIo::new();

    let result = handle_login(
        Some(SecretString::from("lin_api_test_token".to_string())),
        &api_client,
        &storage,
        &io,
    );

    assert!(result.is_ok());
    assert_eq!(
        storage.stored_token(),
        Some("lin_api_test_token".to_string())
    );
}

#[test]
fn test_login_validates_token_with_api_client() {
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let storage = CapturingMockStorage::new();
    let io = MockIo::new();

    let result = handle_login(
        Some(SecretString::from("lin_api_test_token".to_string())),
        &api_client,
        &storage,
        &io,
    );

    assert!(result.is_ok());
    let user_info = result.unwrap();
    assert_eq!(user_info.name, "Alice");
    assert_eq!(user_info.email, "alice@example.com");
}

#[test]
fn test_login_with_invalid_token_shows_error() {
    let api_client = MockAuthClient {
        result: Err(CliError::AuthError("Invalid token".to_string())),
    };
    let storage = CapturingMockStorage::new();
    let io = MockIo::new();

    let result = handle_login(
        Some(SecretString::from("invalid_token".to_string())),
        &api_client,
        &storage,
        &io,
    );

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::AuthError(_)));
    // Token should not be stored if validation fails
    assert_eq!(storage.stored_token(), None);
}

#[test]
fn test_login_with_no_token_returns_error() {
    let api_client = MockAuthClient {
        result: Ok(UserInfo {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
    };
    let storage = CapturingMockStorage::new();
    let io = MockIo::new();

    let result = handle_login(None, &api_client, &storage, &io);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), CliError::AuthError(_)));
}

#[test]
fn test_login_with_api_validation_failure_does_not_store() {
    let api_client = MockAuthClient {
        result: Err(CliError::NetworkError("Connection failed".to_string())),
    };
    let storage = CapturingMockStorage::new();
    let io = MockIo::new();

    let result = handle_login(
        Some(SecretString::from("lin_api_test_token".to_string())),
        &api_client,
        &storage,
        &io,
    );

    assert!(result.is_err());
    // Should not store token if API validation fails
    assert_eq!(storage.stored_token(), None);
}
