use cynic::{MutationBuilder, QueryBuilder};
use linear_cli::client::queries::{
    ProjectMilestoneCreateInput, ProjectMilestoneCreateMutation,
    ProjectMilestoneCreateMutationVariables, ProjectMilestoneDeleteMutation,
    ProjectMilestoneDeleteMutationVariables, ProjectMilestoneUpdateInput,
    ProjectMilestoneUpdateMutation, ProjectMilestoneUpdateMutationVariables,
    ProjectMilestonesQuery, ProjectMilestonesQueryVariables, TimelessDate,
};

#[test]
fn project_milestone_create_serializes_required_fields() {
    let operation =
        ProjectMilestoneCreateMutation::build(ProjectMilestoneCreateMutationVariables {
            input: ProjectMilestoneCreateInput {
                project_id: "project-1".to_string(),
                name: "Beta".to_string(),
                description: None,
                target_date: Some(TimelessDate("2026-06-30".to_string())),
            },
        });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(input["projectId"], "project-1");
    assert_eq!(input["name"], "Beta");
    assert_eq!(input["targetDate"], "2026-06-30");
    assert!(input.get("description").is_none());
}

#[test]
fn project_milestone_update_omits_unset_patch_fields() {
    let operation =
        ProjectMilestoneUpdateMutation::build(ProjectMilestoneUpdateMutationVariables {
            id: "milestone-1".to_string(),
            input: ProjectMilestoneUpdateInput {
                name: None,
                description: Some("Updated".to_string()),
                project_id: None,
                target_date: None,
            },
        });
    let json = serde_json::to_value(&operation).unwrap();
    let input = &json["variables"]["input"];

    assert_eq!(json["variables"]["id"], "milestone-1");
    assert_eq!(input["description"], "Updated");
    assert!(input.get("name").is_none());
    assert!(input.get("projectId").is_none());
    assert!(input.get("targetDate").is_none());
}

#[test]
fn project_milestone_delete_serializes_id() {
    let operation =
        ProjectMilestoneDeleteMutation::build(ProjectMilestoneDeleteMutationVariables {
            id: "milestone-1".to_string(),
        });
    let json = serde_json::to_value(&operation).unwrap();

    assert_eq!(json["variables"]["id"], "milestone-1");
}

#[test]
fn project_milestones_query_serializes_limit() {
    let operation = ProjectMilestonesQuery::build(ProjectMilestonesQueryVariables {
        first: Some(25),
        name: None,
    });
    let json = serde_json::to_value(&operation).unwrap();

    assert_eq!(json["variables"]["first"], 25);
}
