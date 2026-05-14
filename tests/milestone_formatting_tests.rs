use linear_cli::milestones::types::{Milestone, MilestoneList, MilestoneProject};
use linear_cli::output::Formattable;

fn sample_milestone() -> Milestone {
    Milestone {
        id: "milestone-1".to_string(),
        name: "Beta".to_string(),
        description: Some("Beta readiness".to_string()),
        status: "next".to_string(),
        progress: 0.5,
        sort_order: 1000.0,
        target_date: Some("2026-06-30".to_string()),
        project: MilestoneProject {
            id: "project-1".to_string(),
            name: "App".to_string(),
            slug_id: "app".to_string(),
        },
        created_at: "2026-05-01T00:00:00Z".to_string(),
        updated_at: "2026-05-02T00:00:00Z".to_string(),
        archived_at: None,
    }
}

#[test]
fn milestone_json_includes_project_and_status() {
    let json = sample_milestone().to_json().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["name"], "Beta");
    assert_eq!(parsed["status"], "next");
    assert_eq!(parsed["project"]["slug_id"], "app");
}

#[test]
fn milestone_csv_includes_core_fields() {
    let csv = sample_milestone().to_csv().unwrap();

    assert!(csv.contains("id,name,project"));
    assert!(csv.contains("milestone-1"));
    assert!(csv.contains("Beta"));
    assert!(csv.contains("App"));
}

#[test]
fn milestone_markdown_includes_target_date() {
    let markdown = sample_milestone().to_markdown().unwrap();

    assert!(markdown.contains("# Beta"));
    assert!(markdown.contains("**Project:** App"));
    assert!(markdown.contains("2026-06-30"));
}

#[test]
fn milestone_list_table_includes_progress() {
    let table = MilestoneList(vec![sample_milestone()]).to_table().unwrap();

    assert!(table.contains("Beta"));
    assert!(table.contains("50%"));
}
