use crate::client::LinearClient;
use crate::client::queries::{UsersQuery, UsersQueryVariables};
use crate::error::CliError;
use crate::users::types::User;
use cynic::QueryBuilder;

/// Trait for user operations
pub trait UserClient: Send + Sync {
    fn list_users(&self, token: &str, limit: usize) -> Result<Vec<User>, CliError>;
}

impl UserClient for LinearClient {
    fn list_users(&self, token: &str, limit: usize) -> Result<Vec<User>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = UsersQuery::build(UsersQueryVariables {
            first: Some(limit as i32),
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .users;

        Ok(connection.nodes.into_iter().map(Into::into).collect())
    }
}

/// Mock implementation for testing
pub struct MockUserClient {
    pub list_result: Result<Vec<User>, CliError>,
}

impl UserClient for MockUserClient {
    fn list_users(&self, _token: &str, _limit: usize) -> Result<Vec<User>, CliError> {
        self.list_result.clone()
    }
}
