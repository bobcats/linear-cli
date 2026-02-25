use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::IssueClient;
use crate::error::CliError;
use crate::io::Io;
use crate::issues::types::IssueList;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the issue list command
#[allow(clippy::too_many_arguments)]
pub fn handle_list(
    assignee: Option<String>,
    project: Option<String>,
    limit: usize,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch issues from API
    let issues = client.list_issues(token.expose_secret(), assignee, project, limit)?;

    // Wrap in IssueList for collection-specific formatting
    let issue_list = IssueList(issues);

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&issue_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
