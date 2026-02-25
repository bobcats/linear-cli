use crate::client::LinearClient;
use crate::client::queries::{
    BooleanComparatorInput, IDComparatorInput, IssueArchiveMutation, IssueArchiveMutationVariables,
    IssueCreateInput, IssueCreateMutation, IssueCreateMutationVariables, IssueDeleteMutation,
    IssueDeleteMutationVariables, IssueFilterInput, IssueQuery, IssueQueryVariables,
    IssueRelationCreateInput, IssueRelationCreateMutation, IssueRelationCreateMutationVariables,
    IssueRelationType, IssueUnarchiveMutation, IssueUnarchiveMutationVariables, IssueUpdateInput,
    IssueUpdateMutation, IssueUpdateMutationVariables, IssuesQuery, IssuesQueryVariables,
    NullableProjectFilterInput, NullableUserFilterInput, StringComparatorInput,
};
use crate::error::CliError;
use crate::issues::types::Issue;
use cynic::{MutationBuilder, QueryBuilder};

/// Create issue request payload used by the issue client.
#[derive(Debug, Clone, Default)]
pub struct CreateIssueInput {
    pub team_id: String,
    pub title: String,
    pub description: Option<String>,
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub state_id: Option<String>,
    pub priority: Option<i32>,
}

/// Update issue request payload used by the issue client.
#[derive(Debug, Clone, Default)]
pub struct UpdateIssueInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub assignee_id: Option<String>,
    pub project_id: Option<String>,
    pub state_id: Option<String>,
    pub priority: Option<i32>,
}

/// Create issue relation request payload.
#[derive(Debug, Clone)]
pub struct CreateIssueRelationInput {
    pub issue_id: String,
    pub related_issue_id: String,
    pub relation_type: IssueRelationType,
}

fn build_list_filter(assignee: Option<&str>, project: Option<&str>) -> Option<IssueFilterInput> {
    let assignee_filter = assignee.map(|assignee_value| {
        if assignee_value == "@me" {
            NullableUserFilterInput {
                id: None,
                is_me: Some(BooleanComparatorInput { eq: Some(true) }),
                email: None,
            }
        } else if assignee_value.contains('@') {
            NullableUserFilterInput {
                id: None,
                is_me: None,
                email: Some(StringComparatorInput {
                    eq: Some(assignee_value.to_string()),
                }),
            }
        } else {
            NullableUserFilterInput {
                id: Some(IDComparatorInput {
                    eq: Some(cynic::Id::new(assignee_value)),
                }),
                is_me: None,
                email: None,
            }
        }
    });

    let project_filter = project.map(|project_value| {
        if is_uuid_like(project_value) {
            NullableProjectFilterInput {
                id: Some(IDComparatorInput {
                    eq: Some(cynic::Id::new(project_value)),
                }),
                name: None,
                slug_id: None,
            }
        } else if project_value.contains(' ') {
            NullableProjectFilterInput {
                id: None,
                name: Some(StringComparatorInput {
                    eq: Some(project_value.to_string()),
                }),
                slug_id: None,
            }
        } else {
            NullableProjectFilterInput {
                id: None,
                name: None,
                slug_id: Some(StringComparatorInput {
                    eq: Some(project_value.to_string()),
                }),
            }
        }
    });

    if assignee_filter.is_none() && project_filter.is_none() {
        None
    } else {
        Some(IssueFilterInput {
            assignee: assignee_filter,
            project: project_filter,
        })
    }
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }

    for (idx, byte) in bytes.iter().enumerate() {
        let is_dash = matches!(idx, 8 | 13 | 18 | 23);
        if is_dash {
            if *byte != b'-' {
                return false;
            }
            continue;
        }

        if !byte.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}

/// Trait for issue operations with Linear API
pub trait IssueClient: Send + Sync {
    /// Get an issue by identifier
    fn get_issue(&self, token: &str, identifier: &str) -> Result<Issue, CliError>;

    /// List issues with optional filters
    fn list_issues(
        &self,
        token: &str,
        assignee: Option<String>,
        project: Option<String>,
        limit: usize,
    ) -> Result<Vec<Issue>, CliError>;

    /// Create a new issue
    fn create_issue(&self, token: &str, input: CreateIssueInput) -> Result<Issue, CliError> {
        let _ = (token, input);
        Err(CliError::InvalidArgs(
            "issue create is not implemented for this client".to_string(),
        ))
    }

