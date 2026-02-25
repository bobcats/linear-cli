use linear_cli::output::Formattable;
use linear_cli::states::types::{WorkflowState, WorkflowStateList};

fn create_test_states() -> Vec<WorkflowState> {
    vec![
        WorkflowState {
            id: "state-1".to_string(),
            name: "Backlog".to_string(),
            state_type: "backlog".to_string(),
            color: "#bec2c8".to_string(),
            position: 0.0,
            description: Some("Issues waiting to be picked up".to_string()),
            team_name: Some("Engineering".to_string()),
        },
        WorkflowState {
            id: "state-2".to_string(),
            name: "In Progress".to_string(),
            state_type: "started".to_string(),
            color: "#f2c94c".to_string(),
            position: 1.0,
            description: None,
            team_name: Some("Engineering".to_string()),
        },
        WorkflowState {
            id: "state-3".to_string(),
            name: "Done".to_string(),
            state_type: "completed".to_string(),
            color: "#5e6ad2".to_string(),
            position: 2.0,
            description: Some("Completed work".to_string()),
            team_name: Some("Engineering".to_string()),
        },
    ]
}

#[test]
fn test_state_list_to_json_with_empty_list() {
    let list = WorkflowStateList(vec![]);

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.is_array());
    assert_eq!(v.as_array().unwrap().len(), 0);
}

#[test]
fn test_state_list_to_json_with_multiple_states() {
    let list = WorkflowStateList(create_test_states());

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["name"], "Backlog");
    assert_eq!(arr[1]["name"], "In Progress");
    assert_eq!(arr[2]["name"], "Done");
}

#[test]
fn test_state_list_to_csv_with_empty_list() {
    let list = WorkflowStateList(vec![]);

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 1, "Expected header only");
}

#[test]
fn test_state_list_to_csv_with_multiple_states() {
    let list = WorkflowStateList(create_test_states());

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 4, "Expected header + 3 data rows");
    assert!(csv.contains("Backlog"));
    assert!(csv.contains("In Progress"));
    assert!(csv.contains("Done"));
}

#[test]
fn test_state_list_to_markdown_with_empty_list() {
    let list = WorkflowStateList(vec![]);

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(0)"));
}

#[test]
fn test_state_list_to_markdown_with_multiple_states() {
    let list = WorkflowStateList(create_test_states());

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(3)"));
    assert!(md.contains("Backlog"));
    assert!(md.contains("In Progress"));
    assert!(md.contains("Done"));
    assert!(md.contains("backlog"));
    assert!(md.contains("started"));
    assert!(md.contains("completed"));
}

#[test]
fn test_state_list_to_table_with_empty_list() {
    let list = WorkflowStateList(vec![]);

    let result = list.to_table();

    assert!(result.is_ok());
}

#[test]
fn test_state_list_to_table_with_multiple_states() {
    let list = WorkflowStateList(create_test_states());

    let result = list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("Type"));
    assert!(table.contains("Backlog"));
    assert!(table.contains("In Progress"));
    assert!(table.contains("Done"));
    assert!(table.contains("backlog"));
    assert!(table.contains("started"));
    assert!(table.contains("completed"));
}
