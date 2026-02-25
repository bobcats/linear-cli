use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::cycles::CycleClient;
use crate::cycles::types::{Cycle, CycleList};
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle `cycle view <id>` command
pub fn handle_view(
    id: &str,
    client: &dyn CycleClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch cycle from API
    let cycle: Cycle = client.get_cycle(token.expose_secret(), id)?;

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&cycle, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}

/// Handle `cycle list` command
pub fn handle_list(
    limit: usize,
    client: &dyn CycleClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch cycles from API
    let cycles: Vec<Cycle> = client.list_cycles(token.expose_secret(), limit)?;
    let cycle_list = CycleList(cycles);

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&cycle_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}

/// Handle `cycle current` command
pub fn handle_current(
    client: &dyn CycleClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch cycles from API (limited set to find active one)
    let cycles: Vec<Cycle> = client.list_cycles(token.expose_secret(), 50)?;

    // Find the active cycle
    let active_cycle = cycles
        .into_iter()
        .find(|c| c.is_active)
        .ok_or_else(|| CliError::NotFound("No active cycle found".to_string()))?;

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&active_cycle, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
