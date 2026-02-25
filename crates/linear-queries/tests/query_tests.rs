/// Verify that query types compile and can be used to build operations.
use cynic::{MutationBuilder, QueryBuilder};

#[test]
fn test_viewer_query_builds() {
    let operation = linear_queries::ViewerQuery::build(());
    let query_str = operation.query;
    assert!(query_str.contains("viewer"));
}

#[test]
fn test_issue_query_builds() {
    let vars = linear_queries::IssueQueryVariables {
        id: "test-id".to_string(),
    };
    let operation = linear_queries::IssueQuery::build(vars);
    assert!(operation.query.contains("issue"));
}

#[test]
fn test_issues_query_builds() {
    let vars = linear_queries::IssuesQueryVariables {
        first: Some(10),
        filter: None,
    };
    let operation = linear_queries::IssuesQuery::build(vars);
    assert!(operation.query.contains("issues"));
}

#[test]
fn test_issue_create_mutation_builds() {
    let vars = linear_queries::IssueCreateMutationVariables {
        input: linear_queries::IssueCreateInput {
            team_id: "team-1".to_string(),
            title: Some("Test".to_string()),
            description: None,
            assignee_id: None,
            project_id: None,
            state_id: None,
            priority: None,
        },
    };
    let operation = linear_queries::IssueCreateMutation::build(vars);
    assert!(operation.query.contains("issueCreate"));
}

#[test]
fn test_search_issues_query_builds() {
    let vars = linear_queries::SearchIssuesQueryVariables {
        term: "test".to_string(),
        first: Some(5),
        team_id: None,
        include_comments: None,
    };
    let operation = linear_queries::SearchIssuesQuery::build(vars);
    assert!(operation.query.contains("searchIssues"));
}

#[test]
fn test_semantic_search_query_builds() {
    let vars = linear_queries::SemanticSearchQueryVariables {
        query: "test query".to_string(),
        max_results: Some(10),
        types: None,
    };
    let operation = linear_queries::SemanticSearchQuery::build(vars);
    assert!(operation.query.contains("semanticSearch"));
}

#[test]
fn test_issue_delete_mutation_builds() {
    let vars = linear_queries::IssueDeleteMutationVariables {
        id: "issue-id".to_string(),
        permanently_delete: None,
    };
    let operation = linear_queries::IssueDeleteMutation::build(vars);
    assert!(operation.query.contains("issueDelete"));
}

#[test]
fn test_workflow_states_query_builds() {
    let vars = linear_queries::WorkflowStatesQueryVariables {
        first: Some(50),
        team_id: None,
    };
    let operation = linear_queries::WorkflowStatesQuery::build(vars);
    assert!(operation.query.contains("workflowStates"));
}
