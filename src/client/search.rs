use crate::client::LinearClient;
use crate::client::queries::{SearchIssuesQuery, SearchIssuesQueryVariables};
use crate::error::CliError;
use crate::issues::types::Issue;
use cynic::QueryBuilder;

/// Trait for issue search operations
pub trait SearchClient: Send + Sync {
    fn search_issues(
        &self,
        token: &str,
        term: &str,
        team_id: Option<&str>,
        include_comments: bool,
        limit: usize,
    ) -> Result<Vec<Issue>, CliError>;
}

impl SearchClient for LinearClient {
    fn search_issues(
        &self,
        token: &str,
        term: &str,
        team_id: Option<&str>,
        include_comments: bool,
        limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = SearchIssuesQuery::build(SearchIssuesQueryVariables {
            term: term.to_string(),
            first: Some(limit as i32),
            team_id: team_id.map(|s| s.to_string()),
            include_comments: if include_comments { Some(true) } else { None },
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .search_issues;

        payload
            .nodes
            .into_iter()
            .map(|node| node.try_into())
            .collect()
    }
}

/// Mock implementation for testing
pub struct MockSearchClient {
    pub search_result: Result<Vec<Issue>, CliError>,
}

impl SearchClient for MockSearchClient {
    fn search_issues(
        &self,
        _token: &str,
        _term: &str,
        _team_id: Option<&str>,
        _include_comments: bool,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        self.search_result.clone()
    }
}
