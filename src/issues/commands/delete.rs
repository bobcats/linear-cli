use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::issues::IssueClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the issue delete command
pub fn handle_delete(
    identifier: &str,
    permanently: bool,
    client: &dyn IssueClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let token_str = token.expose_secret();

    // Resolve identifier to UUID
    let issue = client.get_issue(token_str, identifier)?;

    // Delete the issue
    client.delete_issue(token_str, &issue.id, permanently)?;

    let format = get_format_with_provider(format_flag, config);
    let msg = if matches!(format, OutputFormat::Json) {
        serde_json::json!({
            "deleted": true,
            "identifier": issue.identifier,
            "id": issue.id,
        })
        .to_string()
    } else {
        format!("Deleted issue {}", issue.identifier)
    };
    io.print(&msg);

    Ok(())
}
