use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::IssueClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};
use secrecy::ExposeSecret;

pub fn handle_archive(
    identifier: &str,
    trash: bool,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let issue = client.archive_issue(token.expose_secret(), identifier, trash)?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&issue, format)?;
    io.print(&output);

    Ok(())
}

pub fn handle_unarchive(
    identifier: &str,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let issue = client.unarchive_issue(token.expose_secret(), identifier)?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&issue, format)?;
    io.print(&output);

    Ok(())
}
