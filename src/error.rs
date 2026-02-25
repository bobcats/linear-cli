use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CliError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Authentication error: {0}")]
    AuthError(String),

    #[error("Invalid arguments: {0}")]
    InvalidArgs(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("{0}")]
    General(String),
}

impl CliError {
    #[must_use]
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::General(_) => 1,
            Self::NotFound(_) => 2,
            Self::AuthError(_) => 3,
            Self::InvalidArgs(_) => 4,
            Self::NetworkError(_) => 5,
            Self::RateLimitExceeded(_) => 6,
        }
    }

    /// Helper for creating authentication errors from keyring errors
    #[must_use]
    pub fn keyring_error(e: impl std::fmt::Display) -> Self {
        Self::AuthError(format!("Keyring error: {e}"))
    }

    /// Helper for creating authentication errors from static strings
    #[must_use]
    pub fn auth_error(msg: &str) -> Self {
        Self::AuthError(msg.to_string())
    }

    /// Helper for "no token found" error
    #[must_use]
    pub fn no_token() -> Self {
        Self::AuthError(
            "No authentication token found. Run 'linear auth login' to authenticate.".to_string(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

impl From<CliError> for ErrorOutput {
    fn from(error: CliError) -> Self {
        // Use static strings to avoid unnecessary allocations
        let (code, message, error_type): (&'static str, String, &'static str) = match error {
            CliError::NotFound(msg) => ("NOT_FOUND", msg, "IssueNotFoundError"),
            CliError::AuthError(msg) => ("AUTH_ERROR", msg, "AuthenticationError"),
            CliError::InvalidArgs(msg) => ("INVALID_ARGS", msg, "InvalidArgumentsError"),
            CliError::NetworkError(msg) => ("NETWORK_ERROR", msg, "NetworkError"),
            CliError::RateLimitExceeded(msg) => ("RATE_LIMIT_EXCEEDED", msg, "RateLimitError"),
            CliError::General(msg) => ("GENERAL_ERROR", msg, "GeneralError"),
        };

        ErrorOutput {
            error: ErrorDetails {
                code: code.to_string(),
                message,
                error_type: error_type.to_string(),
            },
        }
    }
}
