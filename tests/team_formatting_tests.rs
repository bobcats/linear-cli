use linear_cli::output::Formattable;
use linear_cli::teams::types::Team;

#[test]
fn test_team_to_json() {
    // Arrange
    let team = Team {
        id: "team-123".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    };

    // Act
    let result = team.to_json();

    // Assert
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("\"id\":\"team-123\""));
    assert!(json.contains("\"key\":\"ENG\""));
    assert!(json.contains("\"name\":\"Engineering\""));
    assert!(json.contains("\"description\":\"Engineering team\""));
}

#[test]
fn test_team_to_table() {
    // Arrange
    let team = Team {
        id: "team-123".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    };

    // Act
    let result = team.to_table();

    // Assert
    assert!(result.is_ok());
    let table = result.unwrap();

    // Verify key fields are present (vertical layout like Issue)
    assert!(table.contains("ENG"));
    assert!(table.contains("Engineering"));
    assert!(table.contains("Engineering team"));
    assert!(table.contains("Key"));
    assert!(table.contains("Name"));
}

#[test]
fn test_team_to_csv() {
    // Arrange
    let team = Team {
        id: "team-123".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    };

    // Act
    let result = team.to_csv();

    // Assert
    assert!(result.is_ok());
    let csv = result.unwrap();

    // Should have header row
    assert!(csv.contains("key,name"));
    // Should have data row with team values
    assert!(csv.contains("ENG"));
    assert!(csv.contains("Engineering"));
}

#[test]
fn test_team_to_markdown() {
    // Arrange
    let team = Team {
        id: "team-123".to_string(),
        key: "ENG".to_string(),
        name: "Engineering".to_string(),
        description: Some("Engineering team".to_string()),
        color: Some("#FF6900".to_string()),
        icon: Some("🔧".to_string()),
        private: false,
        created_at: "2024-01-15T10:30:00Z".to_string(),
    };

    // Act
    let result = team.to_markdown();

    // Assert
    assert!(result.is_ok());
    let markdown = result.unwrap();

    // Should have heading with team key and name
    assert!(markdown.contains("# ENG"));
    assert!(markdown.contains("Engineering"));
    assert!(markdown.contains("**")); // Bold markers for fields
}
