use cynic::QueryBuilder;
use linear_cli::client::queries::{
    IDComparatorInput, IssueFilterInput, IssuesQuery, IssuesQueryVariables,
    NullableProjectFilterInput, NullableUserFilterInput,
};

#[test]
fn test_issues_query_omits_unset_assignee_filter_when_project_filter_is_set() {
    let operation = IssuesQuery::build(IssuesQueryVariables {
        first: Some(50),
        filter: Some(IssueFilterInput {
            assignee: None,
            project: Some(NullableProjectFilterInput {
                id: Some(IDComparatorInput {
                    eq: Some(cynic::Id::new("project-123")),
                }),
                name: None,
                slug_id: None,
            }),
        }),
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let filter = &json["variables"]["filter"];

    assert!(filter.get("project").is_some());
    assert!(
        filter.get("assignee").is_none(),
        "assignee filter should be omitted when not set"
    );
}

#[test]
fn test_issues_query_omits_unset_project_filter_when_assignee_filter_is_set() {
    let operation = IssuesQuery::build(IssuesQueryVariables {
        first: Some(50),
        filter: Some(IssueFilterInput {
            assignee: Some(NullableUserFilterInput {
                id: Some(IDComparatorInput {
                    eq: Some(cynic::Id::new("user-123")),
                }),
                is_me: None,
                email: None,
            }),
            project: None,
        }),
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let filter = &json["variables"]["filter"];

    assert!(filter.get("assignee").is_some());
    assert!(
        filter.get("project").is_none(),
        "project filter should be omitted when not set"
    );
}