    /// Update an existing issue
    fn update_issue(
        &self,
        token: &str,
        id: &str,
        input: UpdateIssueInput,
    ) -> Result<Issue, CliError> {
        let _ = (token, id, input);
        Err(CliError::InvalidArgs(
            "issue update is not implemented for this client".to_string(),
        ))
    }

    /// Archive an issue
    fn archive_issue(&self, token: &str, id: &str, trash: bool) -> Result<Issue, CliError> {
        let _ = (token, id, trash);
        Err(CliError::InvalidArgs(
            "issue archive is not implemented for this client".to_string(),
        ))
    }

    /// Unarchive an issue
    fn unarchive_issue(&self, token: &str, id: &str) -> Result<Issue, CliError> {
        let _ = (token, id);
        Err(CliError::InvalidArgs(
            "issue unarchive is not implemented for this client".to_string(),
        ))
    }

    /// Delete an issue
    fn delete_issue(&self, token: &str, id: &str, permanently: bool) -> Result<(), CliError> {
        let _ = (token, id, permanently);
        Err(CliError::InvalidArgs(
            "issue delete is not implemented for this client".to_string(),
        ))
    }

    /// Create a relation between two issues
    fn create_issue_relation(
        &self,
        token: &str,
        input: CreateIssueRelationInput,
    ) -> Result<Issue, CliError> {
        let _ = (token, input);
        Err(CliError::InvalidArgs(
            "issue relation create is not implemented for this client".to_string(),
        ))
    }
}

/// Production implementation using Linear GraphQL API
impl IssueClient for LinearClient {
    fn get_issue(&self, token: &str, identifier: &str) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build the issue query using Cynic
        let operation = IssueQuery::build(IssueQueryVariables {
            id: identifier.to_string(),
        });

        // Execute the query using shared method
        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract issue data
        let issue_node = response
            .data
            .and_then(|d| d.issue)
            .ok_or_else(|| CliError::NotFound(format!("Issue {identifier} not found")))?;

        // Convert from Cynic types to our public Issue type using TryFrom
        issue_node.try_into()
    }

    fn create_issue(&self, token: &str, input: CreateIssueInput) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueCreateMutation::build(IssueCreateMutationVariables {
            input: IssueCreateInput {
                team_id: input.team_id,
                title: Some(input.title),
                description: input.description,
                assignee_id: input.assignee_id,
                project_id: input.project_id,
                state_id: input.state_id,
                priority: input.priority,
            },
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_create;

        let issue_node = payload.issue.ok_or_else(|| {
            CliError::General("No issue returned from create mutation".to_string())
        })?;

        issue_node.try_into()
    }

    fn update_issue(
        &self,
        token: &str,
        id: &str,
        input: UpdateIssueInput,
    ) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
            id: id.to_string(),
            input: IssueUpdateInput {
                title: input.title,
                description: input.description,
                assignee_id: input.assignee_id,
                project_id: input.project_id,
                state_id: input.state_id,
                priority: input.priority,
            },
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_update;

        let issue_node = payload.issue.ok_or_else(|| {
            CliError::General("No issue returned from update mutation".to_string())
        })?;

        issue_node.try_into()
    }

    fn archive_issue(&self, token: &str, id: &str, trash: bool) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueArchiveMutation::build(IssueArchiveMutationVariables {
            id: id.to_string(),
            trash: Some(trash),
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_archive;

        let issue_node = payload.entity.ok_or_else(|| {
            CliError::General("No issue returned from archive mutation".to_string())
        })?;

        issue_node.try_into()
    }

    fn unarchive_issue(&self, token: &str, id: &str) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation =
            IssueUnarchiveMutation::build(IssueUnarchiveMutationVariables { id: id.to_string() });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_unarchive;

        let issue_node = payload.entity.ok_or_else(|| {
            CliError::General("No issue returned from unarchive mutation".to_string())
        })?;

        issue_node.try_into()
    }

    fn delete_issue(&self, token: &str, id: &str, permanently: bool) -> Result<(), CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueDeleteMutation::build(IssueDeleteMutationVariables {
            id: id.to_string(),
            permanently_delete: if permanently { Some(true) } else { None },
        });

        self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        Ok(())
    }

    fn create_issue_relation(
        &self,
        token: &str,
        input: CreateIssueRelationInput,
    ) -> Result<Issue, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        let operation = IssueRelationCreateMutation::build(IssueRelationCreateMutationVariables {
            input: IssueRelationCreateInput {
                issue_id: input.issue_id,
                related_issue_id: input.related_issue_id,
                relation_type: input.relation_type,
            },
        });

        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        let payload = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issue_relation_create;

        payload.issue_relation.issue.try_into()
    }

    fn list_issues(
        &self,
        token: &str,
        assignee: Option<String>,
        project: Option<String>,
        limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        if token.is_empty() {
            return Err(CliError::auth_error("Token cannot be empty"));
        }

        // Build filter if assignee or project is provided
        let filter = build_list_filter(assignee.as_deref(), project.as_deref());

        // Build the issues query using Cynic
        let operation = IssuesQuery::build(IssuesQueryVariables {
            first: Some(limit as i32),
            filter,
        });

        // Execute the query using shared method
        let response =
            self.execute_query(token, operation, crate::client::GraphQlErrorType::General)?;

        // Extract issues from response
        let issues_connection = response
            .data
            .ok_or_else(|| CliError::General("No data returned".to_string()))?
            .issues;

        // Convert from Cynic types to our public Issue type using TryFrom
        issues_connection
            .nodes
            .into_iter()
            .map(|issue_node| issue_node.try_into())
            .collect()
    }
}

