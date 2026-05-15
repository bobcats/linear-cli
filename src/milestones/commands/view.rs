use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::MilestoneClient;
use crate::error::CliError;
use crate::io::Io;
use crate::milestones::resolver::{MilestoneReferenceLookup, MilestoneReferenceResolver};
use crate::output::{format_output_to_writer, get_format_with_provider, OutputFormat};
use secrecy::ExposeSecret;

pub fn handle_view(reference: &str, project: Option<&str>, client: &dyn MilestoneClient, lookup: &dyn MilestoneReferenceLookup, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let resolver = MilestoneReferenceResolver::new(lookup);
    let id = resolver.resolve_required_id(token.expose_secret(), reference, project.map(str::to_string))?;
    let milestone = client.get_milestone(token.expose_secret(), &id)?;
    let mut output = Vec::new();
    format_output_to_writer(&milestone, get_format_with_provider(format_flag, config), &mut output)?;
    io.print_bytes(&output);
    Ok(())
}
