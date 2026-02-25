use crate::auth::UserInfo;
use crate::auth::storage::TokenStorage;
use crate::client::auth::AuthClient;
use crate::error::CliError;
use crate::io::Io;
use secrecy::{ExposeSecret, SecretString};

/// Handle the login command
pub fn handle_login(
    token_input: Option<SecretString>,
    api_client: &dyn AuthClient,
    storage: &dyn TokenStorage,
    io: &dyn Io,
) -> Result<UserInfo, CliError> {
    // Get token from input or prompt user
    let token = match token_input {
        Some(t) => t,
        None => {
            io.print("Paste your Linear API token (get it from Settings > API):");
            let token_str = io.read_secret("> ")?;
            SecretString::from(token_str)
        }
    };

    if token.expose_secret().is_empty() {
        return Err(CliError::auth_error("Token cannot be empty"));
    }

    // Validate with API
    let user_info = api_client.validate_token(token.expose_secret())?;

    // Store token and user info in keyring
    storage.store_auth(token.expose_secret(), &user_info)?;

    // Success message
    io.print(&format!(
        "✓ Successfully authenticated as {} ({})",
        user_info.name, user_info.email
    ));
    io.print("  Token and user info stored securely in system keyring");

    Ok(user_info)
}
