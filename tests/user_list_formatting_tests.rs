use linear_cli::output::Formattable;
use linear_cli::users::types::{User, UserList};

fn create_test_users() -> Vec<User> {
    vec![
        User {
            id: "user-1".to_string(),
            name: "Alice Admin".to_string(),
            display_name: "alice".to_string(),
            email: "alice@example.com".to_string(),
            active: true,
            admin: true,
            guest: false,
        },
        User {
            id: "user-2".to_string(),
            name: "Bob Builder".to_string(),
            display_name: "bob".to_string(),
            email: "bob@example.com".to_string(),
            active: true,
            admin: false,
            guest: false,
        },
        User {
            id: "user-3".to_string(),
            name: "Charlie Contractor".to_string(),
            display_name: "charlie".to_string(),
            email: "charlie@example.com".to_string(),
            active: false,
            admin: false,
            guest: true,
        },
    ]
}

#[test]
fn test_user_list_to_json_with_empty_list() {
    let list = UserList(vec![]);

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.is_array());
    assert_eq!(v.as_array().unwrap().len(), 0);
}

#[test]
fn test_user_list_to_json_with_multiple_users() {
    let list = UserList(create_test_users());

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["name"], "Alice Admin");
    assert_eq!(arr[1]["name"], "Bob Builder");
    assert_eq!(arr[2]["name"], "Charlie Contractor");
}

#[test]
fn test_user_list_to_csv_with_empty_list() {
    let list = UserList(vec![]);

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 1, "Expected header only");
}

#[test]
fn test_user_list_to_csv_with_multiple_users() {
    let list = UserList(create_test_users());

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 4, "Expected header + 3 data rows");
    assert!(csv.contains("alice@example.com"));
    assert!(csv.contains("bob@example.com"));
    assert!(csv.contains("charlie@example.com"));
}

#[test]
fn test_user_list_to_markdown_with_empty_list() {
    let list = UserList(vec![]);

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(0)"));
}

#[test]
fn test_user_list_to_markdown_with_multiple_users() {
    let list = UserList(create_test_users());

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(3)"));
    assert!(md.contains("Alice Admin"));
    assert!(md.contains("Bob Builder"));
    assert!(md.contains("Charlie Contractor"));
}

#[test]
fn test_user_list_to_table_with_empty_list() {
    let list = UserList(vec![]);

    let result = list.to_table();

    assert!(result.is_ok());
}

#[test]
fn test_user_list_to_table_with_multiple_users() {
    let list = UserList(create_test_users());

    let result = list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("Email"));
    assert!(table.contains("Alice Admin"));
    assert!(table.contains("Bob Builder"));
    assert!(table.contains("Charlie Contractor"));
}

#[test]
fn test_user_list_to_table_shows_role_column() {
    let list = UserList(create_test_users());

    let result = list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Role"));
    assert!(table.contains("Admin"));
    assert!(table.contains("Member"));
    assert!(table.contains("Guest"));
}

#[test]
fn test_user_list_to_table_shows_active_column() {
    let list = UserList(create_test_users());

    let result = list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Active"));
}
