use crate::client::LinearClient;
use crate::client::queries::ViewerQuery;
use crate::error::CliError;
use cynic::QueryBuilder;
use serde::{Deserialize, Serialize};

/// User information returned from Linear API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// Trait for authentication operations with Linear API
pub trait AuthClient: Send + Sync {
    /// Validate a token and return user information
    fn validate_token(&self, token: &str) -> Result<UserInfo, CliError>;
}

/// Production implementation using Linear GraphQL API
impl AuthClient for LinearClient {
    fn validate_token(&self, token: &str) -> Result<UserInfo, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the viewer query using Cynic
        let operation = ViewerQuery::build(());

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::Auth)?;

        // Extract user info
        let viewer = response
            .data
            .ok_or_else(|| CliError::AuthError("No user data returned".to_string()))?
            .viewer;

        // Convert from Cynic types to our public UserInfo type
        Ok(UserInfo {
            id: viewer.id.inner().to_string(),
            name: viewer.name,
            email: viewer.email,
        })
    }
}

/// Mock implementation for testing
pub struct MockAuthClient {
    pub result: Result<UserInfo, CliError>,
}

impl AuthClient for MockAuthClient {
    fn validate_token(&self, _token: &str) -> Result<UserInfo, CliError> {
        self.result.clone()
    }
}
