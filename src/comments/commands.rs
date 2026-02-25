use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::comments::CommentClient;
use crate::comments::types::CommentList;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle `issue comments <issue-id>` command
pub fn handle_list(
    client: &dyn CommentClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    issue_id: &str,
    limit: usize,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(config, storage)?;

    // Fetch comments from API
    let comments = client.list_comments(token.expose_secret(), issue_id, limit)?;

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output via streaming writer API
    let comment_list = CommentList(comments);
    let mut output = Vec::new();
    format_output_to_writer(&comment_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
