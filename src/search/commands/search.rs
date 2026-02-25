use crate::auth::config::ConfigProvider;
use crate::auth::storage::TokenStorage;
use crate::auth::token::get_token_with_provider;
use crate::client::queries::SemanticSearchResultType;
use crate::client::semantic_search::SemanticSearchClient;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output_to_writer, get_format_with_provider};
use crate::search::types::SemanticSearchResultList;
use secrecy::ExposeSecret;

/// Parse a type string into the GraphQL enum
fn parse_result_type(s: &str) -> Result<SemanticSearchResultType, CliError> {
    match s.to_lowercase().as_str() {
        "issue" => Ok(SemanticSearchResultType::Issue),
        "project" => Ok(SemanticSearchResultType::Project),
        "document" => Ok(SemanticSearchResultType::Document),
        "initiative" => Ok(SemanticSearchResultType::Initiative),
        _ => Err(CliError::InvalidArgs(format!(
            "Invalid search type: {s}. Valid types: issue, project, document, initiative"
        ))),
    }
}

/// Handle the semantic search command
#[allow(clippy::too_many_arguments)]
pub fn handle_semantic_search(
    query: &str,
    type_filter: Option<&str>,
    max_results: Option<i32>,
    client: &dyn SemanticSearchClient,
    config: &dyn ConfigProvider,
    storage: &dyn TokenStorage,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    let token = get_token_with_provider(config, storage)?;

    let types = type_filter
        .map(|t| {
            t.split(',')
                .map(|s| parse_result_type(s.trim()))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;

    let results = client.semantic_search(token.expose_secret(), query, types, max_results)?;

    let result_list = SemanticSearchResultList(results);
    let format = get_format_with_provider(format_flag, config);

    let mut output = Vec::new();
    format_output_to_writer(&result_list, format, &mut output)?;
    io.print_bytes(&output);

    Ok(())
}
