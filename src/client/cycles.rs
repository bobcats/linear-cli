use crate::client::LinearClient;
use crate::client::queries::{CycleQuery, CycleQueryVariables, CyclesQuery, CyclesQueryVariables};
use crate::cycles::types::Cycle;
use crate::error::CliError;
use cynic::QueryBuilder;
use cynic::http::ReqwestBlockingExt;

/// Trait for cycle operations with Linear API
pub trait CycleClient: Send + Sync {
    /// Get a cycle by ID
    fn get_cycle(&self, token: &str, id: &str) -> Result<Cycle, CliError>;

    /// List cycles with optional filters
    fn list_cycles(&self, token: &str, limit: usize) -> Result<Vec<Cycle>, CliError>;
}

/// Production implementation using Linear GraphQL API
impl CycleClient for LinearClient {
    fn get_cycle(&self, token: &str, id: &str) -> Result<Cycle, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the cycle query using Cynic
        let operation = CycleQuery::build(CycleQueryVariables { id: id.to_string() });

        // Execute the query
        let response = self
            .client()
            .post(self.base_url())
            .header("Authorization", token)
            .run_graphql(operation)
            .map_err(|e| CliError::NetworkError(format!("Failed to connect to Linear API: {e}")))?;

        // Check for GraphQL errors
        if let Some(errors) = response.errors {
            crate::client::check_graphql_errors(&errors, crate::client::GraphQlErrorType::General)?;
        }

        // Extract cycle data
        let cycle_node = response
            .data
            .ok_or_else(|| CliError::NotFound(format!("Cycle {id} not found")))?
            .cycle;

        // Convert from Cynic types to our public Cycle type using From
        Ok(cycle_node.into())
    }

    fn list_cycles(&self, token: &str, limit: usize) -> Result<Vec<Cycle>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the cycles query using Cynic
        let operation = CyclesQuery::build(CyclesQueryVariables {
            first: Some(limit as i32),
        });

        // Execute the query
        let response = self
            .client()
            .post(self.base_url())
            .header("Authorization", token)
            .run_graphql(operation)
            .map_err(|e| CliError::NetworkError(format!("Failed to connect to Linear API: {e}")))?;

        // Check for GraphQL errors
        if let Some(errors) = response.errors {
            crate::client::check_graphql_errors(&errors, crate::client::GraphQlErrorType::General)?;
        }

        // Extract cycles from response
        let cycles_connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .cycles;

        // Convert from Cynic types to our public Cycle type using From
        Ok(cycles_connection
            .nodes
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

/// Mock implementation for testing
pub struct MockCycleClient {
    pub result: Result<Cycle, CliError>,
    pub list_result: Result<Vec<Cycle>, CliError>,
}

impl CycleClient for MockCycleClient {
    fn get_cycle(&self, _token: &str, _id: &str) -> Result<Cycle, CliError> {
        self.result.clone()
    }

    fn list_cycles(&self, _token: &str, _limit: usize) -> Result<Vec<Cycle>, CliError> {
        self.list_result.clone()
    }
}
