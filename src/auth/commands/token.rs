use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::error::CliError;
use crate::io::Io;
use secrecy::ExposeSecret;

/// Handle the token command - print raw token for scripting
pub fn handle_token(
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    io.print(token.expose_secret());
    Ok(())
}
