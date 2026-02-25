use linear_cli::comments::types::Comment;
use linear_cli::output::Formattable;

fn create_test_comment() -> Comment {
    Comment {
        id: "comment-123".to_string(),
        body: "This is a test comment with some **markdown** formatting.".to_string(),
        user_name: "Alice Johnson".to_string(),
        user_email: "alice@example.com".to_string(),
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
        edited_at: None,
        issue_identifier: Some("ENG-123".to_string()),
    }
}

#[test]
fn test_comment_to_json_with_full_data() {
    let comment = create_test_comment();
    let json = comment.to_json().unwrap();

    assert!(json.contains("\"id\":\"comment-123\""));
    assert!(json.contains("\"body\":\"This is a test comment"));
    assert!(json.contains("\"user_name\":\"Alice Johnson\""));
    assert!(json.contains("\"user_email\":\"alice@example.com\""));
    assert!(json.contains("\"issue_identifier\":\"ENG-123\""));
    assert!(json.contains("\"edited_at\":null"));
}

#[test]
fn test_comment_to_json_with_edited_timestamp() {
    let mut comment = create_test_comment();
    comment.edited_at = Some("2024-01-15T14:00:00Z".to_string());

    let json = comment.to_json().unwrap();

    assert!(json.contains("\"edited_at\":\"2024-01-15T14:00:00Z\""));
}

#[test]
fn test_comment_to_json_without_issue() {
    let mut comment = create_test_comment();
    comment.issue_identifier = None;

    let json = comment.to_json().unwrap();

    assert!(json.contains("\"issue_identifier\":null"));
}

#[test]
fn test_comment_to_csv_has_header_and_single_row() {
    let comment = create_test_comment();
    let csv = comment.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2, "CSV should have header + 1 data row");

    let header = lines[0];
    assert!(header.contains("id"));
    assert!(header.contains("user_name"));
    assert!(header.contains("body_preview"));
    assert!(header.contains("created_at"));
    assert!(header.contains("edited_at"));
    assert!(header.contains("issue"));
}

#[test]
fn test_comment_to_csv_with_full_data() {
    let comment = create_test_comment();
    let csv = comment.to_csv().unwrap();

    assert!(csv.contains("comment-123"));
    assert!(csv.contains("Alice Johnson"));
    assert!(csv.contains("alice@example.com"));
    assert!(csv.contains("This is a test comment"));
    assert!(csv.contains("ENG-123"));
}

#[test]
fn test_comment_to_csv_truncates_long_body() {
    let mut comment = create_test_comment();
    comment.body = "a".repeat(150); // 150 chars

    let csv = comment.to_csv().unwrap();

    // Should be truncated to 100 chars (97 + "...")
    assert!(csv.contains("..."));
    let body_line = csv.lines().nth(1).unwrap();
    // The body field should not contain all 150 chars
    assert!(!body_line.contains(&"a".repeat(150)));
}

#[test]
fn test_comment_to_markdown_with_full_data() {
    let comment = create_test_comment();
    let markdown = comment.to_markdown().unwrap();

    assert!(markdown.contains("# Comment on ENG-123"));
    assert!(markdown.contains("**Author:** Alice Johnson (alice@example.com)"));
    assert!(markdown.contains("**Created:** 2024-01-15T10:30:00Z"));
    assert!(markdown.contains("This is a test comment with some **markdown** formatting."));
    assert!(markdown.contains("**ID:** comment-123"));
    assert!(markdown.contains("**Updated:** 2024-01-15T10:30:00Z"));
}

#[test]
fn test_comment_to_markdown_with_edited_timestamp() {
    let mut comment = create_test_comment();
    comment.edited_at = Some("2024-01-15T14:00:00Z".to_string());

    let markdown = comment.to_markdown().unwrap();

    assert!(markdown.contains("**Edited:** 2024-01-15T14:00:00Z"));
}

#[test]
fn test_comment_to_markdown_without_issue() {
    let mut comment = create_test_comment();
    comment.issue_identifier = None;

    let markdown = comment.to_markdown().unwrap();

    assert!(markdown.contains("# Comment\n"));
    assert!(!markdown.contains("# Comment on"));
}

#[test]
fn test_comment_to_table_with_full_data() {
    let comment = create_test_comment();
    let table = comment.to_table().unwrap();

    assert!(table.contains("Issue"));
    assert!(table.contains("ENG-123"));
    assert!(table.contains("Author"));
    assert!(table.contains("Alice Johnson (alice@example.com)"));
    assert!(table.contains("Created"));
    assert!(table.contains("Body"));
    assert!(table.contains("This is a test comment"));
}

#[test]
fn test_comment_to_table_with_edited_timestamp() {
    let mut comment = create_test_comment();
    comment.edited_at = Some("2024-01-15T14:00:00Z".to_string());

    let table = comment.to_table().unwrap();

    assert!(table.contains("Edited"));
    assert!(table.contains("2024-01-15T14:00:00Z"));
}

#[test]
fn test_comment_to_table_without_issue() {
    let mut comment = create_test_comment();
    comment.issue_identifier = None;

    let table = comment.to_table().unwrap();

    // Issue row should not appear
    let lines: Vec<&str> = table.lines().collect();
    assert!(
        !lines
            .iter()
            .any(|line| line.contains("Issue") && line.contains("ENG-"))
    );
}
