use crate::auth::{config::ConfigProvider, storage::TokenStorage, token::get_token_with_provider};
use crate::client::milestones::MilestoneClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{get_format_with_provider, resolve_json_style_with_provider, JsonStyle, OutputFormat};
use secrecy::ExposeSecret;
use serde::Serialize;

#[derive(Serialize)]
struct DeleteOutput<'a> { deleted: bool, id: &'a str }

pub fn handle_delete(id: &str, client: &dyn MilestoneClient, config: &dyn ConfigProvider, storage: &dyn TokenStorage, io: &dyn Io, format_flag: Option<OutputFormat>) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    client.delete_milestone(token.expose_secret(), id)?;
    if matches!(get_format_with_provider(format_flag, config), OutputFormat::Json) {
        let payload = DeleteOutput { deleted: true, id };
        let output = match resolve_json_style_with_provider(config) { JsonStyle::Compact => serde_json::to_vec(&payload), JsonStyle::Pretty => serde_json::to_vec_pretty(&payload) }
            .map_err(|e| CliError::General(format!("Failed to serialize delete response: {e}")))?;
        io.print_bytes(&output);
    } else {
        io.print(&format!("Deleted milestone {id}"));
    }
    Ok(())
}
