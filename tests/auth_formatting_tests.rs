use linear_cli::auth::output::{AuthStatus, LogoutResult, TokenSource};
use linear_cli::output::Formattable;

// ============================================================================
// AuthStatus Formatter Tests
// ============================================================================

fn create_test_auth_status(show_full_token: bool) -> AuthStatus {
    AuthStatus {
        logged_in: true,
        user_name: "Alice Smith".to_string(),
        user_email: "alice@example.com".to_string(),
        token: if show_full_token {
            "lin_api_demo_token".to_string()
        } else {
            "lin_api_12***".to_string()
        },
        token_source: TokenSource::Keyring,
        show_full_token,
    }
}

#[test]
fn test_auth_status_to_json() {
    let status = create_test_auth_status(false);
    let json = status.to_json().unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["logged_in"], true);
    assert_eq!(parsed["user_name"], "Alice Smith");
    assert_eq!(parsed["user_email"], "alice@example.com");
    assert_eq!(parsed["token"], "lin_api_12***");
    assert_eq!(parsed["token_source"], "keyring");
}

#[test]
fn test_auth_status_to_csv() {
    let status = create_test_auth_status(false);
    let csv = status.to_csv().unwrap();

    assert!(csv.contains("logged_in"));
    assert!(csv.contains("user_name"));
    assert!(csv.contains("true"));
    assert!(csv.contains("Alice Smith"));
    assert!(csv.contains("alice@example.com"));
    assert!(csv.contains("keyring"));

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2); // header + data
}

#[test]
fn test_auth_status_to_markdown_with_redacted_token() {
    let status = create_test_auth_status(false);
    let md = status.to_markdown().unwrap();

    assert!(md.contains("# Linear Authentication Status"));
    assert!(md.contains("✓ **Logged in**"));
    assert!(md.contains("Alice Smith"));
    assert!(md.contains("alice@example.com"));
    assert!(md.contains("lin_api_12***"));
    assert!(md.contains("keyring"));

    // Should NOT have warning
    assert!(!md.contains("Warning"));
}

#[test]
fn test_auth_status_to_markdown_with_full_token() {
    let status = create_test_auth_status(true);
    let md = status.to_markdown().unwrap();

    assert!(md.contains("lin_api_demo_token"));
    // Should HAVE warning
    assert!(md.contains("Warning"));
    assert!(md.contains("Keep it secret"));
}

#[test]
fn test_auth_status_to_table() {
    let status = create_test_auth_status(false);
    let table = status.to_table().unwrap();

    assert!(table.contains("Status") || table.contains("status"));
    assert!(table.contains("Logged in") || table.contains("logged in"));
    assert!(table.contains("Alice Smith"));
    assert!(table.contains("alice@example.com"));
    assert!(table.contains("lin_api_12***"));
}

#[test]
fn test_auth_status_to_table_shows_warning_with_full_token() {
    let status = create_test_auth_status(true);
    let table = status.to_table().unwrap();

    assert!(table.contains("Warning"));
}

// ============================================================================
// LogoutResult Formatter Tests
// ============================================================================

#[test]
fn test_logout_result_success_to_json() {
    let result = LogoutResult {
        success: true,
        message: "Token removed from keyring".to_string(),
    };

    let json = result.to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["message"], "Token removed from keyring");
}

#[test]
fn test_logout_result_to_csv() {
    let result = LogoutResult {
        success: true,
        message: "Token removed from keyring".to_string(),
    };

    let csv = result.to_csv().unwrap();

    assert!(csv.contains("success"));
    assert!(csv.contains("message"));
    assert!(csv.contains("true"));
    assert!(csv.contains("Token removed from keyring"));

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2); // header + data
}

#[test]
fn test_logout_result_to_markdown_success() {
    let result = LogoutResult {
        success: true,
        message: "Token removed from keyring".to_string(),
    };

    let md = result.to_markdown().unwrap();

    assert!(md.contains("# Logout"));
    assert!(md.contains("✓ **Success**"));
    assert!(md.contains("Token removed from keyring"));
}

#[test]
fn test_logout_result_to_markdown_failure() {
    let result = LogoutResult {
        success: false,
        message: "Failed to access keyring".to_string(),
    };

    let md = result.to_markdown().unwrap();

    assert!(md.contains("✗ **Failed**"));
    assert!(md.contains("Failed to access keyring"));
}

#[test]
fn test_logout_result_to_table() {
    let result = LogoutResult {
        success: true,
        message: "Token removed from keyring".to_string(),
    };

    let table = result.to_table().unwrap();

    assert!(table.contains("Status") || table.contains("status"));
    assert!(table.contains("Success") || table.contains("success"));
    assert!(table.contains("Token removed from keyring"));
}
