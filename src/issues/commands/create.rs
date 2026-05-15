use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{CreateIssueInput, IssueClient};
use crate::error::CliError;
use crate::io::Io;
use crate::issues::resolver::{
    IssueReferenceLookup, IssueReferenceResolver, ResolveIssueRefsInput,
};
use crate::milestones::resolver::{
    MilestoneReferenceLookup, MilestoneReferenceResolver, ResolveMilestoneInput,
    ResolvedMilestonePatch,
};
use crate::output::{
    JsonStyle, OutputFormat, format_output, get_format_with_provider,
    resolve_json_style_with_provider,
};
use secrecy::ExposeSecret;

#[allow(clippy::too_many_arguments)]
pub fn handle_create(
    team: &str,
    title: &str,
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
    let token = get_token_with_provider(config, storage)?;

    let resolver = IssueReferenceResolver::new(lookup);
    let raw_project_ref = project.clone();
    let resolved = resolver.resolve(
        token.expose_secret(),
        &ResolveIssueRefsInput {
            team: Some(team.to_string()),
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
            allow_null_clear: false,
        },
    )?;
    let project_milestone_id = match milestone_patch {
        ResolvedMilestonePatch::Unchanged => None,
        ResolvedMilestonePatch::Set(id) => Some(id),
        ResolvedMilestonePatch::Clear => unreachable!("clear is disabled for issue create"),
    };

    let created = client.create_issue(
        token.expose_secret(),
        CreateIssueInput {
            team_id: resolved.team_id.unwrap_or_else(|| team.to_string()),
            title: title.to_string(),
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

    // Hotspot optimization #3: only activate provider-driven JSON style path
    // when style is explicitly provided via injected config.
    // This preserves existing fast default behavior while enabling deterministic
    // style overrides for benchmarked/tested command paths.
    if matches!(format, OutputFormat::Json) && config.get_var("LINEAR_CLI_JSON_STYLE").is_some() {
        let style = resolve_json_style_with_provider(config);
        let output = match style {
            JsonStyle::Compact => serde_json::to_vec(&created),
            JsonStyle::Pretty => serde_json::to_vec_pretty(&created),
        }
        .map_err(|e| CliError::General(format!("Failed to serialize issue to JSON: {e}")))?;

        io.print_bytes(&output);
        return Ok(());
    }

    let output = format_output(&created, format)?;
    io.print(&output);

    Ok(())
}
