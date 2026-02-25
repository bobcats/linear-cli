use linear_cli::teams::types::Team;

#[test]
fn test_team_serializes_to_json() {
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
    let json_result = serde_json::to_string(&team);

    // Assert
    assert!(json_result.is_ok());
    let json = json_result.unwrap();
    assert!(json.contains("\"id\":\"team-123\""));
    assert!(json.contains("\"key\":\"ENG\""));
    assert!(json.contains("\"name\":\"Engineering\""));
}
