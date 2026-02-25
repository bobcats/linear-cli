pub mod auth;
pub mod comments;
pub mod cycles;
pub mod graphql;
pub mod issues;
pub mod labels;
pub mod projects;
pub mod queries;
pub mod search;
pub mod semantic_search;
pub mod states;
pub mod teams;
pub mod users;

use std::time::Duration;

use crate::error::CliError;
use cynic::GraphQlError;
use reqwest::blocking::Client;

/// Linear API GraphQL endpoint
const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

/// Connection timeout for the HTTP client
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Overall request timeout for the HTTP client
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Check for GraphQL errors and convert to CliError
///
/// This helper function extracts error messages from GraphQL error responses
/// and converts them into the appropriate CliError variant.
pub(crate) fn check_graphql_errors(
    errors: &[GraphQlError],
    error_type: GraphQlErrorType,
) -> Result<(), CliError> {
    if !errors.is_empty() {
        let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();

        let combined_message = error_messages.join(", ");

        return Err(match error_type {
            GraphQlErrorType::Auth => {
                CliError::AuthError(format!("Invalid token: {}", combined_message))
            }
            GraphQlErrorType::General => {
                CliError::General(format!("GraphQL error: {}", combined_message))
            }
        });
    }
    Ok(())
}

/// Type of GraphQL error to return
pub(crate) enum GraphQlErrorType {
    Auth,
    General,
}

/// Production Linear API client that implements all domain traits
pub struct LinearClient {
    http_client: Client,
}

impl Default for LinearClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearClient {
    #[must_use]
    pub fn new() -> Self {
        let http_client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build HTTP client");
        Self { http_client }
    }

    /// Get the HTTP client for use in trait implementations
    pub(crate) fn client(&self) -> &Client {
        &self.http_client
    }

    /// Get the base URL for use in trait implementations
    pub(crate) fn base_url(&self) -> &str {
        LINEAR_API_URL
    }

    /// Execute a GraphQL query with shared error handling
    ///
    /// This method encapsulates the common pattern of:
    /// 1. Building the HTTP request with auth header
    /// 2. Executing the GraphQL operation
    /// 3. Checking for network errors
    /// 4. Checking for GraphQL errors
    /// 5. Returning the response data
    ///
    /// # Arguments
    /// * `token` - The Linear API authentication token
    /// * `operation` - The Cynic operation to execute
    /// * `error_type` - The type of error to return for GraphQL errors
    ///
    /// # Returns
    /// The GraphQL response if successful
    ///
    /// # Errors
    /// Returns `CliError` for network errors, auth errors, or GraphQL errors
    pub(crate) fn execute_query<ResponseData, Vars>(
        &self,
        token: &str,
        operation: cynic::Operation<ResponseData, Vars>,
        error_type: GraphQlErrorType,
    ) -> Result<cynic::GraphQlResponse<ResponseData>, CliError>
    where
        ResponseData: serde::de::DeserializeOwned + 'static,
        Vars: serde::Serialize,
    {
        use cynic::http::ReqwestBlockingExt;

        // Execute the query with auth header
        let response = self
            .client()
            .post(self.base_url())
            .header("Authorization", token)
            .run_graphql(operation)
            .map_err(|e| CliError::NetworkError(format!("Failed to connect to Linear API: {e}")))?;

        // Check for GraphQL errors
        if let Some(errors) = &response.errors {
            check_graphql_errors(errors, error_type)?;
        }

        Ok(response)
    }
}
