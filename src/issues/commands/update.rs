use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{IssueClient, IssueFieldPatch, UpdateIssueInput};
use crate::error::CliError;
use crate::io::Io;
use crate::issues::resolver::{
    IssueReferenceLookup, IssueReferenceResolver, ResolveIssueRefsInput,
};
use crate::milestones::resolver::{
    MilestoneReferenceLookup, MilestoneReferenceResolver, ResolveMilestoneInput,
    ResolvedMilestonePatch,
};
use crate::output::{format_output, get_format_with_provider, OutputFormat};
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
    milestone: Option<String>,
    client: &dyn IssueClient,
    lookup: &dyn IssueReferenceLookup,
    milestone_lookup: &dyn MilestoneReferenceLookup,
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
        && milestone.is_none()
    {
        return Err(CliError::InvalidArgs(
            "issue update requires at least one patch field".to_string(),
        ));
    }

    let token = get_token_with_provider(config, storage)?;

    let raw_project_ref = project.clone();
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

    let milestone_scope = if let Some(project_id) = &resolved.project_id {
        Some(project_id.clone())
    } else if let Some(raw_project_ref) = raw_project_ref {
        milestone_lookup.resolve_project_id_by_slug(token.expose_secret(), &raw_project_ref)?
    } else {
        None
    };

    let milestone_patch = MilestoneReferenceResolver::new(milestone_lookup).resolve_patch(
        token.expose_secret(),
        ResolveMilestoneInput {
            reference: milestone,
            project: milestone_scope,
            allow_null_clear: true,
        },
    )?;

    let project_milestone_id = match milestone_patch {
        ResolvedMilestonePatch::Unchanged => IssueFieldPatch::Unchanged,
        ResolvedMilestonePatch::Set(id) => IssueFieldPatch::Set(id),
        ResolvedMilestonePatch::Clear => IssueFieldPatch::Clear,
    };

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
            project_milestone_id,
        },
    )?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&updated, format)?;
    io.print(&output);

    Ok(())
}
