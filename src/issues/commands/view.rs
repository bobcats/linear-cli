use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::comments::CommentClient;
use crate::client::issues::IssueClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Dependencies for the issue view command
pub struct ViewDeps<'a> {
    pub issue_client: &'a dyn IssueClient,
    pub comment_client: &'a dyn CommentClient,
    pub config: &'a dyn ConfigProvider,
    pub storage: &'a dyn TokenStorage,
    pub io: &'a dyn Io,
}

/// Handle the issue view command
pub fn handle_view(
    identifier: &str,
    with_comments: bool,
    comment_limit: usize,
    deps: &ViewDeps,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    // Get authentication token
    let token = get_token_with_provider(deps.config, deps.storage)?;

    // Fetch issue from API
    let mut issue = deps
        .issue_client
        .get_issue(token.expose_secret(), identifier)?;

    // Optionally fetch and attach comments
    if with_comments {
        let comments =
            deps.comment_client
                .list_comments(token.expose_secret(), &issue.id, comment_limit)?;
        issue.comments = Some(comments);
    }

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, deps.config);

    // Format and output via streaming writer API
    let mut output = Vec::new();
    format_output_to_writer(&issue, format, &mut output)?;
    deps.io.print_bytes(&output);

    Ok(())
}
