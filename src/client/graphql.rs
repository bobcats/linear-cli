use serde::{Deserialize, Serialize};

/// GraphQL request structure
#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
}

/// GraphQL error structure
#[derive(Deserialize)]
pub struct GraphQLError {
    pub message: String,
}
