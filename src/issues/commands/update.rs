use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{IssueClient, UpdateIssueInput};
use crate::error::CliError;
use crate::io::Io;
use crate::issues::resolver::{
    IssueReferenceLookup, IssueReferenceResolver, ResolveIssueRefsInput,
};
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
    parent: Option<String>,
    priority: Option<i32>,
    client: &dyn IssueClient,
    lookup: &dyn IssueReferenceLookup,
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
        && parent.is_none()
        && priority.is_none()
    {
        return Err(CliError::InvalidArgs(
            "issue update requires at least one patch field".to_string(),
        ));
    }

    let token = get_token_with_provider(config, storage)?;

    let resolver = IssueReferenceResolver::new(lookup);
    let resolved = resolver.resolve(
        token.expose_secret(),
        &ResolveIssueRefsInput {
            team: None,
            assignee,
            project,
            state,
            parent,
        },
    )?;

    let updated = client.update_issue(
        token.expose_secret(),
        identifier,
        UpdateIssueInput {
            title,
            description,
            assignee_id: resolved.assignee_id,
            project_id: resolved.project_id,
            state_id: resolved.state_id,
            priority,
            parent_id: resolved.parent_id,
        },
    )?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&updated, format)?;
    io.print(&output);

    Ok(())
}
