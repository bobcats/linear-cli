use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::projects::ProjectClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{
    JsonStyle, OutputFormat, format_output_to_writer, get_format_with_provider,
    resolve_json_style_with_provider,
};
use crate::projects::types::ProjectList;
use secrecy::ExposeSecret;

/// Handle the project list command
pub fn handle_list(
    limit: usize,
    client: &dyn ProjectClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch projects from API
    let projects = client.list_projects(token.expose_secret(), limit)?;

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Hotspot optimization #1: project list JSON fast-path.
    // Resolve JSON style once from injected config, then serialize directly to bytes
    // to avoid intermediate String allocations.
    if matches!(format, OutputFormat::Json) {
        let style = resolve_json_style_with_provider(config);
        let output = match style {
            JsonStyle::Compact => serde_json::to_vec(&projects),
            JsonStyle::Pretty => serde_json::to_vec_pretty(&projects),
        }
        .map_err(|e| CliError::General(format!("Failed to serialize projects to JSON: {e}")))?;

        io.print_bytes(&output);
        return Ok(());
    }

    // Non-JSON formats continue through formatter infrastructure.
    let project_list = ProjectList(projects);
    let mut output = Vec::new();
    format_output_to_writer(&project_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
