use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{CreateIssueRelationInput, IssueClient};
use crate::client::queries::IssueRelationType;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};
use secrecy::ExposeSecret;

#[allow(clippy::too_many_arguments)]
fn handle_relation(
    identifier: &str,
    related: &str,
    relation_type: IssueRelationType,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let issue = client.create_issue_relation(
        token.expose_secret(),
        CreateIssueRelationInput {
            issue_id: identifier.to_string(),
            related_issue_id: related.to_string(),
            relation_type,
        },
    )?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&issue, format)?;
    io.print(&output);

    Ok(())
}

pub fn handle_link(
    identifier: &str,
    related: &str,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    handle_relation(
        identifier,
        related,
        IssueRelationType::Related,
        client,
        config,
        storage,
        io,
        format_flag,
    )
}

pub fn handle_block(
    identifier: &str,
    related: &str,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    handle_relation(
        identifier,
        related,
        IssueRelationType::Blocks,
        client,
        config,
        storage,
        io,
        format_flag,
    )
}

pub fn handle_duplicate(
    identifier: &str,
    related: &str,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    handle_relation(
        identifier,
        related,
        IssueRelationType::Duplicate,
        client,
        config,
        storage,
        io,
        format_flag,
    )
}
