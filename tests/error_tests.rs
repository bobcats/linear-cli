use linear_cli::error::{CliError, ErrorOutput};

#[test]
fn test_not_found_error_has_exit_code_2() {
    let error = CliError::NotFound("Issue ENG-123 not found".to_string());
    assert_eq!(error.exit_code(), 2);
}

#[test]
fn test_auth_error_has_exit_code_3() {
    let error = CliError::AuthError("Invalid token".to_string());
    assert_eq!(error.exit_code(), 3);
}

#[test]
fn test_invalid_args_error_has_exit_code_4() {
    let error = CliError::InvalidArgs("Missing required flag".to_string());
    assert_eq!(error.exit_code(), 4);
}

#[test]
fn test_network_error_has_exit_code_5() {
    let error = CliError::NetworkError("Connection timeout".to_string());
    assert_eq!(error.exit_code(), 5);
}

#[test]
fn test_rate_limit_error_has_exit_code_6() {
    let error = CliError::RateLimitExceeded("Rate limit exceeded".to_string());
    assert_eq!(error.exit_code(), 6);
}

#[test]
fn test_general_error_has_exit_code_1() {
    let error = CliError::General("Something went wrong".to_string());
    assert_eq!(error.exit_code(), 1);
}

#[test]
fn test_error_serializes_to_json() {
    let error = CliError::NotFound("Issue ENG-123 not found".to_string());
    let output: ErrorOutput = error.into();

    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"code\":\"NOT_FOUND\""));
    assert!(json.contains("\"message\":\"Issue ENG-123 not found\""));
    assert!(json.contains("\"type\":\"IssueNotFoundError\""));
}

#[test]
fn test_auth_error_serializes_to_json() {
    let error = CliError::AuthError("Invalid token".to_string());
    let output: ErrorOutput = error.into();

    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"code\":\"AUTH_ERROR\""));
    assert!(json.contains("\"message\":\"Invalid token\""));
    assert!(json.contains("\"type\":\"AuthenticationError\""));
}

#[test]
fn test_error_output_structure() {
    let error = CliError::NetworkError("Connection failed".to_string());
    let output: ErrorOutput = error.into();

    assert_eq!(output.error.code, "NETWORK_ERROR");
    assert_eq!(output.error.message, "Connection failed");
    assert_eq!(output.error.error_type, "NetworkError");
}
