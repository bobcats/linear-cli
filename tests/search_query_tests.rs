use cynic::QueryBuilder;
use linear_cli::client::queries::{SearchIssuesQuery, SearchIssuesQueryVariables};

#[test]
fn test_search_issues_query_serializes_with_term() {
    let operation = SearchIssuesQuery::build(SearchIssuesQueryVariables {
        term: "token refresh".to_string(),
        first: Some(10),
        team_id: None,
        include_comments: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["term"], "token refresh");
    assert_eq!(json["variables"]["first"], 10);
}

#[test]
fn test_search_issues_query_with_team_boost() {
    let operation = SearchIssuesQuery::build(SearchIssuesQueryVariables {
        term: "authentication".to_string(),
        first: Some(25),
        team_id: Some("team-eng-123".to_string()),
        include_comments: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["term"], "authentication");
    assert_eq!(json["variables"]["teamId"], "team-eng-123");
}

#[test]
fn test_search_issues_query_with_include_comments() {
    let operation = SearchIssuesQuery::build(SearchIssuesQueryVariables {
        term: "bug".to_string(),
        first: Some(50),
        team_id: None,
        include_comments: Some(true),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["includeComments"], true);
}

#[test]
fn test_search_issues_query_omits_optional_fields_when_none() {
    let operation = SearchIssuesQuery::build(SearchIssuesQueryVariables {
        term: "test".to_string(),
        first: None,
        team_id: None,
        include_comments: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert!(
        json["variables"].get("teamId").is_none() || json["variables"]["teamId"].is_null(),
        "teamId should be omitted or null when not set"
    );
    assert!(
        json["variables"].get("includeComments").is_none()
            || json["variables"]["includeComments"].is_null(),
        "includeComments should be omitted or null when not set"
    );
}
