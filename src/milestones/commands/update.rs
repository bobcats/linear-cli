use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::{MilestoneClient, UpdateMilestoneInput};
use crate::error::CliError;
use crate::io::Io;
use crate::milestones::commands::list::resolve_project_id;
use crate::milestones::resolver::{MilestoneReferenceLookup, MilestoneReferenceResolver};
use crate::output::{format_output_to_writer, get_format_with_provider, OutputFormat};
use secrecy::ExposeSecret;

pub fn handle_update(reference: &str, project: Option<&str>, mut input: UpdateMilestoneInput, client: &dyn MilestoneClient, lookup: &dyn MilestoneReferenceLookup, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    if input.name.is_none() && input.description.is_none() && input.project_id.is_none() && input.target_date.is_none() && project.is_none() {
        return Err(CliError::InvalidArgs("at least one milestone field must be provided".to_string()));
    }
    let token = get_token_with_provider(config, storage)?;
    let resolver = MilestoneReferenceResolver::new(lookup);
    let id = resolver.resolve_required_id(token.expose_secret(), reference, project.map(str::to_string))?;
    if let Some(project) = project {
        input.project_id = Some(resolve_project_id(token.expose_secret(), project, lookup)?);
    }
    let milestone = client.update_milestone(token.expose_secret(), &id, input)?;
    let mut output = Vec::new();
    format_output_to_writer(&milestone, get_format_with_provider(format_flag, config), &mut output)?;
    io.print_bytes(&output);
    Ok(())
}
