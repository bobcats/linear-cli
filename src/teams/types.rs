use crate::client::queries;
use crate::error::CliError;
use crate::output::{
    CsvResultExt, Formattable, MarkdownFormatter, TableFormatter, fast_markdown_formatter,
    generic_json_formatter, generic_json_list_formatter, generic_table_formatter,
    generic_table_list_formatter,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;

/// Team details returned from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub private: bool,
    pub created_at: String,
}

// Trait implementations for generic formatters

impl TableFormatter for Team {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Key"), Cow::Borrowed(self.key.as_str())),
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())),
        ];

        // Add optional fields
        if let Some(desc) = &self.description {
            rows.push((Cow::Borrowed("Description"), Cow::Borrowed(desc.as_str())));
        }
        if let Some(color) = &self.color {
            rows.push((Cow::Borrowed("Color"), Cow::Borrowed(color.as_str())));
        }
        if let Some(icon) = &self.icon {
            rows.push((Cow::Borrowed("Icon"), Cow::Borrowed(icon.as_str())));
        }

        rows.push((
            Cow::Borrowed("Private"),
            Cow::Borrowed(if self.private { "Yes" } else { "No" }),
        ));
        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));

        rows
    }
}

impl MarkdownFormatter for Team {
    fn markdown_capacity_hint(&self) -> usize {
        200 + self.key.len()
            + self.name.len()
            + self.id.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.color.as_ref().map_or(0, |c| c.len())
            + self.icon.as_ref().map_or(0, |i| i.len())
            + self.created_at.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        // Exact same logic as original custom implementation
        // H1 title with key and name
        writeln!(output, "# {}: {}\n", self.key, self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        // Metadata section
        writeln!(output, "**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown ID: {e}")))?;

        if let Some(desc) = &self.description {
            writeln!(output, "**Description:** {}", desc).map_err(|e| {
                CliError::General(format!("Failed to write markdown description: {e}"))
            })?;
        }

        if let Some(color) = &self.color {
            writeln!(output, "**Color:** {}", color)
                .map_err(|e| CliError::General(format!("Failed to write markdown color: {e}")))?;
        }

        if let Some(icon) = &self.icon {
            writeln!(output, "**Icon:** {}", icon)
                .map_err(|e| CliError::General(format!("Failed to write markdown icon: {e}")))?;
        }

        writeln!(
            output,
            "**Private:** {}",
            if self.private { "Yes" } else { "No" }
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown private: {e}")))?;

        writeln!(output, "**Created:** {}", self.created_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown created: {e}")))?;

        Ok(())
    }
}

impl Formattable for Team {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "key",
            "name",
            "id",
            "description",
            "color",
            "icon",
            "private",
            "created_at",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write data row
        let private_str = if self.private { "true" } else { "false" };
        wtr.write_record([
            &self.key,
            &self.name,
            &self.id,
            self.description.as_deref().unwrap_or(""),
            self.color.as_deref().unwrap_or(""),
            self.icon.as_deref().unwrap_or(""),
            private_str,
            &self.created_at,
        ])
        .csv_err("Failed to write CSV data")?;

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        fast_markdown_formatter(self)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_formatter(self)
    }
}

/// Wrapper type for a list of teams
/// Provides different formatting for collections vs single items
pub struct TeamList(pub Vec<Team>);

impl Formattable for TeamList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record(["key", "name", "id", "description", "private", "created_at"])
            .csv_err("Failed to write CSV header")?;

        // Write each team as a row
        for team in &self.0 {
            let private_str = if team.private { "true" } else { "false" };
            wtr.write_record([
                &team.key,
                &team.name,
                &team.id,
                team.description.as_deref().unwrap_or(""),
                private_str,
                &team.created_at,
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        // Pre-allocate: ~150 bytes per team for structure + actual content
        let capacity = self
            .0
            .iter()
            .map(|t| {
                150 + t.key.len()
                    + t.name.len()
                    + t.id.len()
                    + t.description.as_ref().map_or(0, |d| d.len())
            })
            .sum();
        let mut output = String::with_capacity(capacity);

        // Header with count
        writeln!(output, "## Teams ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;

        // Output each team as a summary card
        for team in &self.0 {
            write!(output, "---\n\n### {}: {}\n", team.key, team.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

            // Metadata on one line
            writeln!(
                output,
                "**ID:** {} | **Private:** {}\n",
                team.id,
                if team.private { "Yes" } else { "No" }
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown metadata: {e}")))?;

            // Description
            if let Some(desc) = &team.description {
                writeln!(output, "{}\n", desc).map_err(|e| {
                    CliError::General(format!("Failed to write markdown description: {e}"))
                })?;
            } else {
                write!(output, "[No description]\n\n").map_err(|e| {
                    CliError::General(format!("Failed to write markdown placeholder: {e}"))
                })?;
            }
        }

        // Final separator
        if !self.0.is_empty() {
            writeln!(output, "---").map_err(|e| {
                CliError::General(format!("Failed to write markdown separator: {e}"))
            })?;
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_list_formatter(
            &self.0,
            &["Key", "Name", "Private", "Description"],
            |team| {
                vec![
                    team.key.clone(),
                    team.name.clone(),
                    if team.private { "Yes" } else { "No" }.to_string(),
                    team.description.clone().unwrap_or_else(|| "—".to_string()),
                ]
            },
        )
    }
}

// From trait implementation for converting Cynic types to domain types

impl From<queries::TeamNode> for Team {
    fn from(node: queries::TeamNode) -> Self {
        Team {
            id: node.id.inner().to_string(),
            key: node.key,
            name: node.name,
            description: node.description,
            color: node.color,
            icon: node.icon,
            private: node.private,
            created_at: node.created_at.0,
        }
    }
}
