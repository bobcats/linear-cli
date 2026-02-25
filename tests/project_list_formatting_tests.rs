use linear_cli::output::Formattable;
use linear_cli::projects::types::{Project, ProjectList};

fn create_test_projects() -> Vec<Project> {
    vec![
        Project {
            id: "project-1".to_string(),
            name: "Backend Services".to_string(),
            description: "Core backend infrastructure".to_string(),
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
        },
        Project {
            id: "project-2".to_string(),
            name: "Mobile App".to_string(),
            description: "iOS and Android apps".to_string(),
            content: None,
            slug_id: "mobile-app".to_string(),
            url: "https://linear.app/team/project/mobile-app".to_string(),
            color: "#8B5CF6".to_string(),
            icon: Some("📱".to_string()),
            status_name: "Planned".to_string(),
            status_type: "planned".to_string(),
            status_color: "#95A2B3".to_string(),
            progress: 0.0,
            priority: 1,
            priority_label: "Medium".to_string(),
            start_date: None,
            target_date: Some("2024-06-30".to_string()),
            lead_name: None,
            created_at: "2024-02-01T10:00:00Z".to_string(),
            updated_at: "2024-02-01T10:00:00Z".to_string(),
        },
    ]
}

#[test]
fn test_project_list_to_json() {
    let projects = create_test_projects();
    let project_list = ProjectList(projects);

    let result = project_list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("\"name\":\"Backend Services\""));
    assert!(json.contains("\"name\":\"Mobile App\""));
    assert!(json.contains("backend-services"));
    assert!(json.contains("mobile-app"));
}

#[test]
fn test_project_list_to_table() {
    let projects = create_test_projects();
    let project_list = ProjectList(projects);

    let result = project_list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    // Should be horizontal layout with columns
    assert!(table.contains("Name"));
    assert!(table.contains("Status"));
    assert!(table.contains("Progress"));
    assert!(table.contains("Backend Services"));
    assert!(table.contains("Mobile App"));
    assert!(table.contains("In Progress"));
    assert!(table.contains("Planned"));
}

#[test]
fn test_project_list_to_csv() {
    let projects = create_test_projects();
    let project_list = ProjectList(projects);

    let result = project_list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    // Should have header
    assert!(csv.contains("name"));
    assert!(csv.contains("status"));
    assert!(csv.contains("progress"));
    // Should have data
    assert!(csv.contains("Backend Services"));
    assert!(csv.contains("Mobile App"));
    assert!(csv.contains("In Progress"));
}

#[test]
fn test_project_list_to_markdown() {
    let projects = create_test_projects();
    let project_list = ProjectList(projects);

    let result = project_list.to_markdown();

    assert!(result.is_ok());
    let markdown = result.unwrap();
    assert!(markdown.contains("Projects"));
    assert!(markdown.contains("(2)"));
    assert!(markdown.contains("Backend Services"));
    assert!(markdown.contains("Mobile App"));
    assert!(markdown.contains("In Progress"));
    assert!(markdown.contains("65%"));
}

#[test]
fn test_project_list_markdown_includes_content() {
    let mut projects = create_test_projects();
    projects[0].content = Some("Detailed backend documentation".to_string());

    let project_list = ProjectList(projects);
    let markdown = project_list.to_markdown().unwrap();

    assert!(markdown.contains("Detailed backend documentation"));
}
