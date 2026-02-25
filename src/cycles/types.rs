use crate::error::CliError;
use crate::output::{
    CsvResultExt, Formattable, MarkdownFormatter, TableFormatter, fast_markdown_formatter,
    generic_json_formatter, generic_json_list_formatter, generic_table_formatter,
    generic_table_list_formatter,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;

/// Format progress as percentage without allocation
/// Uses a stack-allocated buffer to avoid heap allocation
#[inline]
fn format_progress_percent(progress: f64) -> String {
    // Pre-allocate with exact capacity needed: "100%" = 4 chars max
    let mut buf = String::with_capacity(4);
    write!(buf, "{:.0}%", progress * 100.0).unwrap();
    buf
}

/// Format cycle status based on boolean flags
#[inline]
fn format_cycle_status(
    is_active: bool,
    is_future: bool,
    is_past: bool,
    is_next: bool,
    is_previous: bool,
) -> &'static str {
    if is_active {
        "Active"
    } else if is_next {
        "Next"
    } else if is_previous {
        "Previous"
    } else if is_future {
        "Future"
    } else if is_past {
        "Past"
    } else {
        "Unknown"
    }
}

/// Cycle details returned from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cycle {
    pub id: String,
    pub name: String,
    pub number: f64,
    pub description: Option<String>,

    // Dates
    pub starts_at: String,
    pub ends_at: String,
    pub created_at: String,
    pub completed_at: Option<String>,

    // Progress
    pub progress: f64, // 0.0 to 1.0

    // Status flags
    pub is_active: bool,
    pub is_future: bool,
    pub is_next: bool,
    pub is_past: bool,
    pub is_previous: bool,

    // Relationships
    pub team_name: String,
    pub team_key: String,
}

// Trait implementations for generic formatters

impl TableFormatter for Cycle {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (
                Cow::Borrowed("Number"),
                Cow::Owned(format!("#{}", self.number)),
            ),
        ];

        let status = format_cycle_status(
            self.is_active,
            self.is_future,
            self.is_past,
            self.is_next,
            self.is_previous,
        );
        rows.push((Cow::Borrowed("Status"), Cow::Borrowed(status)));

        rows.push((
            Cow::Borrowed("Progress"),
            Cow::Owned(format_progress_percent(self.progress)),
        ));

        rows.push((
            Cow::Borrowed("Team"),
            Cow::Owned(format!("{} ({})", self.team_name, self.team_key)),
        ));

        if let Some(desc) = &self.description {
            rows.push((Cow::Borrowed("Description"), Cow::Borrowed(desc.as_str())));
        }

        rows.push((
            Cow::Borrowed("Start Date"),
            Cow::Borrowed(self.starts_at.as_str()),
        ));
        rows.push((
            Cow::Borrowed("End Date"),
            Cow::Borrowed(self.ends_at.as_str()),
        ));

        if let Some(completed) = &self.completed_at {
            rows.push((
                Cow::Borrowed("Completed"),
                Cow::Borrowed(completed.as_str()),
            ));
        }

        rows.push((Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())));
        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));

        rows
    }
}

