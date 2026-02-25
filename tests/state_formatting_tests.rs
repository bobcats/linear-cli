use linear_cli::output::Formattable;
use linear_cli::states::types::WorkflowState;

fn make_state(name: &str, state_type: &str, color: &str) -> WorkflowState {
    WorkflowState {
        id: format!("state-{}", name.to_lowercase().replace(' ', "-")),
        name: name.to_string(),
        state_type: state_type.to_string(),
        color: color.to_string(),
        position: 1.0,
        description: None,
        team_name: None,
    }
}

fn make_full_state() -> WorkflowState {
    WorkflowState {
        id: "state-123".to_string(),
        name: "In Progress".to_string(),
        state_type: "started".to_string(),
        color: "#f2c94c".to_string(),
        position: 2.0,
        description: Some("Work currently being done".to_string()),
        team_name: Some("Engineering".to_string()),
    }
}

#[test]
fn test_state_to_json() {
    let state = make_full_state();

    let result = state.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["id"], "state-123");
    assert_eq!(v["name"], "In Progress");
    assert_eq!(v["state_type"], "started");
    assert_eq!(v["color"], "#f2c94c");
    assert_eq!(v["position"], 2.0);
    assert_eq!(v["description"], "Work currently being done");
    assert_eq!(v["team_name"], "Engineering");
}

#[test]
fn test_state_to_json_minimal() {
    let state = make_state("Backlog", "backlog", "#bec2c8");

    let result = state.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["name"], "Backlog");
    assert_eq!(v["state_type"], "backlog");
    assert!(v["description"].is_null());
    assert!(v["team_name"].is_null());
}

#[test]
fn test_state_to_csv_has_header_and_single_row() {
    let state = make_full_state();

    let result = state.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 2, "Expected header + 1 data row");
    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("type"));
    assert!(lines[0].contains("color"));
    assert!(lines[1].contains("In Progress"));
    assert!(lines[1].contains("started"));
}

#[test]
fn test_state_to_csv_with_minimal_data() {
    let state = make_state("Backlog", "backlog", "#bec2c8");

    let result = state.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    assert!(csv.contains("Backlog"));
    assert!(csv.contains("backlog"));
}

#[test]
fn test_state_to_markdown() {
    let state = make_full_state();

    let result = state.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# In Progress"));
    assert!(md.contains("started"));
    assert!(md.contains("#f2c94c"));
    assert!(md.contains("Work currently being done"));
    assert!(md.contains("Engineering"));
}

#[test]
fn test_state_to_markdown_minimal() {
    let state = make_state("Backlog", "backlog", "#bec2c8");

    let result = state.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# Backlog"));
    assert!(md.contains("backlog"));
    assert!(!md.contains("Description"));
    assert!(!md.contains("Team"));
}

#[test]
fn test_state_to_table() {
    let state = make_full_state();

    let result = state.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("In Progress"));
    assert!(table.contains("Type"));
    assert!(table.contains("started"));
    assert!(table.contains("Color"));
    assert!(table.contains("#f2c94c"));
    assert!(table.contains("Description"));
    assert!(table.contains("Work currently being done"));
    assert!(table.contains("Team"));
    assert!(table.contains("Engineering"));
}

#[test]
fn test_state_to_table_minimal() {
    let state = make_state("Backlog", "backlog", "#bec2c8");

    let result = state.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Backlog"));
    assert!(table.contains("backlog"));
    // Should not contain optional fields
    assert!(!table.contains("Description"));
    assert!(!table.contains("Team"));
}
