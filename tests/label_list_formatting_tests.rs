use linear_cli::labels::types::{IssueLabel, IssueLabelList};
use linear_cli::output::Formattable;

fn create_test_labels() -> Vec<IssueLabel> {
    vec![
        IssueLabel {
            id: "label-1".to_string(),
            name: "Bug".to_string(),
            color: "#eb5757".to_string(),
            description: Some("Something is broken".to_string()),
            is_group: false,
            parent_name: None,
        },
        IssueLabel {
            id: "label-2".to_string(),
            name: "Feature".to_string(),
            color: "#4ea7fc".to_string(),
            description: Some("New functionality".to_string()),
            is_group: false,
            parent_name: None,
        },
        IssueLabel {
            id: "label-3".to_string(),
            name: "Priority".to_string(),
            color: "#000000".to_string(),
            description: None,
            is_group: true,
            parent_name: None,
        },
    ]
}

#[test]
fn test_label_list_to_json_with_empty_list() {
    let list = IssueLabelList(vec![]);

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.is_array());
    assert_eq!(v.as_array().unwrap().len(), 0);
}

#[test]
fn test_label_list_to_json_with_multiple_labels() {
    let list = IssueLabelList(create_test_labels());

    let result = list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    let arr = v.as_array().unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["name"], "Bug");
    assert_eq!(arr[1]["name"], "Feature");
    assert_eq!(arr[2]["name"], "Priority");
}

#[test]
fn test_label_list_to_csv_with_empty_list() {
    let list = IssueLabelList(vec![]);

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 1, "Expected header only");
}

#[test]
fn test_label_list_to_csv_with_multiple_labels() {
    let list = IssueLabelList(create_test_labels());

    let result = list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 4, "Expected header + 3 data rows");
    assert!(csv.contains("Bug"));
    assert!(csv.contains("Feature"));
    assert!(csv.contains("Priority"));
}

#[test]
fn test_label_list_to_markdown_with_empty_list() {
    let list = IssueLabelList(vec![]);

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(0)"));
}

#[test]
fn test_label_list_to_markdown_with_multiple_labels() {
    let list = IssueLabelList(create_test_labels());

    let result = list.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("(3)"));
    assert!(md.contains("Bug"));
    assert!(md.contains("Feature"));
    assert!(md.contains("Priority"));
}

#[test]
fn test_label_list_to_table_with_empty_list() {
    let list = IssueLabelList(vec![]);

    let result = list.to_table();

    assert!(result.is_ok());
}

#[test]
fn test_label_list_to_table_with_multiple_labels() {
    let list = IssueLabelList(create_test_labels());

    let result = list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("Color"));
    assert!(table.contains("Bug"));
    assert!(table.contains("Feature"));
    assert!(table.contains("Priority"));
}
