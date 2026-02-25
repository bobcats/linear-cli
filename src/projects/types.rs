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

/// Project details returned from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: Option<String>,
    pub slug_id: String,
    pub url: String,
    pub color: String,
    pub icon: Option<String>,

    // Status (from nested ProjectStatus object)
    pub status_name: String,
    pub status_type: String,
    pub status_color: String,

    // Metrics
    pub progress: f64,          // 0.0 to 1.0
    pub priority: i32,          // 0-4
    pub priority_label: String, // "None", "Low", "Medium", "High", "Urgent"

    // Dates
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,

    // Relationships
    pub lead_name: Option<String>, // From lead.name
}

// Trait implementations for generic formatters

impl TableFormatter for Project {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (
                Cow::Borrowed("Description"),
                Cow::Borrowed(self.description.as_str()),
            ),
            (
                Cow::Borrowed("Status"),
                Cow::Borrowed(self.status_name.as_str()),
            ),
        ];

        // Progress as percentage
        rows.push((
            Cow::Borrowed("Progress"),
            Cow::Owned(format_progress_percent(self.progress)),
        ));

        // Priority with label
        rows.push((
            Cow::Borrowed("Priority"),
            Cow::Owned(format!("{} ({})", self.priority_label, self.priority)),
        ));

        // Optional dates
        if let Some(start) = &self.start_date {
            rows.push((Cow::Borrowed("Start Date"), Cow::Borrowed(start.as_str())));
        }
        if let Some(target) = &self.target_date {
            rows.push((Cow::Borrowed("Target Date"), Cow::Borrowed(target.as_str())));
        }

        // Optional lead
        if let Some(lead) = &self.lead_name {
            rows.push((Cow::Borrowed("Lead"), Cow::Borrowed(lead.as_str())));
        }

        // Metadata fields
        rows.push((Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())));
        rows.push((Cow::Borrowed("Slug"), Cow::Borrowed(self.slug_id.as_str())));
        rows.push((Cow::Borrowed("Color"), Cow::Borrowed(self.color.as_str())));

        if let Some(icon) = &self.icon {
            rows.push((Cow::Borrowed("Icon"), Cow::Borrowed(icon.as_str())));
        }

        rows.push((Cow::Borrowed("URL"), Cow::Borrowed(self.url.as_str())));
        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));
        rows.push((
            Cow::Borrowed("Updated"),
            Cow::Borrowed(self.updated_at.as_str()),
        ));

        rows
    }
}

