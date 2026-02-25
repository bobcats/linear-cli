use cynic::MutationBuilder;
use linear_cli::client::queries::{
    CommentCreateInput, CommentCreateMutation, CommentCreateMutationVariables,
};

#[test]
fn test_comment_create_mutation_serializes_required_fields() {
    let operation = CommentCreateMutation::build(CommentCreateMutationVariables {
        input: CommentCreateInput {
            issue_id: Some("issue-123".to_string()),
            body: Some("Looks good to me".to_string()),
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert_eq!(input["issueId"], "issue-123");
    assert_eq!(input["body"], "Looks good to me");
}

#[test]
fn test_comment_create_mutation_omits_unset_optional_fields() {
    let operation = CommentCreateMutation::build(CommentCreateMutationVariables {
        input: CommentCreateInput {
            issue_id: Some("issue-123".to_string()),
            body: Some("Looks good to me".to_string()),
        },
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let input = &json["variables"]["input"];

    assert!(input.get("parentId").is_none());
    assert!(input.get("postId").is_none());
    assert!(input.get("initiativeUpdateId").is_none());
}
