use crate::auth::config::ConfigProvider;
use crate::auth::output::{AuthStatus, TokenSource};
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::auth::AuthClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the status command
pub fn handle_status(
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    api_client: &dyn AuthClient,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get token
    let token = get_token_with_provider(config, storage)?;

    // Try to get cached user info first (avoids API call)
    let user_info = match storage.get_user_info()? {
        Some(info) => info,
        None => {
            // No cached info, validate with API and cache it
            let info = api_client.validate_token(token.expose_secret())?;
            storage.store_auth(token.expose_secret(), &info)?;
            info
        }
    };

    // Determine token source
    let source = if config.get_var("LINEAR_TOKEN").is_some() {
        TokenSource::LinearToken
    } else if config.get_var("LINEAR_API_TOKEN").is_some() {
        TokenSource::LinearApiToken
    } else {
        TokenSource::Keyring
    };

    // Always redact token - use `linear auth token` if you need the raw value
    let token_str = token.expose_secret();
    let preview_len = token_str.len().min(10);
    let token_display = format!("{}***", &token_str[..preview_len]);

    // Create AuthStatus output
    let status = AuthStatus {
        logged_in: true,
        user_name: user_info.name,
        user_email: user_info.email,
        token: token_display,
        token_source: source,
        show_full_token: false,
    };

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output
    let output = format_output(&status, format)?;
    io.print(&output);

    Ok(())
}
