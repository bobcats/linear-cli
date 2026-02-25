use crate::error::CliError;
use secrecy::SecretString;

use super::config::{ConfigProvider, EnvConfigProvider};
use super::storage::{KeyringStorage, TokenStorage};

/// Get token from various sources with precedence:
/// 1. LINEAR_TOKEN environment variable
/// 2. LINEAR_API_TOKEN environment variable
/// 3. Keyring storage
///
/// This is the testable version that accepts injected dependencies
pub fn get_token_with_provider(
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
) -> Result<SecretString, CliError> {
    // Precedence: LINEAR_TOKEN > LINEAR_API_TOKEN > Keyring
    config
        .get_var("LINEAR_TOKEN")
        .or_else(|| config.get_var("LINEAR_API_TOKEN"))
        .or_else(|| storage.get_token().ok().flatten())
        .map(SecretString::from)
        .ok_or_else(CliError::no_token)
}

/// Convenience function for production code
/// Uses real environment variables and keyring storage
pub fn get_token() -> Result<SecretString, CliError> {
    let config = EnvConfigProvider;
    let storage = KeyringStorage::new()?;

    get_token_with_provider(&config, &storage)
}
