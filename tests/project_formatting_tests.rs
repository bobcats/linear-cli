use linear_cli::output::Formattable;
use linear_cli::projects::types::Project;

fn create_test_project() -> Project {
    Project {
        id: "project-123".to_string(),
        name: "Backend Services".to_string(),
        description: "Core backend infrastructure and APIs".to_string(),
        content: None,
        slug_id: "backend-services".to_string(),
        url: "https://linear.app/team/project/backend-services".to_string(),
        color: "#FF6900".to_string(),
        icon: Some("🔧".to_string()),
        status_name: "In Progress".to_string(),
        status_type: "started".to_string(),
        status_color: "#5E6AD2".to_string(),
        progress: 0.65,
        priority: 2,
        priority_label: "High".to_string(),
        start_date: Some("2024-01-15".to_string()),
        target_date: Some("2024-03-31".to_string()),
        lead_name: Some("John Doe".to_string()),
        created_at: "2024-01-10T10:00:00Z".to_string(),
        updated_at: "2024-02-15T14:30:00Z".to_string(),
    }
}

#[test]
fn test_project_to_json() {
    let project = create_test_project();
    let json = project.to_json().unwrap();

    assert!(json.contains("Backend Services"));
    assert!(json.contains("project-123"));
    assert!(json.contains("In Progress"));
    assert!(json.contains("0.65")); // progress
}

#[test]
fn test_project_to_table() {
    let project = create_test_project();
    let table = project.to_table().unwrap();

    // Vertical layout - field names as first column
    assert!(table.contains("Name"));
    assert!(table.contains("Backend Services"));
    assert!(table.contains("Status"));
    assert!(table.contains("In Progress"));
    assert!(table.contains("Progress"));
}

#[test]
fn test_project_to_csv() {
    let project = create_test_project();
    let csv = project.to_csv().unwrap();

    // Should have header row
    assert!(csv.contains("name"));
    assert!(csv.contains("status"));
    assert!(csv.contains("progress"));

    // Should have data
    assert!(csv.contains("Backend Services"));
    assert!(csv.contains("In Progress"));
}

#[test]
fn test_project_to_markdown() {
    let project = create_test_project();
    let markdown = project.to_markdown().unwrap();

    // Should have title
    assert!(markdown.contains("# Backend Services"));

    // Should have status
    assert!(markdown.contains("**Status:**"));
    assert!(markdown.contains("In Progress"));

    // Should have progress
    assert!(markdown.contains("**Progress:**"));
}

#[test]
fn test_project_table_shows_progress_as_percentage() {
    let project = create_test_project();
    let table = project.to_table().unwrap();

    // Progress should be shown as percentage
    assert!(table.contains("65%"));
}

#[test]
fn test_project_markdown_shows_status_and_dates() {
    let project = create_test_project();
    let markdown = project.to_markdown().unwrap();

    assert!(markdown.contains("**Status:** In Progress"));
    assert!(markdown.contains("2024-01-15"));
    assert!(markdown.contains("2024-03-31"));
}

#[test]
fn test_project_with_content_serializes_to_json() {
    let mut project = create_test_project();
    project.content = Some("# Project Overview\n\nThis is the detailed content.".to_string());

    let json = project.to_json().unwrap();
    assert!(json.contains("Project Overview"));
    assert!(json.contains("detailed content"));
}

#[test]
fn test_project_markdown_includes_content() {
    let mut project = create_test_project();
    project.content = Some("# Project Overview\n\nThis is the detailed content.".to_string());

    let markdown = project.to_markdown().unwrap();
    assert!(markdown.contains("## Content"));
    assert!(markdown.contains("Project Overview"));
    assert!(markdown.contains("detailed content"));
}

#[test]
fn test_project_markdown_omits_empty_content() {
    let mut project = create_test_project();
    project.content = Some(String::new());

    let markdown = project.to_markdown().unwrap();
    assert!(!markdown.contains("## Content"));
}
