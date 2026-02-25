use linear_cli::output::Formattable;
use linear_cli::teams::types::{Team, TeamList};

fn create_test_teams() -> Vec<Team> {
    vec![
        Team {
            id: "team-1".to_string(),
            key: "ENG".to_string(),
            name: "Engineering".to_string(),
            description: Some("Engineering team".to_string()),
            color: Some("#FF6900".to_string()),
            icon: Some("🔧".to_string()),
            private: false,
            created_at: "2024-01-15T10:30:00Z".to_string(),
        },
        Team {
            id: "team-2".to_string(),
            key: "DESIGN".to_string(),
            name: "Design".to_string(),
            description: None,
            color: Some("#8B5CF6".to_string()),
            icon: Some("🎨".to_string()),
            private: true,
            created_at: "2024-01-16T10:30:00Z".to_string(),
        },
    ]
}

#[test]
fn test_team_list_to_json() {
    let teams = create_test_teams();
    let team_list = TeamList(teams);

    let result = team_list.to_json();

    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("\"key\":\"ENG\""));
    assert!(json.contains("\"key\":\"DESIGN\""));
    assert!(json.contains("Engineering"));
    assert!(json.contains("Design"));
}

#[test]
fn test_team_list_to_table() {
    let teams = create_test_teams();
    let team_list = TeamList(teams);

    let result = team_list.to_table();

    assert!(result.is_ok());
    let table = result.unwrap();
    // Should be horizontal layout with columns
    assert!(table.contains("Key"));
    assert!(table.contains("Name"));
    assert!(table.contains("ENG"));
    assert!(table.contains("DESIGN"));
    assert!(table.contains("Engineering"));
    assert!(table.contains("Design"));
}

#[test]
fn test_team_list_to_csv() {
    let teams = create_test_teams();
    let team_list = TeamList(teams);

    let result = team_list.to_csv();

    assert!(result.is_ok());
    let csv = result.unwrap();
    // Should have header
    assert!(csv.contains("key"));
    assert!(csv.contains("name"));
    // Should have data
    assert!(csv.contains("ENG"));
    assert!(csv.contains("DESIGN"));
}

#[test]
fn test_team_list_to_markdown() {
    let teams = create_test_teams();
    let team_list = TeamList(teams);

    let result = team_list.to_markdown();

    assert!(result.is_ok());
    let markdown = result.unwrap();
    // Should have header with count
    assert!(markdown.contains("Teams"));
    assert!(markdown.contains("(2)"));
    // Should have team info
    assert!(markdown.contains("ENG"));
    assert!(markdown.contains("Engineering"));
}
