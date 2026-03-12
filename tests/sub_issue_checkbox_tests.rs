use linear_cli::issues::types::SubIssue;

fn sub_issue_with_state(state_type: &str) -> SubIssue {
    SubIssue {
        identifier: "ENG-1".to_string(),
        title: "Test".to_string(),
        state_name: "Some State".to_string(),
        state_type: state_type.to_string(),
        assignee_name: None,
    }
}

#[test]
fn completed_maps_to_checked() {
    assert_eq!(sub_issue_with_state("completed").checkbox(), "[x]");
}

#[test]
fn canceled_maps_to_dash() {
    assert_eq!(sub_issue_with_state("canceled").checkbox(), "[-]");
}

#[test]
fn started_maps_to_tilde() {
    assert_eq!(sub_issue_with_state("started").checkbox(), "[~]");
}

#[test]
fn unstarted_maps_to_empty() {
    assert_eq!(sub_issue_with_state("unstarted").checkbox(), "[ ]");
}

#[test]
fn backlog_maps_to_empty() {
    assert_eq!(sub_issue_with_state("backlog").checkbox(), "[ ]");
}

#[test]
fn triage_maps_to_question() {
    assert_eq!(sub_issue_with_state("triage").checkbox(), "[?]");
}

#[test]
fn unknown_state_maps_to_empty() {
    assert_eq!(sub_issue_with_state("whatever").checkbox(), "[ ]");
}

#[test]
fn is_completed_true_for_completed() {
    assert!(sub_issue_with_state("completed").is_completed());
}

#[test]
fn is_completed_false_for_started() {
    assert!(!sub_issue_with_state("started").is_completed());
}
