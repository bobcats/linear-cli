use cynic::MutationBuilder;
use linear_cli::client::queries::{
    IssueRelationCreateInput, IssueRelationCreateMutation, IssueRelationCreateMutationVariables,
    IssueRelationType,
};

#[test]
fn test_issue_relation_create_mutation_serializes_related_relation() {
    let operation = IssueRelationCreateMutation::build(IssueRelationCreateMutationVariables {
        input: IssueRelationCreateInput {
            issue_id: "issue-123".to_string(),
            related_issue_id: "issue-456".to_string(),
            relation_type: IssueRelationType::Related,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert_eq!(input["issueId"], "issue-123");
    assert_eq!(input["relatedIssueId"], "issue-456");
    assert_eq!(input["type"], "related");
}

#[test]
fn test_issue_relation_create_mutation_serializes_blocks_relation() {
    let operation = IssueRelationCreateMutation::build(IssueRelationCreateMutationVariables {
        input: IssueRelationCreateInput {
            issue_id: "issue-123".to_string(),
            related_issue_id: "issue-456".to_string(),
            relation_type: IssueRelationType::Blocks,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    assert_eq!(json["variables"]["input"]["type"], "blocks");
}

#[test]
fn test_issue_relation_create_mutation_serializes_duplicate_relation() {
    let operation = IssueRelationCreateMutation::build(IssueRelationCreateMutationVariables {
        input: IssueRelationCreateInput {
            issue_id: "issue-123".to_string(),
            related_issue_id: "issue-456".to_string(),
            relation_type: IssueRelationType::Duplicate,
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    assert_eq!(json["variables"]["input"]["type"], "duplicate");
}
