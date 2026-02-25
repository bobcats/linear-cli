/// Issue search results reuse the existing IssueList formatter.
/// These tests verify that search results format correctly through the same path.
use linear_cli::issues::types::{Issue, IssueList, IssueState, Priority, User};
use linear_cli::output::Formattable;

fn make_search_result(identifier: &str, title: &str) -> Issue {
    Issue {
        id: format!("id-{identifier}"),
        identifier: identifier.to_string(),
        title: title.to_string(),
        description: None,
        priority: Priority::from_i32(0),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        },
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        url: format!("https://linear.app/issue/{identifier}"),
        project: None,
        comments: None,
    }
}

#[test]
fn test_search_results_format_as_json_list() {
    let results = IssueList(vec![
        make_search_result("ENG-123", "Fix token refresh"),
        make_search_result("ENG-456", "Token expiry handling"),
    ]);

    let json = results.to_json().expect("should format as JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    let arr = parsed.as_array().expect("should be array");
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["identifier"], "ENG-123");
    assert_eq!(arr[1]["title"], "Token expiry handling");
}

#[test]
fn test_search_results_format_as_csv() {
    let results = IssueList(vec![make_search_result("ENG-123", "Fix token refresh")]);

    let csv = results.to_csv().expect("should format as CSV");
    assert!(csv.contains("identifier"));
    assert!(csv.contains("ENG-123"));
    assert!(csv.contains("Fix token refresh"));
}

#[test]
fn test_search_results_format_as_markdown() {
    let results = IssueList(vec![make_search_result("ENG-123", "Fix token refresh")]);

    let md = results.to_markdown().expect("should format as markdown");
    assert!(md.contains("ENG-123"));
    assert!(md.contains("Fix token refresh"));
}

#[test]
fn test_search_results_empty_list() {
    let results = IssueList(vec![]);

    let json = results.to_json().expect("should format empty list");
    assert_eq!(json, "[]");
}
