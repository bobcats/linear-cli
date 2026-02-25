use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::labels::LabelClient;
use crate::error::CliError;
use crate::io::Io;
use crate::labels::types::IssueLabelList;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use secrecy::ExposeSecret;

/// Handle the label list command
pub fn handle_list(
    limit: usize,
    team_key: Option<&str>,
    client: &dyn LabelClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;
    let labels = client.list_labels(token.expose_secret(), team_key, limit)?;
    let label_list = IssueLabelList(labels);
    let format = get_format_with_provider(format_flag, config);

    let mut output = Vec::new();
    format_output_to_writer(&label_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