impl MarkdownFormatter for Project {
    fn markdown_capacity_hint(&self) -> usize {
        300 + self.name.len()
            + self.id.len()
            + self.description.len()
            + self.content.as_ref().map_or(0, |c| c.len())
            + self.slug_id.len()
            + self.url.len()
            + self.color.len()
            + self.icon.as_ref().map_or(0, |i| i.len())
            + self.status_name.len()
            + self.priority_label.len()
            + self.start_date.as_ref().map_or(0, |d| d.len())
            + self.target_date.as_ref().map_or(0, |d| d.len())
            + self.lead_name.as_ref().map_or(0, |l| l.len())
            + self.created_at.len()
            + self.updated_at.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        // Exact same logic as original custom implementation
        // H1 title with project name
        writeln!(output, "# {}\n", self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        // Description
        writeln!(output, "{}\n", self.description)
            .map_err(|e| CliError::General(format!("Failed to write markdown description: {e}")))?;

        if let Some(content) = &self.content
            && !content.is_empty()
        {
            writeln!(output, "## Content\n\n{}\n", content)
                .map_err(|e| CliError::General(format!("Failed to write markdown content: {e}")))?;
        }

        // Status
        writeln!(output, "**Status:** {}", self.status_name)
            .map_err(|e| CliError::General(format!("Failed to write markdown status: {e}")))?;

        // Progress as percentage
        writeln!(output, "**Progress:** {:.0}%", self.progress * 100.0)
            .map_err(|e| CliError::General(format!("Failed to write markdown progress: {e}")))?;

        // Priority
        writeln!(
            output,
            "**Priority:** {} ({})",
            self.priority_label, self.priority
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown priority: {e}")))?;

        // Dates
        if let Some(start) = &self.start_date {
            writeln!(output, "**Start Date:** {}", start).map_err(|e| {
                CliError::General(format!("Failed to write markdown start date: {e}"))
            })?;
        }
        if let Some(target) = &self.target_date {
            writeln!(output, "**Target Date:** {}", target).map_err(|e| {
                CliError::General(format!("Failed to write markdown target date: {e}"))
            })?;
        }

        // Lead
        if let Some(lead) = &self.lead_name {
            writeln!(output, "**Lead:** {}", lead)
                .map_err(|e| CliError::General(format!("Failed to write markdown lead: {e}")))?;
        }

        // Metadata
        writeln!(output, "\n**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown ID: {e}")))?;
        writeln!(output, "**Slug:** {}", self.slug_id)
            .map_err(|e| CliError::General(format!("Failed to write markdown slug: {e}")))?;
        writeln!(output, "**Color:** {}", self.color)
            .map_err(|e| CliError::General(format!("Failed to write markdown color: {e}")))?;

        if let Some(icon) = &self.icon {
            writeln!(output, "**Icon:** {}", icon)
                .map_err(|e| CliError::General(format!("Failed to write markdown icon: {e}")))?;
        }

        writeln!(output, "**URL:** {}", self.url)
            .map_err(|e| CliError::General(format!("Failed to write markdown URL: {e}")))?;
        writeln!(output, "**Created:** {}", self.created_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown created: {e}")))?;
        writeln!(output, "**Updated:** {}", self.updated_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown updated: {e}")))?;

        Ok(())
    }
}

impl Formattable for Project {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "name",
            "slug_id",
            "id",
            "description",
            "status",
            "progress",
            "priority",
            "priority_label",
            "start_date",
            "target_date",
            "lead_name",
            "color",
            "icon",
            "url",
            "created_at",
            "updated_at",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write data row
        let progress_str = format_progress_percent(self.progress);
        wtr.write_record([
            &self.name,
            &self.slug_id,
            &self.id,
            &self.description,
            &self.status_name,
            &progress_str,
            &self.priority.to_string(),
            &self.priority_label,
            self.start_date.as_deref().unwrap_or(""),
            self.target_date.as_deref().unwrap_or(""),
            self.lead_name.as_deref().unwrap_or(""),
            &self.color,
            self.icon.as_deref().unwrap_or(""),
            &self.url,
            &self.created_at,
            &self.updated_at,
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

/// Wrapper type for a list of projects
/// Provides different formatting for collections vs single items
pub struct ProjectList(pub Vec<Project>);

impl Formattable for ProjectList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "id",
            "name",
            "slug_id",
            "status",
            "progress",
            "priority",
            "start_date",
            "target_date",
            "lead_name",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write each project as a row
        for project in &self.0 {
            let progress_str = format_progress_percent(project.progress);
            wtr.write_record([
                &project.id,
                &project.name,
                &project.slug_id,
                &project.status_name,
                &progress_str,
                &project.priority_label,
                project.start_date.as_deref().unwrap_or(""),
                project.target_date.as_deref().unwrap_or(""),
                project.lead_name.as_deref().unwrap_or(""),
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        let capacity: usize = self
            .0
            .iter()
            .map(|p| {
                200 + p.name.len()
                    + p.slug_id.len()
                    + p.description.len()
                    + p.content.as_ref().map_or(0, |c| c.len())
                    + p.status_name.len()
                    + p.priority_label.len()
                    + p.start_date.as_ref().map_or(0, |d| d.len())
                    + p.target_date.as_ref().map_or(0, |d| d.len())
                    + p.lead_name.as_ref().map_or(0, |l| l.len())
            })
            .sum();
        let mut output = String::with_capacity(capacity);

        // Header with count
        writeln!(output, "## Projects ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;

        // Output each project as a summary card
        for project in &self.0 {
            write!(output, "---\n\n### {}\n", project.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

            // Status and progress on one line
            writeln!(
                output,
                "**Status:** {} | **Progress:** {:.0}% | **Priority:** {}",
                project.status_name,
                project.progress * 100.0,
                project.priority_label
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown metadata: {e}")))?;

            // Dates if available - write directly to avoid vec allocation
            if project.start_date.is_some() || project.target_date.is_some() {
                write!(output, "**Dates:** ").map_err(|e| {
                    CliError::General(format!("Failed to write markdown dates label: {e}"))
                })?;

                if let Some(start) = &project.start_date {
                    write!(output, "Start: {start}").map_err(|e| {
                        CliError::General(format!("Failed to write markdown start date: {e}"))
                    })?;

                    if project.target_date.is_some() {
                        write!(output, " | ").map_err(|e| {
                            CliError::General(format!(
                                "Failed to write markdown date separator: {e}"
                            ))
                        })?;
                    }
                }

                if let Some(target) = &project.target_date {
                    write!(output, "Target: {target}").map_err(|e| {
                        CliError::General(format!("Failed to write markdown target date: {e}"))
                    })?;
                }

                writeln!(output).map_err(|e| {
                    CliError::General(format!("Failed to write markdown newline: {e}"))
                })?;
            }

            // Lead if available
            if let Some(lead) = &project.lead_name {
                writeln!(output, "**Lead:** {}", lead).map_err(|e| {
                    CliError::General(format!("Failed to write markdown lead: {e}"))
                })?;
            }

            writeln!(output, "\n{}\n", project.description).map_err(|e| {
                CliError::General(format!("Failed to write markdown description: {e}"))
            })?;

            if let Some(content) = &project.content
                && !content.is_empty()
            {
                writeln!(output, "{}\n", content).map_err(|e| {
                    CliError::General(format!("Failed to write markdown content: {e}"))
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
            &["ID", "Name", "Status", "Progress", "Priority", "Lead"],
            |project| {
                vec![
                    project.id.clone(),
                    project.name.clone(),
                    project.status_name.clone(),
                    format_progress_percent(project.progress),
                    project.priority_label.clone(),
                    project.lead_name.clone().unwrap_or_else(|| "—".to_string()),
                ]
            },
        )
    }
}

// From trait implementation for converting Cynic types to domain types

impl From<crate::client::queries::ProjectNode> for Project {
    fn from(node: crate::client::queries::ProjectNode) -> Self {
        // Convert ProjectStatusType enum to String directly in match
        let status_type = match node.status.status_type {
            crate::client::queries::ProjectStatusType::Backlog => "backlog".to_owned(),
            crate::client::queries::ProjectStatusType::Planned => "planned".to_owned(),
            crate::client::queries::ProjectStatusType::Started => "started".to_owned(),
            crate::client::queries::ProjectStatusType::Paused => "paused".to_owned(),
            crate::client::queries::ProjectStatusType::Completed => "completed".to_owned(),
            crate::client::queries::ProjectStatusType::Canceled => "canceled".to_owned(),
        };

        Project {
            id: node.id.inner().to_string(),
            name: node.name,
            description: node.description,
            content: node.content,
            slug_id: node.slug_id,
            url: node.url,
            color: node.color,
            icon: node.icon,
            status_name: node.status.name,
            status_type,
            status_color: node.status.color,
            progress: node.progress,
            priority: node.priority,
            priority_label: node.priority_label,
            start_date: node.start_date.map(|d| d.0),
            target_date: node.target_date.map(|d| d.0),
            lead_name: node.lead.map(|l| l.name),
            created_at: node.created_at.0,
            updated_at: node.updated_at.0,
        }
    }
}
