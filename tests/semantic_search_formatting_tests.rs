use linear_cli::output::Formattable;
use linear_cli::search::types::{SemanticSearchResult, SemanticSearchResultList};

fn make_issue_result() -> SemanticSearchResult {
    SemanticSearchResult {
        id: "result-1".to_string(),
        result_type: "issue".to_string(),
        title: "Fix authentication bug".to_string(),
        identifier: Some("ENG-123".to_string()),
        url: Some("https://linear.app/issue/ENG-123".to_string()),
    }
}

fn make_project_result() -> SemanticSearchResult {
    SemanticSearchResult {
        id: "result-2".to_string(),
        result_type: "project".to_string(),
        title: "Q1 Platform Migration".to_string(),
        identifier: None,
        url: None,
    }
}

#[test]
fn test_semantic_result_to_json() {
    let result = make_issue_result();
    let json = result.to_json().expect("should format as JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    assert_eq!(parsed["result_type"], "issue");
    assert_eq!(parsed["title"], "Fix authentication bug");
    assert_eq!(parsed["identifier"], "ENG-123");
}

#[test]
fn test_semantic_result_to_csv() {
    let result = make_issue_result();
    let csv = result.to_csv().expect("should format as CSV");
    assert!(csv.contains("result_type"));
    assert!(csv.contains("issue"));
    assert!(csv.contains("ENG-123"));
}

#[test]
fn test_semantic_result_to_markdown() {
    let result = make_issue_result();
    let md = result.to_markdown().expect("should format as markdown");
    assert!(md.contains("Fix authentication bug"));
    assert!(md.contains("issue"));
}

#[test]
fn test_semantic_result_list_to_json() {
    let results = SemanticSearchResultList(vec![make_issue_result(), make_project_result()]);
    let json = results.to_json().expect("should format as JSON");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");
    let arr = parsed.as_array().expect("should be array");
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["result_type"], "issue");
    assert_eq!(arr[1]["result_type"], "project");
}

#[test]
fn test_semantic_result_list_to_csv() {
    let results = SemanticSearchResultList(vec![make_issue_result(), make_project_result()]);
    let csv = results.to_csv().expect("should format as CSV");
    assert!(csv.contains("ENG-123"));
    assert!(csv.contains("Q1 Platform Migration"));
}

#[test]
fn test_semantic_result_list_empty() {
    let results = SemanticSearchResultList(vec![]);
    let json = results.to_json().expect("should format empty list");
    assert_eq!(json, "[]");
}

#[test]
fn test_semantic_result_list_to_table() {
    let results = SemanticSearchResultList(vec![make_issue_result(), make_project_result()]);
    let table = results.to_table().expect("should format as table");
    assert!(table.contains("ENG-123") || table.contains("Fix authentication bug"));
    assert!(table.contains("Q1 Platform Migration"));
}
