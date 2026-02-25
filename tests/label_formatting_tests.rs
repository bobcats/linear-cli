use linear_cli::labels::types::IssueLabel;
use linear_cli::output::Formattable;

fn make_label(name: &str, color: &str) -> IssueLabel {
    IssueLabel {
        id: format!("label-{}", name.to_lowercase().replace(' ', "-")),
        name: name.to_string(),
        color: color.to_string(),
        description: None,
        is_group: false,
        parent_name: None,
    }
}

fn make_full_label() -> IssueLabel {
    IssueLabel {
        id: "label-123".to_string(),
        name: "Bug".to_string(),
        color: "#eb5757".to_string(),
        description: Some("Something is broken".to_string()),
        is_group: false,
        parent_name: Some("Priority".to_string()),
    }
}

#[test]
fn test_label_to_json() {
    let label = make_full_label();

    let result = label.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["id"], "label-123");
    assert_eq!(v["name"], "Bug");
    assert_eq!(v["color"], "#eb5757");
    assert_eq!(v["description"], "Something is broken");
    assert_eq!(v["is_group"], false);
    assert_eq!(v["parent_name"], "Priority");
}

#[test]
fn test_label_to_json_minimal() {
    let label = make_label("Feature", "#4ea7fc");

    let result = label.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(v["name"], "Feature");
    assert!(v["description"].is_null());
    assert!(v["parent_name"].is_null());
}

#[test]
fn test_label_to_csv_has_header_and_single_row() {
    let label = make_full_label();

    let result = label.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    let lines: Vec<&str> = csv.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("color"));
    assert!(lines[1].contains("Bug"));
    assert!(lines[1].contains("#eb5757"));
}

#[test]
fn test_label_to_csv_with_minimal_data() {
    let label = make_label("Feature", "#4ea7fc");

    let result = label.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    assert!(csv.contains("Feature"));
}

#[test]
fn test_label_to_markdown() {
    let label = make_full_label();

    let result = label.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# Bug"));
    assert!(md.contains("#eb5757"));
    assert!(md.contains("Something is broken"));
    assert!(md.contains("Priority"));
}

#[test]
fn test_label_to_markdown_with_parent_shows_hierarchy() {
    let label = make_full_label();

    let result = label.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("Parent"));
    assert!(md.contains("Priority"));
}

#[test]
fn test_label_to_markdown_minimal() {
    let label = make_label("Feature", "#4ea7fc");

    let result = label.to_markdown();

    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# Feature"));
    assert!(!md.contains("Parent"));
    assert!(!md.contains("Description"));
}

#[test]
fn test_label_to_table() {
    let label = make_full_label();

    let result = label.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Name"));
    assert!(table.contains("Bug"));
    assert!(table.contains("Color"));
    assert!(table.contains("#eb5757"));
    assert!(table.contains("Description"));
    assert!(table.contains("Something is broken"));
    assert!(table.contains("Parent"));
    assert!(table.contains("Priority"));
}

#[test]
fn test_label_to_table_group_label() {
    let label = IssueLabel {
        id: "label-grp".to_string(),
        name: "Priority".to_string(),
        color: "#000000".to_string(),
        description: None,
        is_group: true,
        parent_name: None,
    };

    let result = label.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.contains("Group"));
    assert!(table.contains("Yes"));
}
