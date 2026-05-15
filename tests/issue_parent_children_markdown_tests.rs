use linear_cli::issues::types::{Issue, IssueState, ParentIssue, Priority, SubIssue, User};
use linear_cli::output::Formattable;

fn base_issue() -> Issue {
    Issue {
        id: "issue-1".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Child issue".to_string(),
        description: Some("A child".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
            state_type: "started".to_string(),
        },
        priority: Priority::Medium,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        project: None,
        milestone: None,
        created_at: "2026-03-11T00:00:00Z".to_string(),
        updated_at: "2026-03-11T00:00:00Z".to_string(),
        url: "https://linear.app/test/issue/ENG-123".to_string(),
        parent: None,
        children: None,
        comments: None,
    }
}

#[test]
fn markdown_includes_parent_when_present() {
    let mut issue = base_issue();
    issue.parent = Some(ParentIssue {
        id: "parent-1".to_string(),
        identifier: "ENG-100".to_string(),
        title: "Parent task".to_string(),
    });

    let md = issue.to_markdown().unwrap();
    assert!(md.contains("**Parent:** ENG-100 — Parent task"));
}

#[test]
fn markdown_omits_parent_when_absent() {
    let issue = base_issue();
    let md = issue.to_markdown().unwrap();
    assert!(!md.contains("Parent:"));
}

#[test]
fn markdown_renders_sub_issues_with_checkboxes() {
    let mut issue = base_issue();
    issue.children = Some(vec![
        SubIssue {
            identifier: "ENG-201".to_string(),
            title: "Setup".to_string(),
            state_name: "Done".to_string(),
            state_type: "completed".to_string(),
            assignee_name: Some("Alice".to_string()),
        },
        SubIssue {
            identifier: "ENG-202".to_string(),
            title: "Implement".to_string(),
            state_name: "In Progress".to_string(),
            state_type: "started".to_string(),
            assignee_name: None,
        },
        SubIssue {
            identifier: "ENG-203".to_string(),
            title: "Review".to_string(),
            state_name: "Todo".to_string(),
            state_type: "unstarted".to_string(),
            assignee_name: Some("Bob".to_string()),
        },
    ]);

    let md = issue.to_markdown().unwrap();
    assert!(md.contains("## Sub-issues (1/3)"));
    assert!(md.contains("- [x] ENG-201 Setup — @Alice"));
    assert!(md.contains("- [~] ENG-202 Implement"));
    assert!(md.contains("- [ ] ENG-203 Review — @Bob"));
}

#[test]
fn markdown_omits_sub_issues_when_absent() {
    let issue = base_issue();
    let md = issue.to_markdown().unwrap();
    assert!(!md.contains("Sub-issues"));
}

#[test]
fn json_includes_parent_and_children_when_present() {
    let mut issue = base_issue();
    issue.parent = Some(ParentIssue {
        id: "parent-1".to_string(),
        identifier: "ENG-100".to_string(),
        title: "Parent task".to_string(),
    });
    issue.children = Some(vec![SubIssue {
        identifier: "ENG-201".to_string(),
        title: "Sub-task".to_string(),
        state_name: "Done".to_string(),
        state_type: "completed".to_string(),
        assignee_name: Some("Alice".to_string()),
    }]);

    let json_str = issue.to_json().unwrap();
    let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert_eq!(v["parent"]["identifier"], "ENG-100");
    assert_eq!(v["children"][0]["identifier"], "ENG-201");
}

#[test]
fn json_omits_parent_and_children_when_absent() {
    let issue = base_issue();
    let json_str = issue.to_json().unwrap();
    let v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    assert!(v.get("parent").is_none());
    assert!(v.get("children").is_none());
}
