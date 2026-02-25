use linear_cli::output::{OutputFormat, format_output, format_output_to_writer};
use linear_cli::teams::types::{Team, TeamList};

fn sample_team(id: &str, key: &str, name: &str) -> Team {
    Team {
        id: id.to_string(),
        key: key.to_string(),
        name: name.to_string(),
        description: Some(format!("{name} team")),
        color: Some("#123456".to_string()),
        icon: Some("⚡".to_string()),
        private: false,
        created_at: "2026-02-24T00:00:00Z".to_string(),
    }
}

#[test]
fn test_streaming_single_item_matches_string_output_all_formats() {
    let team = sample_team("team-1", "ENG", "Engineering");

    for format in [
        OutputFormat::Json,
        OutputFormat::Csv,
        OutputFormat::Markdown,
        OutputFormat::Table,
    ] {
        let expected = format_output(&team, format).expect("string output should format");

        let mut bytes = Vec::new();
        format_output_to_writer(&team, format, &mut bytes).expect("streaming output should format");
        let actual = String::from_utf8(bytes).expect("streaming output must be utf-8");

        assert_eq!(actual, expected, "streaming parity failed for {format:?}");
    }
}

#[test]
fn test_streaming_list_matches_string_output_all_formats() {
    let teams = TeamList(vec![
        sample_team("team-1", "ENG", "Engineering"),
        sample_team("team-2", "DES", "Design"),
    ]);

    for format in [
        OutputFormat::Json,
        OutputFormat::Csv,
        OutputFormat::Markdown,
        OutputFormat::Table,
    ] {
        let expected = format_output(&teams, format).expect("string output should format");

        let mut bytes = Vec::new();
        format_output_to_writer(&teams, format, &mut bytes)
            .expect("streaming output should format");
        let actual = String::from_utf8(bytes).expect("streaming output must be utf-8");

        assert_eq!(actual, expected, "streaming parity failed for {format:?}");
    }
}
