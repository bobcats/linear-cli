use crate::client::LinearClient;
use crate::client::queries::{
    CommentCreateInput, CommentCreateMutation, CommentCreateMutationVariables,
    CommentDeleteMutation, CommentDeleteMutationVariables, IssueCommentsQuery,
    IssueCommentsQueryVariables,
};
use crate::comments::types::Comment;
use crate::error::CliError;
use cynic::{MutationBuilder, QueryBuilder};

/// Create comment request payload.
#[derive(Debug, Clone, Default)]
pub struct CreateCommentInput {
    pub issue_id: String,
    pub body: String,
}

/// Trait for comment operations with Linear API
pub trait CommentClient: Send + Sync {
    /// List comments for an issue
    fn list_comments(
        &self,
        token: &str,
        issue_id: &str,
        limit: usize,
    ) -> Result<Vec<Comment>, CliError>;

    /// Delete a comment
    fn delete_comment(&self, token: &str, id: &str) -> Result<(), CliError> {
        let _ = (token, id);
        Err(CliError::InvalidArgs(
            "comment delete is not implemented for this client".to_string(),
        ))
    }

    /// Create a comment on an issue
    fn create_comment(&self, token: &str, input: CreateCommentInput) -> Result<Comment, CliError> {
        let _ = (token, input);
        Err(CliError::InvalidArgs(
            "comment create is not implemented for this client".to_string(),
        ))
    }
}

/// Production implementation using Linear GraphQL API
impl CommentClient for LinearClient {
    fn list_comments(
        &self,
        token: &str,
        issue_id: &str,
        limit: usize,
    ) -> Result<Vec<Comment>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the issue comments query using Cynic
        let operation = IssueCommentsQuery::build(IssueCommentsQueryVariables {
            id: issue_id.to_string(),
            first: Some(limit as i32),
        });

        // Execute the query using shared method
        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract issue and comments from response
        let issue_with_comments = response
            .data
            .and_then(|d| d.issue)
            .ok_or_else(|| CliError::NotFound(format!("Issue {issue_id} not found")))?;

        // Convert from Cynic types to our public Comment type using From
        Ok(issue_with_comments
            .comments
            .nodes
            .into_iter()
            .map(|comment_node| comment_node.into())
            .collect())
    }

    fn create_comment(&self, token: &str, input: CreateCommentInput) -> Result<Comment, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = CommentCreateMutation::build(CommentCreateMutationVariables {
            input: CommentCreateInput {
                issue_id: Some(input.issue_id),
                body: Some(input.body),
            },
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .comment_create;

        Ok(payload.comment.into())
    }

    fn delete_comment(&self, token: &str, id: &str) -> Result<(), CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation =
            CommentDeleteMutation::build(CommentDeleteMutationVariables { id: id.to_string() });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .comment_delete;

        if !payload.success {
            return Err(CliError::General("Comment delete failed".to_string()));
        }

        Ok(())
    }
}

/// Mock implementation for testing
pub struct MockCommentClient {
    pub list_result: Result<Vec<Comment>, CliError>,
    pub create_result: Result<Comment, CliError>,
    pub delete_result: Result<(), CliError>,
}

impl CommentClient for MockCommentClient {
    fn list_comments(
        &self,
        _token: &str,
        _issue_id: &str,
        _limit: usize,
    ) -> Result<Vec<Comment>, CliError> {
        self.list_result.clone()
    }

    fn delete_comment(&self, _token: &str, _id: &str) -> Result<(), CliError> {
        self.delete_result.clone()
    }

    fn create_comment(
        &self,
        _token: &str,
        _input: CreateCommentInput,
    ) -> Result<Comment, CliError> {
        self.create_result.clone()
    }
}
