use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{IssueClient, UpdateIssueInput};
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};
use secrecy::ExposeSecret;

#[allow(clippy::too_many_arguments)]
pub fn handle_update(
    identifier: &str,
    title: Option<String>,
    description: Option<String>,
    assignee: Option<String>,
    project: Option<String>,
    state: Option<String>,
    priority: Option<i32>,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    if title.is_none()
        && description.is_none()
        && assignee.is_none()
        && project.is_none()
        && state.is_none()
        && priority.is_none()
    {
        return Err(CliError::InvalidArgs(
            "issue update requires at least one patch field".to_string(),
        ));
    }

    let token = get_token_with_provider(config, storage)?;

    let updated = client.update_issue(
        token.expose_secret(),
        identifier,
        UpdateIssueInput {
            title,
            description,
            assignee_id: assignee,
            project_id: project,
            state_id: state,
            priority,
        },
    )?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&updated, format)?;
    io.print(&output);

    Ok(())
}
