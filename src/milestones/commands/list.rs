use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::MilestoneClient;
use crate::error::CliError;
use crate::io::Io;
use crate::milestones::resolver::MilestoneReferenceLookup;
use crate::milestones::types::MilestoneList;
use crate::output::{format_output_to_writer, get_format_with_provider, OutputFormat};
use secrecy::ExposeSecret;

pub fn handle_list(project: Option<&str>, limit: usize, client: &dyn MilestoneClient, lookup: &dyn MilestoneReferenceLookup, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let project_id = match project {
        Some(project) => Some(resolve_project_id(token.expose_secret(), project, lookup)?),
        None => None,
    };
    let milestones = client.list_milestones(token.expose_secret(), project_id.as_deref(), None, limit)?;
    let mut output = Vec::new();
    format_output_to_writer(&MilestoneList(milestones), get_format_with_provider(format_flag, config), &mut output)?;
    io.print_bytes(&output);
    Ok(())
}

pub(crate) fn resolve_project_id(token: &str, project: &str, lookup: &dyn MilestoneReferenceLookup) -> Result<String, CliError> {
    if is_uuid_like(project) {
        return Ok(project.to_string());
    }
    lookup.resolve_project_id_by_slug(token, project)?.ok_or_else(|| CliError::NotFound(format!("project not found for slug: {project}")))
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36 && bytes.iter().enumerate().all(|(idx, byte)| {
        if matches!(idx, 8 | 13 | 18 | 23) { *byte == b'-' } else { byte.is_ascii_hexdigit() }
    })
}
