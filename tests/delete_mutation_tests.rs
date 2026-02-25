use cynic::MutationBuilder;
use linear_cli::client::queries::{
    CommentDeleteMutation, CommentDeleteMutationVariables, IssueDeleteMutation,
    IssueDeleteMutationVariables,
};

// ── issueDelete mutation tests ──

#[test]
fn test_issue_delete_mutation_serializes_with_id() {
    let operation = IssueDeleteMutation::build(IssueDeleteMutationVariables {
        id: "issue-123".to_string(),
        permanently_delete: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["id"], "issue-123");
}

#[test]
fn test_issue_delete_mutation_with_permanent_flag() {
    let operation = IssueDeleteMutation::build(IssueDeleteMutationVariables {
        id: "issue-456".to_string(),
        permanently_delete: Some(true),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["id"], "issue-456");
    assert_eq!(json["variables"]["permanentlyDelete"], true);
}

#[test]
fn test_issue_delete_mutation_omits_permanent_when_none() {
    let operation = IssueDeleteMutation::build(IssueDeleteMutationVariables {
        id: "issue-789".to_string(),
        permanently_delete: None,
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert!(
        json["variables"].get("permanentlyDelete").is_none()
            || json["variables"]["permanentlyDelete"].is_null()
    );
}

// ── commentDelete mutation tests ──

#[test]
fn test_comment_delete_mutation_serializes_with_id() {
    let operation = CommentDeleteMutation::build(CommentDeleteMutationVariables {
        id: "comment-123".to_string(),
    });

    let json = serde_json::to_value(&operation).expect("should serialize");
    assert_eq!(json["variables"]["id"], "comment-123");
}
