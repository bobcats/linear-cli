use linear_cli::cycles::types::Cycle;
use linear_cli::output::Formattable;

fn create_test_cycle() -> Cycle {
    Cycle {
        id: "cycle-123".to_string(),
        name: "Sprint 24".to_string(),
        number: 24.0,
        description: Some("Q1 planning sprint focused on performance improvements".to_string()),
        starts_at: "2024-01-15".to_string(),
        ends_at: "2024-01-29".to_string(),
        created_at: "2024-01-10T10:00:00Z".to_string(),
        completed_at: None,
        progress: 0.65,
        is_active: true,
        is_future: false,
        is_next: false,
        is_past: false,
        is_previous: false,
        team_name: "Engineering".to_string(),
        team_key: "ENG".to_string(),
    }
}

#[test]
fn test_cycle_to_json_with_full_data() {
    let cycle = create_test_cycle();
    let json = cycle.to_json().unwrap();

    assert!(json.contains("\"id\":\"cycle-123\""));
    assert!(json.contains("\"name\":\"Sprint 24\""));
    assert!(json.contains("\"number\":24"));
    assert!(json.contains("\"progress\":0.65"));
    assert!(json.contains("\"is_active\":true"));
    assert!(json.contains("\"team_name\":\"Engineering\""));
}

#[test]
fn test_cycle_to_json_with_minimal_data() {
    let cycle = Cycle {
        id: "cycle-456".to_string(),
        name: "Sprint 25".to_string(),
        number: 25.0,
        description: None,
        starts_at: "2024-01-29".to_string(),
        ends_at: "2024-02-12".to_string(),
        created_at: "2024-01-25T10:00:00Z".to_string(),
        completed_at: None,
        progress: 0.0,
        is_active: false,
        is_future: true,
        is_next: true,
        is_past: false,
        is_previous: false,
        team_name: "Product".to_string(),
        team_key: "PROD".to_string(),
    };

    let json = cycle.to_json().unwrap();

    assert!(json.contains("\"id\":\"cycle-456\""));
    assert!(json.contains("\"description\":null"));
    assert!(json.contains("\"is_future\":true"));
    assert!(json.contains("\"is_next\":true"));
}

#[test]
fn test_cycle_to_csv_has_header_and_single_row() {
    let cycle = create_test_cycle();
    let csv = cycle.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 2, "CSV should have header + 1 data row");

    let header = lines[0];
    assert!(header.contains("name"));
    assert!(header.contains("number"));
    assert!(header.contains("team"));
    assert!(header.contains("status"));
    assert!(header.contains("progress"));
}

#[test]
fn test_cycle_to_csv_with_full_data() {
    let cycle = create_test_cycle();
    let csv = cycle.to_csv().unwrap();

    assert!(csv.contains("Sprint 24"));
    assert!(csv.contains("24"));
    assert!(csv.contains("Engineering (ENG)"));
    assert!(csv.contains("Active"));
    assert!(csv.contains("65%"));
    assert!(csv.contains("2024-01-15"));
    assert!(csv.contains("2024-01-29"));
}

#[test]
fn test_cycle_to_csv_with_minimal_data() {
    let cycle = Cycle {
        id: "cycle-789".to_string(),
        name: "Sprint 26".to_string(),
        number: 26.0,
        description: None,
        starts_at: "2024-02-12".to_string(),
        ends_at: "2024-02-26".to_string(),
        created_at: "2024-02-08T10:00:00Z".to_string(),
        completed_at: None,
        progress: 0.0,
        is_active: false,
        is_future: true,
        is_next: false,
        is_past: false,
        is_previous: false,
        team_name: "Design".to_string(),
        team_key: "DES".to_string(),
    };

    let csv = cycle.to_csv().unwrap();

    assert!(csv.contains("Sprint 26"));
    assert!(csv.contains("Future"));
    assert!(csv.contains("0%"));
}

#[test]
fn test_cycle_to_markdown_with_full_data() {
    let cycle = create_test_cycle();
    let markdown = cycle.to_markdown().unwrap();

    // Check title
    assert!(markdown.contains("# Sprint 24 (Cycle #24)"));

    // Check description
    assert!(markdown.contains("Q1 planning sprint"));

    // Check status and progress
    assert!(markdown.contains("**Status:** Active"));
    assert!(markdown.contains("**Progress:** 65%"));

    // Check team
    assert!(markdown.contains("**Team:** Engineering (ENG)"));

    // Check dates
    assert!(markdown.contains("**Start Date:** 2024-01-15"));
    assert!(markdown.contains("**End Date:** 2024-01-29"));

    // Should not show completed date if None
    assert!(!markdown.contains("**Completed:**"));
}

#[test]
fn test_cycle_to_markdown_with_minimal_data() {
    let cycle = Cycle {
        id: "cycle-completed".to_string(),
        name: "Sprint 23".to_string(),
        number: 23.0,
        description: None,
        starts_at: "2024-01-01".to_string(),
        ends_at: "2024-01-14".to_string(),
        created_at: "2023-12-28T10:00:00Z".to_string(),
        completed_at: Some("2024-01-14T18:00:00Z".to_string()),
        progress: 1.0,
        is_active: false,
        is_future: false,
        is_next: false,
        is_past: true,
        is_previous: true,
        team_name: "Engineering".to_string(),
        team_key: "ENG".to_string(),
    };

    let markdown = cycle.to_markdown().unwrap();

    // Check status
    assert!(markdown.contains("**Status:** Previous"));

    // Check completion
    assert!(markdown.contains("**Completed:** 2024-01-14T18:00:00Z"));
    assert!(markdown.contains("**Progress:** 100%"));
}

#[test]
fn test_cycle_to_table_uses_vertical_layout() {
    let cycle = create_test_cycle();
    let table = cycle.to_table().unwrap();

    // Vertical layout means each field is on its own row
    assert!(table.contains("Name"));
    assert!(table.contains("Sprint 24"));
    assert!(table.contains("Number"));
    assert!(table.contains("#24"));
    assert!(table.contains("Status"));
    assert!(table.contains("Active"));
    assert!(table.contains("Progress"));
    assert!(table.contains("65%"));
}

#[test]
fn test_cycle_to_table_with_full_data() {
    let cycle = create_test_cycle();
    let table = cycle.to_table().unwrap();

    assert!(table.contains("Sprint 24"));
    assert!(table.contains("Active"));
    assert!(table.contains("65%"));
    assert!(table.contains("Engineering (ENG)"));
    assert!(table.contains("2024-01-15"));
    assert!(table.contains("2024-01-29"));
    assert!(table.contains("Q1 planning sprint"));
}

#[test]
fn test_cycle_to_table_with_minimal_data() {
    let cycle = Cycle {
        id: "cycle-minimal".to_string(),
        name: "Sprint 27".to_string(),
        number: 27.0,
        description: None,
        starts_at: "2024-02-26".to_string(),
        ends_at: "2024-03-11".to_string(),
        created_at: "2024-02-22T10:00:00Z".to_string(),
        completed_at: None,
        progress: 0.0,
        is_active: false,
        is_future: false,
        is_next: true,
        is_past: false,
        is_previous: false,
        team_name: "Marketing".to_string(),
        team_key: "MKT".to_string(),
    };

    let table = cycle.to_table().unwrap();

    assert!(table.contains("Sprint 27"));
    assert!(table.contains("Next"));
    assert!(table.contains("0%"));
    // Description should not appear when None
    assert!(
        !table.contains("Description")
            || table.lines().filter(|l| l.contains("Description")).count() == 0
    );
}
