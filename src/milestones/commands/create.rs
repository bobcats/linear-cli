use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::{CreateMilestoneInput, MilestoneClient};
use crate::error::CliError;
use crate::io::Io;
use crate::milestones::commands::list::resolve_project_id;
use crate::milestones::resolver::MilestoneReferenceLookup;
use crate::output::{format_output_to_writer, get_format_with_provider, OutputFormat};
use secrecy::ExposeSecret;

pub fn handle_create(project: &str, name: &str, description: Option<String>, target_date: Option<String>, client: &dyn MilestoneClient, lookup: &dyn MilestoneReferenceLookup, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let project_id = resolve_project_id(token.expose_secret(), project, lookup)?;
    let milestone = client.create_milestone(token.expose_secret(), CreateMilestoneInput { project_id, name: name.to_string(), description, target_date })?;
    let mut output = Vec::new();
    format_output_to_writer(&milestone, get_format_with_provider(format_flag, config), &mut output)?;
    io.print_bytes(&output);
    Ok(())
}
