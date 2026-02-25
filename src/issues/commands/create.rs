use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::{CreateIssueInput, IssueClient};
use crate::error::CliError;
use crate::io::Io;
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
    priority: Option<i32>,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let created = client.create_issue(
        token.expose_secret(),
        CreateIssueInput {
            team_id: team.to_string(),
            title: title.to_string(),
            description,
            assignee_id: assignee,
            project_id: project,
            state_id: state,
            priority,
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
