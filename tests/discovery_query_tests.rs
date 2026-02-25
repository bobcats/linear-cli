use cynic::QueryBuilder;
use linear_cli::client::queries::{
    IssueLabelsQuery, IssueLabelsQueryVariables, UsersQuery, UsersQueryVariables,
    WorkflowStatesQuery, WorkflowStatesQueryVariables,
};

// ── WorkflowStates query tests ──

#[test]
fn test_workflow_states_query_serializes_with_limit() {
    let operation = WorkflowStatesQuery::build(WorkflowStatesQueryVariables {
        first: Some(50),
        team_id: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["first"], 50);
}

#[test]
fn test_workflow_states_query_serializes_team_filter() {
    let operation = WorkflowStatesQuery::build(WorkflowStatesQueryVariables {
        first: Some(50),
        team_id: Some("team-eng".to_string()),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["teamId"], "team-eng");
}

#[test]
fn test_workflow_states_query_omits_unset_team_filter() {
    let operation = WorkflowStatesQuery::build(WorkflowStatesQueryVariables {
        first: Some(50),
        team_id: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert!(
        json["variables"].get("teamId").is_none() || json["variables"]["teamId"].is_null(),
        "teamId should be omitted or null when not set"
    );
}

// ── IssueLabels query tests ──

#[test]
fn test_issue_labels_query_serializes_with_limit() {
    let operation = IssueLabelsQuery::build(IssueLabelsQueryVariables {
        first: Some(100),
        team_id: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["first"], 100);
}

#[test]
fn test_issue_labels_query_serializes_team_filter() {
    let operation = IssueLabelsQuery::build(IssueLabelsQueryVariables {
        first: Some(50),
        team_id: Some("team-design".to_string()),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["teamId"], "team-design");
}

// ── Users query tests ──

#[test]
fn test_users_query_serializes_with_limit() {
    let operation = UsersQuery::build(UsersQueryVariables { first: Some(25) });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["first"], 25);
}
