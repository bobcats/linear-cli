use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::comments::{CommentClient, CreateCommentInput};
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};
use secrecy::ExposeSecret;

pub fn handle_comment_add(
    identifier: &str,
    body: &str,
    client: &dyn CommentClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let comment = client.create_comment(
        token.expose_secret(),
        CreateCommentInput {
            issue_id: identifier.to_string(),
            body: body.to_string(),
        },
    )?;

    let format = get_format_with_provider(format_flag, config);
    let output = format_output(&comment, format)?;
    io.print(&output);

    Ok(())
}
