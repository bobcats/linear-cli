use linear_cli::client::issues::{IssueClient, MockIssueClient};
use linear_cli::error::CliError;
use linear_cli::issues::types::{Issue, IssueState, Priority, User};

#[test]
fn test_issue_serializes_to_json() {
    let issue = Issue {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        identifier: "ENG-123".to_string(),
        title: "Fix login bug".to_string(),
        description: Some("User cannot login with email".to_string()),
        state: IssueState {
            id: "state-1".to_string(),
            name: "In Progress".to_string(),
        },
        priority: Priority::Urgent,
        assignee: Some(User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }),
        creator: User {
            id: "user-2".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-16T14:20:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-123".to_string(),
        project: None,
        comments: None,
    };

    let json = serde_json::to_string(&issue).expect("Failed to serialize");

    // Verify it contains expected fields
    assert!(json.contains("ENG-123"));
    assert!(json.contains("Fix login bug"));
    assert!(json.contains("In Progress"));
    assert!(json.contains("alice@example.com"));
}

#[test]
fn test_issue_with_no_assignee_serializes() {
    let issue = Issue {
        id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        identifier: "ENG-124".to_string(),
        title: "Add feature".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::None,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-124".to_string(),
        project: None,
        comments: None,
    };

    let json = serde_json::to_string(&issue).expect("Failed to serialize");

    // Verify it handles optional fields
    assert!(json.contains("ENG-124"));
    assert!(json.contains("null") || !json.contains("assignee"));
}

#[test]
fn test_mock_issue_client_returns_configured_issue() {
    let expected_issue = Issue {
        id: "123".to_string(),
        identifier: "ENG-999".to_string(),
        title: "Test issue".to_string(),
        description: None,
        state: IssueState {
            id: "state-1".to_string(),
            name: "Todo".to_string(),
        },
        priority: Priority::None,
        assignee: None,
        creator: User {
            id: "user-1".to_string(),
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        created_at: "2024-01-15T10:30:00Z".to_string(),
        updated_at: "2024-01-15T10:30:00Z".to_string(),
        url: "https://linear.app/company/issue/ENG-999".to_string(),
        project: None,
        comments: None,
    };

    let mock_client = MockIssueClient {
        result: Ok(expected_issue.clone()),
        list_result: Ok(vec![]),
    };

    let result = mock_client.get_issue("fake_token", "ENG-999");

    assert!(result.is_ok());
    let issue = result.unwrap();
    assert_eq!(issue.identifier, "ENG-999");
    assert_eq!(issue.title, "Test issue");
}

#[test]
fn test_mock_issue_client_returns_configured_error() {
    let mock_client = MockIssueClient {
        result: Err(CliError::NotFound("Issue not found".to_string())),
        list_result: Ok(vec![]),
    };

    let result = mock_client.get_issue("fake_token", "ENG-999");

    assert!(result.is_err());
    match result.unwrap_err() {
        CliError::NotFound(msg) => assert!(msg.contains("not found")),
        _ => panic!("Expected NotFound error"),
    }
}
