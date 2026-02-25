use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::output::Formattable;

/// Helper function to create a test issue with all fields populated
fn create_test_issue_full() -> Issue {
    Issue {
        id: "issue-123".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Fix authentication bug".to_string(),
        description: Some("Users cannot login with SSO".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::Urgent,
        assignee: Some(User {
            id: "user-1".to_string(),
            name: "Alice Smith".to_string(),
            email: "alice@example.com".to_string(),
        }),
        creator: User {
            id: "user-2".to_string(),
            name: "Bob Jones".to_string(),
            email: "bob@example.com".to_string(),
        },
        project: None,
        created_at: "2025-11-01T10:00:00Z".to_string(),
        updated_at: "2025-11-13T09:30:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-123".to_string(),
        comments: None,
    }
}

fn create_test_issue_minimal() -> Issue {
    Issue {
        id: "issue-124".to_string(),
        identifier: "ENG-124".to_string(),
        title: "Add dark mode".to_string(),
        description: None,
        state: IssueState {
            id: "state-2".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::High,
        assignee: None,
        creator: User {
            id: "user-2".to_string(),
            name: "Bob Jones".to_string(),
            email: "bob@example.com".to_string(),
        },
        project: None,
        created_at: "2025-11-02T14:00:00Z".to_string(),
        updated_at: "2025-11-02T14:00:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-124".to_string(),
        comments: None,
    }
}

// ============================================================================
// JSON Formatter Tests
// ============================================================================

#[test]
fn test_issue_to_json_with_full_data() {
    let issue = create_test_issue_full();
    let json = issue.to_json().unwrap();

    // Should be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    // Check key fields
    assert_eq!(parsed["identifier"], "ENG-123");
    assert_eq!(parsed["title"], "Fix authentication bug");
    assert_eq!(parsed["description"], "Users cannot login with SSO");
    assert_eq!(parsed["state"]["name"], "In Progress");
    assert_eq!(parsed["priority"], "urgent");
    assert_eq!(parsed["assignee"]["name"], "Alice Smith");
}

#[test]
fn test_issue_to_json_with_minimal_data() {
    let issue = create_test_issue_minimal();
    let json = issue.to_json().unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["identifier"], "ENG-124");
    assert_eq!(parsed["description"], serde_json::Value::Null);
    assert_eq!(parsed["assignee"], serde_json::Value::Null);
}

// ============================================================================
// CSV Formatter Tests
// ============================================================================

#[test]
fn test_issue_to_csv_with_full_data() {
    let issue = create_test_issue_full();
    let csv = issue.to_csv().unwrap();

    // Should have headers
    assert!(csv.contains("identifier"));
    assert!(csv.contains("title"));
    assert!(csv.contains("state"));

    // Should have data row
    assert!(csv.contains("ENG-123"));
    assert!(csv.contains("Fix authentication bug"));
    assert!(csv.contains("In Progress"));
    assert!(csv.contains("Alice Smith"));
}

#[test]
fn test_issue_to_csv_with_minimal_data() {
    let issue = create_test_issue_minimal();
    let csv = issue.to_csv().unwrap();

    assert!(csv.contains("ENG-124"));
    assert!(csv.contains("Add dark mode"));
    // Empty assignee should be represented (empty cell or placeholder)
    assert!(csv.contains("Todo"));
}

#[test]
fn test_issue_to_csv_has_header_and_single_row() {
    let issue = create_test_issue_full();
    let csv = issue.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    // Should have exactly 2 lines: header + data
    assert_eq!(lines.len(), 2);
}

// ============================================================================
// Markdown Formatter Tests
// ============================================================================

#[test]
fn test_issue_to_markdown_with_full_data() {
    let issue = create_test_issue_full();
    let md = issue.to_markdown().unwrap();

    // Should have H1 title with identifier
    assert!(md.contains("# ENG-123: Fix authentication bug"));

    // Should have metadata section
    assert!(md.contains("**State:**"));
    assert!(md.contains("In Progress"));
    assert!(md.contains("**Priority:**"));
    assert!(md.contains("**Assignee:**"));
    assert!(md.contains("Alice Smith"));

    // Should have Description section
    assert!(md.contains("## Description"));
    assert!(md.contains("Users cannot login with SSO"));

    // Should have Details section
    assert!(md.contains("## Details"));
    assert!(md.contains("**Created:**"));
    assert!(md.contains("**Updated:**"));
    assert!(md.contains("**URL:**"));
}

#[test]
fn test_issue_to_markdown_with_minimal_data() {
    let issue = create_test_issue_minimal();
    let md = issue.to_markdown().unwrap();

    assert!(md.contains("# ENG-124: Add dark mode"));
    assert!(md.contains("**State:** Todo"));

    // No assignee should show placeholder
    assert!(md.contains("**Assignee:**"));

    // Description should show placeholder for None
    assert!(md.contains("## Description"));
}

// ============================================================================
// Table Formatter Tests
// ============================================================================

#[test]
fn test_issue_to_table_with_full_data() {
    let issue = create_test_issue_full();
    let table = issue.to_table().unwrap();

    // Should contain field names
    assert!(table.contains("Identifier") || table.contains("identifier"));
    assert!(table.contains("Title") || table.contains("title"));

    // Should contain values
    assert!(table.contains("ENG-123"));
    assert!(table.contains("Fix authentication bug"));
    assert!(table.contains("In Progress"));
    assert!(table.contains("Alice Smith"));
}

#[test]
fn test_issue_to_table_with_minimal_data() {
    let issue = create_test_issue_minimal();
    let table = issue.to_table().unwrap();

    assert!(table.contains("ENG-124"));
    assert!(table.contains("Add dark mode"));
    assert!(table.contains("Todo"));
}

#[test]
fn test_issue_to_table_uses_vertical_layout() {
    let issue = create_test_issue_full();
    let table = issue.to_table().unwrap();

    // Vertical layout means each field is a separate row
    // We should see multiple lines
    let lines: Vec<&str> = table.lines().collect();
    assert!(
        lines.len() > 5,
        "Table should have multiple rows for vertical layout"
    );
}

#[test]
fn test_issue_with_project_shows_project_name() {
    use linear_cli::issues::types::IssueProject;

    let mut issue = create_test_issue_full();
    issue.project = Some(IssueProject {
        id: "proj-1".to_string(),
        name: "Backend Services".to_string(),
        slug_id: "backend-services".to_string(),
    });

    let table = issue.to_table().unwrap();
    assert!(table.contains("Project"));
    assert!(table.contains("Backend Services"));

    let md = issue.to_markdown().unwrap();
    assert!(md.contains("**Project:** Backend Services"));

    let json = issue.to_json().unwrap();
    assert!(json.contains("Backend Services"));
}
