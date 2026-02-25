use linear_cli::comments::types::{Comment, CommentList};
use linear_cli::output::Formattable;

fn create_test_comments(count: usize) -> Vec<Comment> {
    (0..count)
        .map(|i| Comment {
            id: format!("comment-{}", i),
            body: format!("This is comment {} with some content.", i),
            user_name: format!("User {}", i),
            user_email: format!("user{}@example.com", i),
            created_at: "2024-01-15T10:00:00Z".to_string(),
            updated_at: "2024-01-15T10:00:00Z".to_string(),
            edited_at: if i.is_multiple_of(2) {
                Some("2024-01-15T14:00:00Z".to_string())
            } else {
                None
            },
            issue_identifier: Some(format!("ENG-{}", i + 100)),
        })
        .collect()
}

#[test]
fn test_comment_list_to_json_empty() {
    let list = CommentList(vec![]);
    let json = list.to_json().unwrap();

    assert_eq!(json.trim(), "[]");
}

#[test]
fn test_comment_list_to_json_single_comment() {
    let comments = create_test_comments(1);
    let list = CommentList(comments);
    let json = list.to_json().unwrap();

    assert!(json.contains("\"id\":\"comment-0\""));
    assert!(json.contains("\"user_name\":\"User 0\""));
}

#[test]
fn test_comment_list_to_json_multiple_comments() {
    let comments = create_test_comments(3);
    let list = CommentList(comments);
    let json = list.to_json().unwrap();

    assert!(json.contains("\"id\":\"comment-0\""));
    assert!(json.contains("\"id\":\"comment-1\""));
    assert!(json.contains("\"id\":\"comment-2\""));
}

#[test]
fn test_comment_list_to_csv_empty() {
    let list = CommentList(vec![]);
    let csv = list.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1, "CSV should have only header");
    assert!(lines[0].contains("id"));
    assert!(lines[0].contains("user_name"));
}

#[test]
fn test_comment_list_to_csv_has_header() {
    let comments = create_test_comments(2);
    let list = CommentList(comments);
    let csv = list.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 3, "CSV should have header + 2 data rows");

    let header = lines[0];
    assert!(header.contains("id"));
    assert!(header.contains("user_name"));
    assert!(header.contains("body_preview"));
    assert!(header.contains("created_at"));
    assert!(header.contains("edited_at"));
}

#[test]
fn test_comment_list_to_csv_multiple_rows() {
    let comments = create_test_comments(3);
    let list = CommentList(comments);
    let csv = list.to_csv().unwrap();

    assert!(csv.contains("comment-0"));
    assert!(csv.contains("comment-1"));
    assert!(csv.contains("comment-2"));
    assert!(csv.contains("User 0"));
    assert!(csv.contains("User 1"));
    assert!(csv.contains("User 2"));
}

#[test]
fn test_comment_list_to_csv_truncates_long_bodies() {
    let mut comments = create_test_comments(1);
    comments[0].body = "a".repeat(150);

    let list = CommentList(comments);
    let csv = list.to_csv().unwrap();

    // Should contain ellipsis for truncation
    assert!(csv.contains("..."));
}

#[test]
fn test_comment_list_to_markdown_empty() {
    let list = CommentList(vec![]);
    let markdown = list.to_markdown().unwrap();

    assert!(markdown.contains("## Comments (0)"));
}

#[test]
fn test_comment_list_to_markdown_with_count() {
    let comments = create_test_comments(5);
    let list = CommentList(comments);
    let markdown = list.to_markdown().unwrap();

    assert!(markdown.contains("## Comments (5)"));
}

#[test]
fn test_comment_list_to_markdown_multiple_comments() {
    let comments = create_test_comments(3);
    let list = CommentList(comments);
    let markdown = list.to_markdown().unwrap();

    assert!(markdown.contains("### User 0"));
    assert!(markdown.contains("### User 1"));
    assert!(markdown.contains("### User 2"));
    assert!(markdown.contains("This is comment 0"));
    assert!(markdown.contains("This is comment 1"));
    assert!(markdown.contains("This is comment 2"));
}

#[test]
fn test_comment_list_to_markdown_shows_edited() {
    let comments = create_test_comments(2);
    let list = CommentList(comments);
    let markdown = list.to_markdown().unwrap();

    // First comment (index 0) should show edited timestamp
    assert!(markdown.contains("**Edited:** 2024-01-15T14:00:00Z"));
}

#[test]
fn test_comment_list_to_markdown_has_separators() {
    let comments = create_test_comments(2);
    let list = CommentList(comments);
    let markdown = list.to_markdown().unwrap();

    // Should have separators between comments
    let separator_count = markdown.matches("---").count();
    assert!(separator_count >= 2, "Should have at least 2 separators");
}

#[test]
fn test_comment_list_to_table_empty() {
    let list = CommentList(vec![]);
    let table = list.to_table().unwrap();

    // Should have header but no rows
    assert!(table.contains("Author"));
    assert!(table.contains("Body Preview"));
}

#[test]
fn test_comment_list_to_table_has_headers() {
    let comments = create_test_comments(1);
    let list = CommentList(comments);
    let table = list.to_table().unwrap();

    assert!(table.contains("Author"));
    assert!(table.contains("Body Preview"));
    assert!(table.contains("Created"));
    assert!(table.contains("Edited"));
}

#[test]
fn test_comment_list_to_table_multiple_rows() {
    let comments = create_test_comments(3);
    let list = CommentList(comments);
    let table = list.to_table().unwrap();

    assert!(table.contains("User 0"));
    assert!(table.contains("User 1"));
    assert!(table.contains("User 2"));
}

#[test]
fn test_comment_list_to_table_truncates_long_bodies() {
    let mut comments = create_test_comments(1);
    comments[0].body = "a".repeat(150);

    let list = CommentList(comments);
    let table = list.to_table().unwrap();

    // Should contain ellipsis for truncation
    assert!(table.contains("..."));
}

#[test]
fn test_comment_list_to_table_shows_edited_or_empty() {
    let comments = create_test_comments(2);
    let list = CommentList(comments);
    let table = list.to_table().unwrap();

    // One should have edited timestamp, one should be empty
    assert!(table.contains("2024-01-15T14:00:00Z"));
}