/// Mock implementation for testing
pub struct MockIssueClient {
    pub result: Result<Issue, CliError>,
    pub list_result: Result<Vec<Issue>, CliError>,
}

impl IssueClient for MockIssueClient {
    fn get_issue(&self, _token: &str, _identifier: &str) -> Result<Issue, CliError> {
        self.result.clone()
    }

    fn list_issues(
        &self,
        _token: &str,
        _assignee: Option<String>,
        _project: Option<String>,
        _limit: usize,
    ) -> Result<Vec<Issue>, CliError> {
        self.list_result.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::build_list_filter;

    #[test]
    fn test_build_list_filter_maps_assignee_me_to_is_me_comparator() {
        let filter = build_list_filter(Some("@me"), None).expect("filter should exist");
        let assignee = filter.assignee.expect("assignee filter should exist");

        assert!(assignee.id.is_none());
        assert!(assignee.email.is_none());
        assert_eq!(assignee.is_me.and_then(|c| c.eq), Some(true));
    }

    #[test]
    fn test_build_list_filter_maps_assignee_email_to_email_comparator() {
        let filter =
            build_list_filter(Some("user@example.com"), None).expect("filter should exist");
        let assignee = filter.assignee.expect("assignee filter should exist");

        assert!(assignee.id.is_none());
        assert!(assignee.is_me.is_none());
        assert_eq!(
            assignee.email.and_then(|c| c.eq),
            Some("user@example.com".to_string())
        );
    }

    #[test]
    fn test_build_list_filter_maps_project_name_to_name_comparator() {
        let filter =
            build_list_filter(None, Some("Buildr Agent Phase 2")).expect("filter should exist");
        let project = filter.project.expect("project filter should exist");

        assert!(project.id.is_none());
        assert!(project.slug_id.is_none());
        assert_eq!(
            project.name.and_then(|c| c.eq),
            Some("Buildr Agent Phase 2".to_string())
        );
    }

    #[test]
    fn test_build_list_filter_maps_project_slug_to_slug_comparator() {
        let filter =
            build_list_filter(None, Some("buildr-agent-phase-2")).expect("filter should exist");
        let project = filter.project.expect("project filter should exist");

        assert!(project.id.is_none());
        assert!(project.name.is_none());
        assert_eq!(
            project.slug_id.and_then(|c| c.eq),
            Some("buildr-agent-phase-2".to_string())
        );
    }

    #[test]
    fn test_build_list_filter_keeps_assignee_id_as_id_comparator() {
        let filter = build_list_filter(Some("0591a2d2-09ea-4858-a4a2-5127d2fc4f20"), None)
            .expect("filter should exist");
        let assignee = filter.assignee.expect("assignee filter should exist");

        assert_eq!(
            assignee
                .id
                .and_then(|c| c.eq)
                .map(|id| id.inner().to_string()),
            Some("0591a2d2-09ea-4858-a4a2-5127d2fc4f20".to_string())
        );
        assert!(assignee.is_me.is_none());
        assert!(assignee.email.is_none());
    }

    #[test]
    fn test_build_list_filter_keeps_project_uuid_as_id_comparator() {
        let filter = build_list_filter(None, Some("6e6ffe5e-8b52-433e-8c31-1f6ef2591cd9"))
            .expect("filter should exist");
        let project = filter.project.expect("project filter should exist");

        assert_eq!(
            project
                .id
                .and_then(|c| c.eq)
                .map(|id| id.inner().to_string()),
            Some("6e6ffe5e-8b52-433e-8c31-1f6ef2591cd9".to_string())
        );
        assert!(project.name.is_none());
        assert!(project.slug_id.is_none());
    }
}
