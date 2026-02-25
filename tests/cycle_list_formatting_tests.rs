use linear_cli::cycles::types::{Cycle, CycleList};
use linear_cli::output::Formattable;

fn create_test_cycles(count: usize) -> Vec<Cycle> {
    (0..count)
        .map(|i| Cycle {
            id: format!("cycle-{i}"),
            name: format!("Sprint {}", 20 + i),
            number: (20 + i) as f64,
            description: if i.is_multiple_of(2) {
                Some(format!("Description for sprint {}", 20 + i))
            } else {
                None
            },
            starts_at: format!("2024-{:02}-01", 1 + i),
            ends_at: format!("2024-{:02}-15", 1 + i),
            created_at: format!("2024-{:02}-01T10:00:00Z", 1 + i),
            completed_at: if i.is_multiple_of(3) {
                Some(format!("2024-{:02}-15T18:00:00Z", 1 + i))
            } else {
                None
            },
            progress: (i as f64 * 0.2) % 1.0,
            is_active: i == 1,
            is_future: i > 2,
            is_next: i == 2,
            is_past: i == 0,
            is_previous: i == 0,
            team_name: format!("Team {}", i % 3),
            team_key: match i % 3 {
                0 => "ENG".to_string(),
                1 => "PROD".to_string(),
                _ => "DES".to_string(),
            },
        })
        .collect()
}

#[test]
fn test_cycle_list_to_json_with_empty_list() {
    let list = CycleList(vec![]);
    let json = list.to_json().unwrap();

    assert_eq!(json.trim(), "[]");
}

#[test]
fn test_cycle_list_to_json_with_single_cycle() {
    let cycles = create_test_cycles(1);
    let list = CycleList(cycles);
    let json = list.to_json().unwrap();

    assert!(json.contains("\"id\":\"cycle-0\""));
    assert!(json.contains("\"name\":\"Sprint 20\""));
    assert!(json.starts_with('['));
    assert!(json.ends_with(']'));
}

#[test]
fn test_cycle_list_to_json_with_multiple_cycles() {
    let cycles = create_test_cycles(3);
    let list = CycleList(cycles);
    let json = list.to_json().unwrap();

    assert!(json.contains("\"id\":\"cycle-0\""));
    assert!(json.contains("\"id\":\"cycle-1\""));
    assert!(json.contains("\"id\":\"cycle-2\""));
    assert!(json.contains("\"name\":\"Sprint 20\""));
    assert!(json.contains("\"name\":\"Sprint 21\""));
    assert!(json.contains("\"name\":\"Sprint 22\""));
}

#[test]
fn test_cycle_list_to_csv_with_empty_list() {
    let list = CycleList(vec![]);
    let csv = list.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1, "CSV should have only header row");
    assert!(lines[0].contains("name"));
    assert!(lines[0].contains("number"));
}

#[test]
fn test_cycle_list_to_csv_with_multiple_cycles() {
    let cycles = create_test_cycles(3);
    let list = CycleList(cycles);
    let csv = list.to_csv().unwrap();

    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 4, "CSV should have header + 3 data rows");

    // Check that all cycles are present
    assert!(csv.contains("Sprint 20"));
    assert!(csv.contains("Sprint 21"));
    assert!(csv.contains("Sprint 22"));

    // Check status formatting
    assert!(csv.contains("Previous")); // cycle 0
    assert!(csv.contains("Active")); // cycle 1
    assert!(csv.contains("Next")); // cycle 2

    // Check progress formatting
    assert!(csv.contains("0%"));
    assert!(csv.contains("20%"));
    assert!(csv.contains("40%"));

    // Check team formatting
    assert!(csv.contains("Team 0 (ENG)"));
    assert!(csv.contains("Team 1 (PROD)"));
    assert!(csv.contains("Team 2 (DES)"));
}

#[test]
fn test_cycle_list_to_markdown_with_empty_list() {
    let list = CycleList(vec![]);
    let markdown = list.to_markdown().unwrap();

    assert!(markdown.contains("## Cycles (0)"));
    // Empty list should not have separators
    assert!(!markdown.contains("---"));
}

#[test]
fn test_cycle_list_to_markdown_with_multiple_cycles() {
    let cycles = create_test_cycles(3);
    let list = CycleList(cycles);
    let markdown = list.to_markdown().unwrap();

    // Check header
    assert!(markdown.contains("## Cycles (3)"));

    // Check all cycle names
    assert!(markdown.contains("### Sprint 20"));
    assert!(markdown.contains("### Sprint 21"));
    assert!(markdown.contains("### Sprint 22"));

    // Check status, progress, team info
    assert!(markdown.contains("**Status:** Previous"));
    assert!(markdown.contains("**Status:** Active"));
    assert!(markdown.contains("**Status:** Next"));

    assert!(markdown.contains("**Progress:** 0%"));
    assert!(markdown.contains("**Progress:** 20%"));
    assert!(markdown.contains("**Progress:** 40%"));

    assert!(markdown.contains("**Team:** Team 0 (ENG)"));
    assert!(markdown.contains("**Team:** Team 1 (PROD)"));

    // Check dates
    assert!(markdown.contains("**Dates:** 2024-01-01 → 2024-01-15"));
    assert!(markdown.contains("**Dates:** 2024-02-01 → 2024-02-15"));

    // Check separators
    let separator_count = markdown.matches("---").count();
    assert_eq!(separator_count, 4); // 3 between cycles + 1 final
}

#[test]
fn test_cycle_list_to_table_with_empty_list() {
    let list = CycleList(vec![]);
    let table = list.to_table().unwrap();

    // Should have header row even when empty
    assert!(table.contains("Name"));
    assert!(table.contains("Number"));
    assert!(table.contains("Status"));
    assert!(table.contains("Progress"));
}

#[test]
fn test_cycle_list_to_table_uses_horizontal_layout() {
    let cycles = create_test_cycles(2);
    let list = CycleList(cycles);
    let table = list.to_table().unwrap();

    // Horizontal layout means column headers
    assert!(table.contains("Name"));
    assert!(table.contains("Number"));
    assert!(table.contains("Status"));
    assert!(table.contains("Progress"));
    assert!(table.contains("Team"));
    assert!(table.contains("Dates"));
}

#[test]
fn test_cycle_list_to_table_with_multiple_cycles() {
    let cycles = create_test_cycles(3);
    let list = CycleList(cycles);
    let table = list.to_table().unwrap();

    // Check cycle names
    assert!(table.contains("Sprint 20"));
    assert!(table.contains("Sprint 21"));
    assert!(table.contains("Sprint 22"));

    // Check cycle numbers
    assert!(table.contains("#20"));
    assert!(table.contains("#21"));
    assert!(table.contains("#22"));

    // Check status
    assert!(table.contains("Previous"));
    assert!(table.contains("Active"));
    assert!(table.contains("Next"));

    // Check progress
    assert!(table.contains("0%"));
    assert!(table.contains("20%"));
    assert!(table.contains("40%"));

    // Check dates
    assert!(table.contains("2024-01-01 → 2024-01-15"));
    assert!(table.contains("2024-02-01 → 2024-02-15"));
}
