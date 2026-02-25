use linear_cli::issues::types::{Issue, IssueList, IssueState, Priority, User};
use linear_cli::output::Formattable;

/// Helper to create a test issue list with multiple issues
fn create_test_issue_list() -> IssueList {
    let issue1 = Issue {
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
        created_at: "2025-11-01T10:00:00Z".to_string(),
        updated_at: "2025-11-13T09:30:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-123".to_string(),
        project: None,
        comments: None,
    };

    let issue2 = Issue {
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
        created_at: "2025-11-02T14:00:00Z".to_string(),
        updated_at: "2025-11-02T14:00:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-124".to_string(),
        project: None,
        comments: None,
    };

    let issue3 = Issue {
        id: "issue-125".to_string(),
        identifier: "ENG-125".to_string(),
        title: "Optimize queries".to_string(),
        description: Some("Improve database query performance".to_string()),
        state: IssueState {
            id: "state-3".to_string(),
            name: "In Review".to_string(),
        },
        priority: Priority::Urgent,
        assignee: Some(User {
            id: "user-3".to_string(),
            name: "Charlie Davis".to_string(),
            email: "charlie@example.com".to_string(),
        }),
        creator: User {
            id: "user-1".to_string(),
            name: "Alice Smith".to_string(),
            email: "alice@example.com".to_string(),
        },
        created_at: "2025-11-03T09:00:00Z".to_string(),
        updated_at: "2025-11-10T15:30:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-125".to_string(),
        project: None,
        comments: None,
    };

    IssueList(vec![issue1, issue2, issue3])
}

// ============================================================================
// JSON Formatter Tests
// ============================================================================

#[test]
fn test_issue_list_to_json_with_multiple_issues() {
    let list = create_test_issue_list();
    let json = list.to_json().unwrap();

    // Should be valid JSON array
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());

    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 3);

    // Check first issue
    assert_eq!(arr[0]["identifier"], "ENG-123");
    // Check second issue
    assert_eq!(arr[1]["identifier"], "ENG-124");
    // Check third issue
    assert_eq!(arr[2]["identifier"], "ENG-125");
}

#[test]
fn test_issue_list_to_json_with_empty_list() {
    let list = IssueList(vec![]);
    let json = list.to_json().unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 0);
}

#[test]
fn test_issue_list_to_json_with_single_issue() {
    let issue = Issue {
        id: "issue-1".to_string(),
        identifier: "TEST-1".to_string(),
        title: "Test".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Done".to_string(),
        },
        priority: Priority::Medium,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        },
        created_at: "2025-01-01T00:00:00Z".to_string(),
        updated_at: "2025-01-01T00:00:00Z".to_string(),
        url: "https://linear.app/test".to_string(),
        project: None,
        comments: None,
    };

    let list = IssueList(vec![issue]);
    let json = list.to_json().unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 1);
}

// ============================================================================
// CSV Formatter Tests
// ============================================================================

#[test]
fn test_issue_list_to_csv_with_multiple_issues() {
    let list = create_test_issue_list();
    let csv = list.to_csv().unwrap();

    // Should have header row
    assert!(csv.contains("identifier"));
    assert!(csv.contains("title"));
    assert!(csv.contains("state"));

    // Should have all three issues
    assert!(csv.contains("ENG-123"));
    assert!(csv.contains("ENG-124"));
    assert!(csv.contains("ENG-125"));

    // Check CSV structure: 1 header + 3 data rows = 4 lines
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 4);
}

#[test]
fn test_issue_list_to_csv_with_empty_list() {
    let list = IssueList(vec![]);
    let csv = list.to_csv().unwrap();

    // Should still have header row
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1); // Just the header
    assert!(csv.contains("identifier"));
}

// ============================================================================
// Markdown Formatter Tests
// ============================================================================

#[test]
fn test_issue_list_to_markdown_with_multiple_issues() {
    let list = create_test_issue_list();
    let md = list.to_markdown().unwrap();

    // Should have count header
    assert!(md.contains("## Issues (3)"));

    // Should have all three issue cards
    assert!(md.contains("### ENG-123: Fix authentication bug"));
    assert!(md.contains("### ENG-124: Add dark mode"));
    assert!(md.contains("### ENG-125: Optimize queries"));

    // Should have metadata for each
    assert!(md.contains("**State:** In Progress"));
    assert!(md.contains("**State:** Todo"));
    assert!(md.contains("**State:** In Review"));

    // Should have descriptions
    assert!(md.contains("Users cannot login with SSO"));
    assert!(md.contains("[No description]"));
    assert!(md.contains("Improve database query performance"));

    // Should have separators
    assert!(md.contains("---"));
}

#[test]
fn test_issue_list_to_markdown_with_empty_list() {
    let list = IssueList(vec![]);
    let md = list.to_markdown().unwrap();

    assert!(md.contains("## Issues (0)"));
}

// ============================================================================
// Table Formatter Tests
// ============================================================================

#[test]
fn test_issue_list_to_table_with_multiple_issues() {
    let list = create_test_issue_list();
    let table = list.to_table().unwrap();

    // Should have column headers
    assert!(table.contains("ID") || table.contains("Identifier"));
    assert!(table.contains("Title"));
    assert!(table.contains("State"));

    // Should have all three issues
    assert!(table.contains("ENG-123"));
    assert!(table.contains("ENG-124"));
    assert!(table.contains("ENG-125"));

    assert!(table.contains("Fix authentication bug"));
    assert!(table.contains("Add dark mode"));
    assert!(table.contains("Optimize queries"));
}

#[test]
fn test_issue_list_to_table_with_empty_list() {
    let list = IssueList(vec![]);
    let table = list.to_table().unwrap();

    // Should have headers but no data rows
    assert!(table.contains("ID") || table.contains("Identifier"));
}

#[test]
fn test_issue_list_to_table_uses_horizontal_layout() {
    let list = create_test_issue_list();
    let table = list.to_table().unwrap();

    // Horizontal layout means issues are rows, not columns
    // We should have more than one data row
    let lines: Vec<&str> = table.lines().collect();
    assert!(
        lines.len() > 3,
        "Table should have multiple rows for multiple issues"
    );
}