impl MarkdownFormatter for Cycle {
    fn markdown_capacity_hint(&self) -> usize {
        400 + self.name.len()
            + self.id.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.team_name.len()
            + self.team_key.len()
            + self.starts_at.len()
            + self.ends_at.len()
            + self.completed_at.as_ref().map_or(0, |c| c.len())
            + self.created_at.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        // Exact same logic as original custom implementation
        // H1 title with cycle name and number
        writeln!(output, "# {} (Cycle #{})\n", self.name, self.number)
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        // Description
        if let Some(desc) = &self.description {
            writeln!(output, "{}\n", desc).map_err(|e| {
                CliError::General(format!("Failed to write markdown description: {e}"))
            })?;
        }

        // Status and progress
        let status = format_cycle_status(
            self.is_active,
            self.is_future,
            self.is_past,
            self.is_next,
            self.is_previous,
        );
        writeln!(
            output,
            "**Status:** {} | **Progress:** {:.0}%",
            status,
            self.progress * 100.0
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown status: {e}")))?;

        // Team
        writeln!(output, "**Team:** {} ({})", self.team_name, self.team_key)
            .map_err(|e| CliError::General(format!("Failed to write markdown team: {e}")))?;

        // Dates
        writeln!(output, "\n**Start Date:** {}", self.starts_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown start date: {e}")))?;
        writeln!(output, "**End Date:** {}", self.ends_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown end date: {e}")))?;

        if let Some(completed) = &self.completed_at {
            writeln!(output, "**Completed:** {}", completed).map_err(|e| {
                CliError::General(format!("Failed to write markdown completed: {e}"))
            })?;
        }

        // Metadata
        writeln!(output, "\n**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown ID: {e}")))?;
        writeln!(output, "**Created:** {}", self.created_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown created: {e}")))?;

        Ok(())
    }
}

impl Formattable for Cycle {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "name",
            "number",
            "team",
            "status",
            "progress",
            "starts_at",
            "ends_at",
            "completed_at",
            "id",
            "created_at",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write data row
        let progress_str = format_progress_percent(self.progress);
        let status = format_cycle_status(
            self.is_active,
            self.is_future,
            self.is_past,
            self.is_next,
            self.is_previous,
        );

        wtr.write_record([
            &self.name,
            &self.number.to_string(),
            &format!("{} ({})", self.team_name, self.team_key),
            status,
            &progress_str,
            &self.starts_at,
            &self.ends_at,
            self.completed_at.as_deref().unwrap_or(""),
            &self.id,
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

/// Wrapper type for a list of cycles
/// Provides different formatting for collections vs single items
pub struct CycleList(pub Vec<Cycle>);

impl Formattable for CycleList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "name",
            "number",
            "team",
            "status",
            "progress",
            "starts_at",
            "ends_at",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write each cycle as a row
        for cycle in &self.0 {
            let progress_str = format_progress_percent(cycle.progress);
            let status = format_cycle_status(
                cycle.is_active,
                cycle.is_future,
                cycle.is_past,
                cycle.is_next,
                cycle.is_previous,
            );

            wtr.write_record([
                &cycle.name,
                &cycle.number.to_string(),
                &format!("{} ({})", cycle.team_name, cycle.team_key),
                status,
                &progress_str,
                &cycle.starts_at,
                &cycle.ends_at,
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        // Pre-allocate: ~200 bytes per cycle for structure + actual content
        let capacity: usize = self
            .0
            .iter()
            .map(|c| {
                200 + c.name.len()
                    + c.team_name.len()
                    + c.team_key.len()
                    + c.description.as_ref().map_or(0, |d| d.len())
            })
            .sum();
        let mut output = String::with_capacity(capacity);

        // Header with count
        writeln!(output, "## Cycles ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;

        // Output each cycle as a summary card
        for cycle in &self.0 {
            write!(
                output,
                "---\n\n### {} (Cycle #{})\n",
                cycle.name, cycle.number
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

            // Status, progress, team on one line
            let status = format_cycle_status(
                cycle.is_active,
                cycle.is_future,
                cycle.is_past,
                cycle.is_next,
                cycle.is_previous,
            );
            writeln!(
                output,
                "**Status:** {} | **Progress:** {:.0}% | **Team:** {} ({})",
                status,
                cycle.progress * 100.0,
                cycle.team_name,
                cycle.team_key
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown metadata: {e}")))?;

            // Dates
            writeln!(output, "**Dates:** {} → {}", cycle.starts_at, cycle.ends_at)
                .map_err(|e| CliError::General(format!("Failed to write markdown dates: {e}")))?;

            // Description if available
            if let Some(desc) = &cycle.description {
                writeln!(output, "\n{}\n", desc).map_err(|e| {
                    CliError::General(format!("Failed to write markdown description: {e}"))
                })?;
            } else {
                writeln!(output).map_err(|e| {
                    CliError::General(format!("Failed to write markdown newline: {e}"))
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
            &["Name", "Number", "Status", "Progress", "Team", "Dates"],
            |cycle| {
                let status = format_cycle_status(
                    cycle.is_active,
                    cycle.is_future,
                    cycle.is_past,
                    cycle.is_next,
                    cycle.is_previous,
                );

                vec![
                    cycle.name.clone(),
                    format!("#{}", cycle.number),
                    status.to_string(),
                    format_progress_percent(cycle.progress),
                    format!("{} ({})", cycle.team_name, cycle.team_key),
                    format!("{} → {}", cycle.starts_at, cycle.ends_at),
                ]
            },
        )
    }
}

// From trait implementation for converting Cynic types to domain types

impl From<crate::client::queries::CycleNode> for Cycle {
    fn from(node: crate::client::queries::CycleNode) -> Self {
        Cycle {
            id: node.id.inner().to_string(),
            name: node
                .name
                .unwrap_or_else(|| format!("Cycle #{}", node.number)),
            number: node.number,
            description: node.description,
            starts_at: node.starts_at.0,
            ends_at: node.ends_at.0,
            created_at: node.created_at.0,
            completed_at: node.completed_at.map(|d| d.0),
            progress: node.progress,
            is_active: node.is_active,
            is_future: node.is_future,
            is_next: node.is_next,
            is_past: node.is_past,
            is_previous: node.is_previous,
            team_name: node.team.name,
            team_key: node.team.key,
        }
    }
}
