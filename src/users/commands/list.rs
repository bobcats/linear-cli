use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::users::UserClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use crate::users::types::UserList;
use secrecy::ExposeSecret;

/// Handle the user list command
pub fn handle_list(
    limit: usize,
    client: &dyn UserClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let users = client.list_users(token.expose_secret(), limit)?;
    let user_list = UserList(users);
    let format = get_format_with_provider(format_flag, config);

    let mut output = Vec::new();
    format_output_to_writer(&user_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
