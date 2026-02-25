use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::search::SearchClient;
use crate::error::CliError;
use crate::io::Io;
use crate::issues::types::IssueList;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Search parameters
pub struct SearchParams<'a> {
    pub term: &'a str,
    pub team_id: Option<&'a str>,
    pub include_comments: bool,
    pub limit: usize,
}

/// Handle the issue search command
#[allow(clippy::too_many_arguments)]
pub fn handle_search(
    term: &str,
    team_id: Option<&str>,
    include_comments: bool,
    limit: usize,
    client: &dyn SearchClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let issues = client.search_issues(
        token.expose_secret(),
        term,
        team_id,
        include_comments,
        limit,
    )?;
    let issue_list = IssueList(issues);
    let format = get_format_with_provider(format_flag, config);

    let mut output = Vec::new();
    format_output_to_writer(&issue_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
