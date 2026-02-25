use crate::auth::UserInfo;
use crate::error::CliError;
use serde::{Deserialize, Serialize};

/// Keyring service name for storing credentials
const KEYRING_SERVICE: &str = "linear-cli";
/// Keyring username/entry name
const KEYRING_USERNAME: &str = "api-token";

/// Stored authentication data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct AuthData {
    token: String,
    user_info: Option<UserInfo>,
}

/// Trait for token storage operations
pub trait TokenStorage: Send + Sync {
    /// Get the stored token
    fn get_token(&self) -> Result<Option<String>, CliError>;

    /// Get cached user info
    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError>;

    /// Store token and user info together
    fn store_auth(&self, token: &str, user_info: &UserInfo) -> Result<(), CliError>;

    /// Delete all stored auth data
    fn delete(&self) -> Result<(), CliError>;
}

/// Production implementation using system keyring
pub struct KeyringStorage {
    entry: keyring::Entry,
}

impl KeyringStorage {
    pub fn new() -> Result<Self, CliError> {
        let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
            .map_err(CliError::keyring_error)?;
        Ok(Self { entry })
    }

    /// Helper method to retrieve and parse stored auth data
    fn get_auth_data(&self) -> Result<Option<AuthData>, CliError> {
        match self.entry.get_password() {
            Ok(stored) => {
                let auth_data = serde_json::from_str::<AuthData>(&stored).map_err(|e| {
                    CliError::General(format!("Failed to parse stored auth data: {}", e))
                })?;
                Ok(Some(auth_data))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CliError::keyring_error(e)),
        }
    }
}

impl TokenStorage for KeyringStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.get_auth_data()?.map(|auth| auth.token))
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(self.get_auth_data()?.and_then(|auth| auth.user_info))
    }

    fn store_auth(&self, token: &str, user_info: &UserInfo) -> Result<(), CliError> {
        let auth_data = AuthData {
            token: token.to_string(),
            user_info: Some(user_info.clone()),
        };
        let json = serde_json::to_string(&auth_data)
            .map_err(|e| CliError::General(format!("Failed to serialize auth data: {}", e)))?;
        self.entry
            .set_password(&json)
            .map_err(CliError::keyring_error)
    }

    fn delete(&self) -> Result<(), CliError> {
        self.entry
            .delete_credential()
            .map_err(CliError::keyring_error)
    }
}

/// Test implementation using in-memory storage
/// Only available when building tests or with test-utils feature
pub struct MockTokenStorage {
    pub token: Option<String>,
    pub user_info: Option<UserInfo>,
}

impl Default for MockTokenStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MockTokenStorage {
    #[must_use]
    pub fn new() -> Self {
        Self {
            token: None,
            user_info: None,
        }
    }

    #[must_use]
    pub fn with_token(token: String) -> Self {
        Self {
            token: Some(token),
            user_info: None,
        }
    }
}

impl TokenStorage for MockTokenStorage {
    fn get_token(&self) -> Result<Option<String>, CliError> {
        Ok(self.token.clone())
    }

    fn get_user_info(&self) -> Result<Option<UserInfo>, CliError> {
        Ok(self.user_info.clone())
    }

    fn store_auth(&self, _token: &str, _user_info: &UserInfo) -> Result<(), CliError> {
        Ok(())
    }

    fn delete(&self) -> Result<(), CliError> {
        Ok(())
    }
}
