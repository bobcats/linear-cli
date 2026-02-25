use linear_cli::projects::types::Project;

#[test]
fn test_project_serializes_to_json() {
    let project = Project {
        id: "project-1".to_string(),
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
    };

    let json = serde_json::to_string(&project).unwrap();
    assert!(json.contains("Backend Services"));
    assert!(json.contains("project-1"));
    assert!(json.contains("In Progress"));
}
