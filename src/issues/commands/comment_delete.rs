use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::comments::CommentClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the comment delete command
pub fn handle_comment_delete(
    comment_id: &str,
    client: &dyn CommentClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    client.delete_comment(token.expose_secret(), comment_id)?;

    let format = get_format_with_provider(format_flag, config);
    let msg = if matches!(format, OutputFormat::Json) {
        serde_json::json!({
            "deleted": true,
            "id": comment_id,
        })
        .to_string()
    } else {
        format!("Deleted comment {comment_id}")
    };
    io.print(&msg);

    Ok(())
}
