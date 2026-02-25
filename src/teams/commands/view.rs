use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::teams::TeamClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the team view command
pub fn handle_view(
    id: &str,
    client: &dyn TeamClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch team from API
    let team = client.get_team(token.expose_secret(), id)?;

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&team, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
