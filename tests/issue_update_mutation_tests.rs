use cynic::MutationBuilder;
use linear_cli::client::queries::{
    IssueUpdateInput, IssueUpdateMutation, IssueUpdateMutationVariables, NullableIssueUpdateField,
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
            parent_id: None,
            project_milestone_id: NullableIssueUpdateField::Unchanged,
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
            parent_id: None,
            project_milestone_id: NullableIssueUpdateField::Unchanged,
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
    assert!(
        input.get("projectMilestoneId").is_none(),
        "projectMilestoneId should be omitted"
    );
}

#[test]
fn test_issue_update_mutation_serializes_project_milestone_id_set() {
    let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: None,
            description: None,
            assignee_id: None,
            project_id: None,
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: NullableIssueUpdateField::Set("milestone-1".to_string()),
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert_eq!(input["projectMilestoneId"], "milestone-1");
}

#[test]
fn test_issue_update_mutation_omits_project_milestone_id_when_unchanged() {
    let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: Some("Rename".to_string()),
            description: None,
            assignee_id: None,
            project_id: None,
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: NullableIssueUpdateField::Unchanged,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert!(input.get("projectMilestoneId").is_none());
}

#[test]
fn test_issue_update_mutation_serializes_project_milestone_id_clear() {
    let operation = IssueUpdateMutation::build(IssueUpdateMutationVariables {
        id: "issue-123".to_string(),
        input: IssueUpdateInput {
            title: None,
            description: None,
            assignee_id: None,
            project_id: None,
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: NullableIssueUpdateField::Clear,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert!(input.get("projectMilestoneId").is_some());
    assert!(input["projectMilestoneId"].is_null());
}
