use linear_cli::auth::config::TestConfigProvider;
use linear_cli::auth::storage::{MockTokenStorage, TokenStorage};
use linear_cli::auth::token::get_token_with_provider;
use linear_cli::client::auth::UserInfo;
use linear_cli::error::CliError;
use secrecy::ExposeSecret;
use std::collections::HashMap;

struct ErrorStorage {
    error: CliError,
}

impl TokenStorage for ErrorStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Err(self.error.clone())
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

#[test]
fn test_linear_token_env_var_takes_precedence() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "token_from_env".to_string());
    values.insert("LINEAR_API_TOKEN".to_string(), "other_token".to_string());

    let config = TestConfigProvider { values };
    let storage = MockTokenStorage::with_token("keyring_token".to_string());

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token.expose_secret(), "token_from_env");
}

#[test]
fn test_linear_api_token_env_var_used_when_linear_token_not_set() {
    let mut values = HashMap::new();
    values.insert(
        "LINEAR_API_TOKEN".to_string(),
        "api_token_from_env".to_string(),
    );

    let config = TestConfigProvider { values };
    let storage = MockTokenStorage::with_token("keyring_token".to_string());

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token.expose_secret(), "api_token_from_env");
}

#[test]
fn test_keyring_used_when_no_env_vars() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::with_token("keyring_token".to_string());

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_ok());
    let token = result.unwrap();
    assert_eq!(token.expose_secret(), "keyring_token");
}

#[test]
fn test_missing_token_returns_auth_error() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = MockTokenStorage::new();

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, CliError::AuthError(_)));
    }
}

#[test]
fn test_token_wrapped_in_secret_string_not_visible_in_debug() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "secret_token_12345".to_string());

    let config = TestConfigProvider { values };
    let storage = MockTokenStorage::new();

    let result = get_token_with_provider(&config, &storage);

    if let Ok(token) = result {
        let debug_output = format!("{:?}", token);
        // Secret should be redacted in debug output
        assert!(!debug_output.contains("secret_token_12345"));
        assert!(debug_output.contains("Secret"));
    }
}

#[test]
fn test_keyring_read_error_is_propagated_when_no_env_token() {
    let config = TestConfigProvider {
        values: HashMap::new(),
    };
    let storage = ErrorStorage {
        error: CliError::AuthError("Keyring unavailable".to_string()),
    };

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(CliError::AuthError(ref message)) if message == "Keyring unavailable"
    ));
}

#[test]
fn test_env_token_bypasses_keyring_read_error() {
    let mut values = HashMap::new();
    values.insert("LINEAR_TOKEN".to_string(), "token_from_env".to_string());

    let config = TestConfigProvider { values };
    let storage = ErrorStorage {
        error: CliError::AuthError("Keyring unavailable".to_string()),
    };

    let result = get_token_with_provider(&config, &storage);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().expose_secret(), "token_from_env");
}
