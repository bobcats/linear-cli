use linear_cli::client::queries::{
    DateTime, IssueChildConnection, IssueChildNode, IssueNode, IssueUser, WorkflowState,
};
use linear_cli::issues::types::Issue;

fn workflow_state(name: &str, state_type: &str) -> WorkflowState {
    WorkflowState {
        id: cynic::Id::new(format!("state-{name}")),
        name: name.to_string(),
        state_type: state_type.to_string(),
    }
}

fn child(identifier: &str, sort_order: Option<f64>) -> IssueChildNode {
    IssueChildNode {
        id: cynic::Id::new(format!("child-{identifier}")),
        identifier: identifier.to_string(),
        title: format!("Title for {identifier}"),
        state: workflow_state("Todo", "unstarted"),
        assignee: Some(IssueUser {
            id: cynic::Id::new("user-1"),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
        sub_issue_sort_order: sort_order,
    }
}

#[test]
fn issue_try_from_sorts_children_by_sub_issue_sort_order() {
    let node = IssueNode {
        id: cynic::Id::new("issue-1"),
        identifier: "ENG-123".to_string(),
        title: "Parent issue".to_string(),
        description: Some("Parent description".to_string()),
        state: workflow_state("In Progress", "started"),
        priority: 3.0,
        assignee: None,
        creator: Some(IssueUser {
            id: cynic::Id::new("user-2"),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        }),
        project: None,
        project_milestone: None,
        created_at: DateTime("2026-03-13T00:00:00Z".to_string()),
        updated_at: DateTime("2026-03-13T00:00:00Z".to_string()),
        url: "https://linear.app/test/issue/ENG-123".to_string(),
        parent: None,
        children: IssueChildConnection {
            nodes: vec![
                child("ENG-203", Some(30.0)),
                child("ENG-201", Some(10.0)),
                child("ENG-202", Some(20.0)),
            ],
        },
    };

    let issue = Issue::try_from(node).unwrap();
    let child_identifiers: Vec<_> = issue
        .children
        .unwrap()
        .into_iter()
        .map(|child| child.identifier)
        .collect();

    assert_eq!(child_identifiers, vec!["ENG-201", "ENG-202", "ENG-203"]);
}

#[test]
fn issue_try_from_places_children_without_sort_order_after_sorted_children() {
    let node = IssueNode {
        id: cynic::Id::new("issue-2"),
        identifier: "ENG-124".to_string(),
        title: "Parent issue with gaps".to_string(),
        description: Some("Parent description".to_string()),
        state: workflow_state("In Progress", "started"),
        priority: 3.0,
        assignee: None,
        creator: Some(IssueUser {
            id: cynic::Id::new("user-2"),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        }),
        project: None,
        project_milestone: None,
        created_at: DateTime("2026-03-13T00:00:00Z".to_string()),
        updated_at: DateTime("2026-03-13T00:00:00Z".to_string()),
        url: "https://linear.app/test/issue/ENG-124".to_string(),
        parent: None,
        children: IssueChildConnection {
            nodes: vec![
                child("ENG-203", None),
                child("ENG-201", Some(10.0)),
                child("ENG-204", None),
                child("ENG-202", Some(20.0)),
            ],
        },
    };

    let issue = Issue::try_from(node).unwrap();
    let child_identifiers: Vec<_> = issue
        .children
        .unwrap()
        .into_iter()
        .map(|child| child.identifier)
        .collect();

    assert_eq!(
        child_identifiers,
        vec!["ENG-201", "ENG-202", "ENG-203", "ENG-204"]
    );
}
