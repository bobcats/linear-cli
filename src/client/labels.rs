use crate::client::LinearClient;
use crate::client::queries::{IssueLabelsQuery, IssueLabelsQueryVariables};
use crate::error::CliError;
use crate::labels::types::IssueLabel;
use cynic::QueryBuilder;

/// Trait for label operations
pub trait LabelClient: Send + Sync {
    fn list_labels(
        &self,
        token: &str,
        team_key: Option<&str>,
        limit: usize,
    ) -> Result<Vec<IssueLabel>, CliError>;
}

impl LabelClient for LinearClient {
    fn list_labels(
        &self,
        token: &str,
        team_key: Option<&str>,
        limit: usize,
    ) -> Result<Vec<IssueLabel>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueLabelsQuery::build(IssueLabelsQueryVariables {
            first: Some(limit as i32),
            team_id: team_key.map(|k| k.to_string()),
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_labels;

        Ok(connection.nodes.into_iter().map(Into::into).collect())
    }
}

/// Mock implementation for testing
pub struct MockLabelClient {
    pub list_result: Result<Vec<IssueLabel>, CliError>,
}

impl LabelClient for MockLabelClient {
    fn list_labels(
        &self,
        _token: &str,
        _team_key: Option<&str>,
        _limit: usize,
    ) -> Result<Vec<IssueLabel>, CliError> {
        self.list_result.clone()
    }
}
