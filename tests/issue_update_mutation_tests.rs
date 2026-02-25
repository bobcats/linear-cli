use cynic::MutationBuilder;
use linear_cli::client::queries::{
    IssueUpdateInput, IssueUpdateMutation, IssueUpdateMutationVariables,
};

#[test]
fn test_issue_update_mutation_serializes_patch_fields() {
    let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: Some("New issue title".to_string()),
            description: Some("Updated markdown description".to_string()),
            assignee_id: Some("user-456".to_string()),
            project_id: Some("project-789".to_string()),
            state_id: Some("state-111".to_string()),
            priority: Some(2),
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert_eq!(json["variables"]["id"], "issue-123");
    assert_eq!(input["title"], "New issue title");
    assert_eq!(input["description"], "Updated markdown description");
    assert_eq!(input["assigneeId"], "user-456");
    assert_eq!(input["projectId"], "project-789");
    assert_eq!(input["stateId"], "state-111");
    assert_eq!(input["priority"], 2);
}

#[test]
fn test_issue_update_mutation_omits_unset_optional_patch_fields() {
    let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: None,
            description: None,
            assignee_id: None,
            project_id: Some("project-789".to_string()),
            state_id: None,
            priority: None,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert!(input.get("title").is_none(), "title should be omitted");
    assert!(
        input.get("description").is_none(),
        "description should be omitted"
    );
    assert!(
        input.get("assigneeId").is_none(),
        "assigneeId should be omitted"
    );
    assert_eq!(input["projectId"], "project-789");
    assert!(input.get("stateId").is_none(), "stateId should be omitted");
    assert!(
        input.get("priority").is_none(),
        "priority should be omitted"
    );
}
