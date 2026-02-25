use crate::client::LinearClient;
use crate::client::queries::{TeamQuery, TeamQueryVariables, TeamsQuery, TeamsQueryVariables};
use crate::error::CliError;
use crate::teams::types::Team;
use cynic::QueryBuilder;

/// Trait for team operations with Linear API
pub trait TeamClient: Send + Sync {
    /// Get a team by ID
    fn get_team(&self, token: &str, id: &str) -> Result<Team, CliError>;

    /// List teams with optional filters
    fn list_teams(&self, token: &str, limit: usize) -> Result<Vec<Team>, CliError>;
}

/// Production implementation using Linear GraphQL API
impl TeamClient for LinearClient {
    fn get_team(&self, token: &str, id: &str) -> Result<Team, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the team query using Cynic
        let operation = TeamQuery::build(TeamQueryVariables { id: id.to_string() });

        // Execute the query using shared method
        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract team data
        let team_node = response
            .data
            .ok_or_else(|| CliError::NotFound(format!("Team {id} not found")))?
            .team;

        // Convert from Cynic types to our public Team type using From
        Ok(team_node.into())
    }

    fn list_teams(&self, token: &str, limit: usize) -> Result<Vec<Team>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the teams query using Cynic
        let operation = TeamsQuery::build(TeamsQueryVariables {
            first: Some(limit as i32),
        });

        // Execute the query using shared method
        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract teams from response
        let teams_connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .teams;

        // Convert from Cynic types to our public Team type using From
        Ok(teams_connection.nodes.into_iter().map(Into::into).collect())
    }
}

/// Mock implementation for testing
pub struct MockTeamClient {
    pub result: Result<Team, CliError>,
    pub list_result: Result<Vec<Team>, CliError>,
}

impl TeamClient for MockTeamClient {
    fn get_team(&self, _token: &str, _id: &str) -> Result<Team, CliError> {
        self.result.clone()
    }

    fn list_teams(&self, _token: &str, _limit: usize) -> Result<Vec<Team>, CliError> {
        self.list_result.clone()
    }
}
