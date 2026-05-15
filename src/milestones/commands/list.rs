use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::MilestoneClient;
use crate::error::CliError;
use crate::io::Io;
use crate::milestones::types::MilestoneList;
use crate::output::{format_output_to_writer, get_format_with_provider, OutputFormat};
use secrecy::ExposeSecret;

pub fn handle_list(project_id: Option<&str>, limit: usize, client: &dyn MilestoneClient, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let milestones = client.list_milestones(token.expose_secret(), project_id, None, limit)?;
    let mut output = Vec::new();
    format_output_to_writer(&MilestoneList(milestones), get_format_with_provider(format_flag, config), &mut output)?;
    io.print_bytes(&output);
    Ok(())
}
