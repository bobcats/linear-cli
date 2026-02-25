use crate::client::LinearClient;
use crate::client::queries::{WorkflowStatesQuery, WorkflowStatesQueryVariables};
use crate::error::CliError;
use crate::states::types::WorkflowState;
use cynic::QueryBuilder;

/// Trait for workflow state operations
pub trait StateClient: Send + Sync {
    fn list_states(
        &self,
        token: &str,
        team_key: Option<&str>,
        limit: usize,
    ) -> Result<Vec<WorkflowState>, CliError>;
}

impl StateClient for LinearClient {
    fn list_states(
        &self,
        token: &str,
        team_key: Option<&str>,
        limit: usize,
    ) -> Result<Vec<WorkflowState>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = WorkflowStatesQuery::build(WorkflowStatesQueryVariables {
            first: Some(limit as i32),
            team_id: team_key.map(|k| k.to_string()),
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .workflow_states;

        Ok(connection.nodes.into_iter().map(Into::into).collect())
    }
}

/// Mock implementation for testing
pub struct MockStateClient {
    pub list_result: Result<Vec<WorkflowState>, CliError>,
}

impl StateClient for MockStateClient {
    fn list_states(
        &self,
        _token: &str,
        _team_key: Option<&str>,
        _limit: usize,
    ) -> Result<Vec<WorkflowState>, CliError> {
        self.list_result.clone()
    }
}
