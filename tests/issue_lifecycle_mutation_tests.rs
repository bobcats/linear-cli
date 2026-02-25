use cynic::MutationBuilder;
use linear_cli::client::queries::{
    IssueArchiveMutation, IssueArchiveMutationVariables, IssueUnarchiveMutation,
    IssueUnarchiveMutationVariables,
};

#[test]
fn test_issue_archive_mutation_serializes_required_and_optional_fields() {
    let operation = IssueArchiveMutation::build(IssueArchiveMutationVariables {
        id: "issue-123".to_string(),
        trash: Some(true),
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let vars = &json["variables"];

    assert_eq!(vars["id"], "issue-123");
    assert_eq!(vars["trash"], true);
}

#[test]
fn test_issue_unarchive_mutation_serializes_required_fields() {
    let operation = IssueUnarchiveMutation::build(IssueUnarchiveMutationVariables {
        id: "issue-123".to_string(),
    });

    let json = serde_json::to_value(&operation).expect("operation should serialize to JSON");
    let vars = &json["variables"];

    assert_eq!(vars["id"], "issue-123");
    assert!(vars.get("trash").is_none());
}
