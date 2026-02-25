use linear_cli::output::{Formattable, generic_table_list_formatter};
use linear_cli::projects::types::{Project, ProjectList};
use linear_cli::teams::types::{Team, TeamList};

fn make_team(index: usize) -> Team {
    Team {
        id: format!("team-{index}"),
        key: format!("T{index:03}"),
        name: format!("Team {index}"),
        description: Some(format!("Description for team {index}")),
        color: Some("#123456".to_string()),
        icon: Some("⚙️".to_string()),
        private: index.is_multiple_of(2),
        created_at: "2026-02-24T00:00:00Z".to_string(),
    }
}

fn make_project(index: usize) -> Project {
    Project {
        id: format!("project-{index}"),
        name: format!("Project {index}"),
        description: format!("Description for project {index}"),
        content: None,
        slug_id: format!("project-{index}"),
        url: format!("https://linear.app/project/{index}"),
        color: "#2563eb".to_string(),
        icon: Some("📦".to_string()),
        status_name: "In Progress".to_string(),
        status_type: "started".to_string(),
        status_color: "#60a5fa".to_string(),
        progress: 0.5,
        priority: 2,
        priority_label: "High".to_string(),
        start_date: Some("2026-02-01".to_string()),
        target_date: Some("2026-03-01".to_string()),
        created_at: "2026-02-01T00:00:00Z".to_string(),
        updated_at: "2026-02-24T00:00:00Z".to_string(),
        lead_name: Some("Alice".to_string()),
    }
}

#[test]
fn test_large_team_table_parity_with_generic_list_formatter() {
    let teams: Vec<Team> = (0..500).map(make_team).collect();
    let expected = TeamList(teams.clone())
        .to_table()
        .expect("existing team list formatter should succeed");

    let actual =
        generic_table_list_formatter(&teams, &["Key", "Name", "Private", "Description"], |team| {
            vec![
                team.key.clone(),
                team.name.clone(),
                if team.private { "Yes" } else { "No" }.to_string(),
                team.description.clone().unwrap_or_else(|| "—".to_string()),
            ]
        })
        .expect("generic list formatter should succeed");

    assert_eq!(actual, expected);
}

#[test]
fn test_large_project_table_parity_with_generic_list_formatter() {
    let projects: Vec<Project> = (0..500).map(make_project).collect();
    let expected = ProjectList(projects.clone())
        .to_table()
        .expect("existing project list formatter should succeed");

    let actual = generic_table_list_formatter(
        &projects,
        &["ID", "Name", "Status", "Progress", "Priority", "Lead"],
        |project| {
            vec![
                project.id.clone(),
                project.name.clone(),
                project.status_name.clone(),
                format!("{:.0}%", project.progress * 100.0),
                project.priority_label.clone(),
                project.lead_name.clone().unwrap_or_else(|| "—".to_string()),
            ]
        },
    )
    .expect("generic list formatter should succeed");

    assert_eq!(actual, expected);
}
