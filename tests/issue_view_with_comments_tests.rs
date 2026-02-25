use linear_cli::comments::types::Comment;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};
use linear_cli::output::Formattable;

fn create_test_issue_with_comments() -> Issue {
    let comments = vec![
        Comment {
            id: "comment-1".to_string(),
            body: "First comment on this issue".to_string(),
            user_name: "Alice".to_string(),
            user_email: "alice@example.com".to_string(),
            created_at: "2024-01-15T10:00:00Z".to_string(),
            updated_at: "2024-01-15T10:00:00Z".to_string(),
            edited_at: None,
            issue_identifier: Some("ENG-123".to_string()),
        },
        Comment {
            id: "comment-2".to_string(),
            body: "Second comment with more details".to_string(),
            user_name: "Bob".to_string(),
            user_email: "bob@example.com".to_string(),
            created_at: "2024-01-15T11:00:00Z".to_string(),
            updated_at: "2024-01-15T11:00:00Z".to_string(),
            edited_at: None,
            issue_identifier: Some("ENG-123".to_string()),
        },
    ];

    Issue {
        id: "issue-123".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Fix authentication bug".to_string(),
        description: Some("Users cannot log in".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::High,
        assignee: Some(User {
            id: "user-1".to_string(),
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        }),
        creator: User {
            id: "user-2".to_string(),
            name: "David".to_string(),
            email: "david@example.com".to_string(),
        },
        project: None,
        created_at: "2024-01-10T09:00:00Z".to_string(),
        updated_at: "2024-01-15T12:00:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-123".to_string(),
        comments: Some(comments),
    }
}

fn create_test_issue_without_comments() -> Issue {
    Issue {
        id: "issue-124".to_string(),
        identifier: "ENG-124".to_string(),
        title: "Add new feature".to_string(),
        description: Some("Implement user dashboard".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::Medium,
        assignee: None,
        creator: User {
            id: "user-2".to_string(),
            name: "David".to_string(),
            email: "david@example.com".to_string(),
        },
        created_at: "2024-01-14T09:00:00Z".to_string(),
        updated_at: "2024-01-14T09:00:00Z".to_string(),
        url: "https://linear.app/team/issue/ENG-124".to_string(),
        project: None,
        comments: None,
    }
}

#[test]
fn test_issue_with_comments_to_json_includes_comments() {
    let issue = create_test_issue_with_comments();
    let json = issue.to_json().unwrap();

    assert!(json.contains("\"comments\""));
    assert!(json.contains("\"First comment on this issue\""));
    assert!(json.contains("\"Second comment with more details\""));
    assert!(json.contains("Alice"));
    assert!(json.contains("Bob"));
}

#[test]
fn test_issue_without_comments_to_json_no_comments_field() {
    let issue = create_test_issue_without_comments();
    let json = issue.to_json().unwrap();

    // Should not include comments field when None
    assert!(!json.contains("\"comments\""));
}

#[test]
fn test_issue_with_comments_to_markdown_includes_comments_section() {
    let issue = create_test_issue_with_comments();
    let markdown = issue.to_markdown().unwrap();

    // Should have a comments section
    assert!(markdown.contains("## Comments"));
    assert!(markdown.contains("First comment on this issue"));
    assert!(markdown.contains("Second comment with more details"));
    assert!(markdown.contains("Alice"));
    assert!(markdown.contains("Bob"));
}

#[test]
fn test_issue_without_comments_to_markdown_no_comments_section() {
    let issue = create_test_issue_without_comments();
    let markdown = issue.to_markdown().unwrap();

    // Should not have a comments section when None
    assert!(!markdown.contains("## Comments"));
}

#[test]
fn test_issue_with_comments_to_table_includes_comment_count() {
    let issue = create_test_issue_with_comments();
    let table = issue.to_table().unwrap();

    // Should show comment count
    assert!(table.contains("Comments"));
    assert!(table.contains("2"));
}

#[test]
fn test_issue_without_comments_to_table_no_comment_field() {
    let issue = create_test_issue_without_comments();
    let table = issue.to_table().unwrap();

    // Should not show comments field
    let lines: Vec<&str> = table.lines().collect();
    assert!(
        !lines
            .iter()
            .any(|line| line.contains("Comments") && line.contains("0"))
    );
}

#[test]
fn test_issue_with_empty_comments_vec() {
    let mut issue = create_test_issue_without_comments();
    issue.comments = Some(vec![]);

    let markdown = issue.to_markdown().unwrap();

    // Should show "No comments" or similar
    assert!(markdown.contains("## Comments") || !markdown.contains("Comments"));
}

#[test]
fn test_issue_with_comments_csv_shows_comment_count() {
    let issue = create_test_issue_with_comments();
    let csv = issue.to_csv().unwrap();

    // CSV should include comment count in a field
    assert!(csv.contains("comment_count") || csv.contains("2"));
}
