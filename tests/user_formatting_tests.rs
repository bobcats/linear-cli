use linear_cli::output::Formattable;
use linear_cli::users::types::User;

fn make_user(name: &str, email: &str) -> User {
    User {
        id: format!("user-{}", name.to_lowercase().replace(' ', "-")),
        name: name.to_string(),
        display_name: name.to_string(),
        email: email.to_string(),
        active: true,
        admin: false,
        guest: false,
    }
}

fn make_full_user() -> User {
    User {
        id: "user-123".to_string(),
        name: "Brian Smith".to_string(),
        display_name: "brian".to_string(),
        email: "brian@example.com".to_string(),
        active: true,
        admin: true,
        guest: false,
    }
}

#[test]
fn test_user_to_json() {
    let user = make_full_user();

    let result = user.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["id"], "user-123");
    assert_eq!(v["name"], "Brian Smith");
    assert_eq!(v["display_name"], "brian");
    assert_eq!(v["email"], "brian@example.com");
    assert_eq!(v["active"], true);
    assert_eq!(v["admin"], true);
    assert_eq!(v["guest"], false);
}

#[test]
fn test_user_to_json_guest() {
    let user = User {
        id: "user-guest".to_string(),
        name: "Guest User".to_string(),
        display_name: "guest".to_string(),
        email: "guest@example.com".to_string(),
        active: true,
        admin: false,
        guest: true,
    };

    let result = user.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["guest"], true);
}

#[test]
fn test_user_to_csv_has_header_and_single_row() {
    let user = make_full_user();

    let result = user.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("email"));
    assert!(lines[1].contains("Brian Smith"));
    assert!(lines[1].contains("brian@example.com"));
}

#[test]
fn test_user_to_markdown() {
    let user = make_full_user();

    let result = user.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# Brian Smith"));
    assert!(md.contains("brian@example.com"));
    assert!(md.contains("brian"));
}

#[test]
fn test_user_to_markdown_inactive() {
    let user = User {
        active: false,
        ..make_user("Inactive Person", "inactive@example.com")
    };

    let result = user.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("Inactive Person"));
    assert!(md.contains("No"));
}

#[test]
fn test_user_to_table() {
    let user = make_full_user();

    let result = user.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("Brian Smith"));
    assert!(table.contains("Email"));
    assert!(table.contains("brian@example.com"));
    assert!(table.contains("Display Name"));
    assert!(table.contains("brian"));
}

#[test]
fn test_user_to_table_shows_role() {
    let user = make_full_user();

    let result = user.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Role"));
    assert!(table.contains("Admin"));
}

#[test]
fn test_user_to_table_guest_role() {
    let user = User {
        guest: true,
        ..make_user("Guest User", "guest@example.com")
    };

    let result = user.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Guest"));
}
