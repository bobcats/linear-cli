use cynic::MutationBuilder;
use linear_cli::client::queries::{
    IssueCreateInput, IssueCreateMutation, IssueCreateMutationVariables,
};

#[test]
fn test_issue_create_mutation_serializes_required_fields() {
    let operation = IssueCreateMutation::build(IssueCreateMutationVariables {
        input: IssueCreateInput {
            team_id: "team-123".to_string(),
            title: Some("Implement issue create command".to_string()),
            description: None,
            assignee_id: None,
            project_id: None,
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: None,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert_eq!(input["teamId"], "team-123");
    assert_eq!(input["title"], "Implement issue create command");
}

#[test]
fn test_issue_create_mutation_omits_unset_optional_fields() {
    let operation = IssueCreateMutation::build(IssueCreateMutationVariables {
        input: IssueCreateInput {
            team_id: "team-123".to_string(),
            title: Some("Implement issue create command".to_string()),
            description: None,
            assignee_id: None,
            project_id: Some("project-456".to_string()),
            state_id: None,
            priority: Some(2),
            parent_id: None,
            project_milestone_id: None,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert!(
        input.get("description").is_none(),
        "description should be omitted when not set"
    );
    assert!(
        input.get("assigneeId").is_none(),
        "assigneeId should be omitted when not set"
    );
    assert!(
        input.get("stateId").is_none(),
        "stateId should be omitted when not set"
    );
    assert_eq!(input["projectId"], "project-456");
    assert_eq!(input["priority"], 2);
}

#[test]
fn test_issue_create_mutation_serializes_project_milestone_id() {
    let operation = IssueCreateMutation::build(IssueCreateMutationVariables {
        input: IssueCreateInput {
            team_id: "team-123".to_string(),
            title: Some("Ship beta".to_string()),
            description: None,
            assignee_id: None,
            project_id: Some("project-1".to_string()),
            state_id: None,
            priority: None,
            parent_id: None,
            project_milestone_id: Some("milestone-1".to_string()),
        },
    });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(input["projectMilestoneId"], "milestone-1");
}
