use crate::error::CliError;
use crate::output::{
    CsvResultExt, Formattable, MarkdownFormatter, TableFormatter, fast_markdown_formatter,
    generic_json_formatter, generic_json_list_formatter, generic_table_formatter,
    generic_table_list_formatter,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneProject {
    pub id: String,
    pub name: String,
    pub slug_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: String,
    pub progress: f64,
    pub sort_order: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    pub project: MilestoneProject,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

pub struct MilestoneList(pub Vec<Milestone>);

fn format_progress_percent(progress: f64) -> String {
    format!("{:.0}%", progress * 100.0)
}

impl TableFormatter for Milestone {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (
                Cow::Borrowed("Project"),
                Cow::Borrowed(self.project.name.as_str()),
            ),
            (Cow::Borrowed("Status"), Cow::Borrowed(self.status.as_str())),
            (
                Cow::Borrowed("Progress"),
                Cow::Owned(format_progress_percent(self.progress)),
            ),
        ];

        if let Some(description) = &self.description {
            rows.push((
                Cow::Borrowed("Description"),
                Cow::Borrowed(description.as_str()),
            ));
        }
        if let Some(target_date) = &self.target_date {
            rows.push((
                Cow::Borrowed("Target Date"),
                Cow::Borrowed(target_date.as_str()),
            ));
        }

        rows.push((Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())));
        rows.push((
            Cow::Borrowed("Project ID"),
            Cow::Borrowed(self.project.id.as_str()),
        ));
        rows.push((
            Cow::Borrowed("Project Slug"),
            Cow::Borrowed(self.project.slug_id.as_str()),
        ));
        rows.push((
            Cow::Borrowed("Sort Order"),
            Cow::Owned(self.sort_order.to_string()),
        ));
        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));
        rows.push((
            Cow::Borrowed("Updated"),
            Cow::Borrowed(self.updated_at.as_str()),
        ));
        if let Some(archived_at) = &self.archived_at {
            rows.push((
                Cow::Borrowed("Archived"),
                Cow::Borrowed(archived_at.as_str()),
            ));
        }

        rows
    }
}

impl MarkdownFormatter for Milestone {
    fn markdown_capacity_hint(&self) -> usize {
        200 + self.name.len()
            + self.id.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.status.len()
            + self.target_date.as_ref().map_or(0, |d| d.len())
            + self.project.name.len()
            + self.project.slug_id.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        writeln!(output, "# {}\n", self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        if let Some(description) = &self.description {
            writeln!(output, "{}\n", description).map_err(|e| {
                CliError::General(format!("Failed to write markdown description: {e}"))
            })?;
        }

        writeln!(output, "**Project:** {}", self.project.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown project: {e}")))?;
        writeln!(output, "**Status:** {}", self.status)
            .map_err(|e| CliError::General(format!("Failed to write markdown status: {e}")))?;
        writeln!(output, "**Progress:** {:.0}%", self.progress * 100.0)
            .map_err(|e| CliError::General(format!("Failed to write markdown progress: {e}")))?;

        if let Some(target_date) = &self.target_date {
            writeln!(output, "**Target Date:** {}", target_date).map_err(|e| {
                CliError::General(format!("Failed to write markdown target date: {e}"))
            })?;
        }

        writeln!(output, "\n**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown ID: {e}")))?;
        writeln!(output, "**Project Slug:** {}", self.project.slug_id)
            .map_err(|e| CliError::General(format!("Failed to write markdown slug: {e}")))?;
        writeln!(output, "**Created:** {}", self.created_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown created: {e}")))?;
        writeln!(output, "**Updated:** {}", self.updated_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown updated: {e}")))?;

        Ok(())
    }
}

impl Formattable for Milestone {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record([
            "id",
            "name",
            "project",
            "project_id",
            "project_slug",
            "status",
            "progress",
            "target_date",
            "description",
            "sort_order",
            "created_at",
            "updated_at",
            "archived_at",
        ])
        .csv_err("Failed to write CSV header")?;

        let progress = format_progress_percent(self.progress);
        let sort_order = self.sort_order.to_string();
        wtr.write_record([
            self.id.as_str(),
            self.name.as_str(),
            self.project.name.as_str(),
            self.project.id.as_str(),
            self.project.slug_id.as_str(),
            self.status.as_str(),
            progress.as_str(),
            self.target_date.as_deref().unwrap_or(""),
            self.description.as_deref().unwrap_or(""),
            sort_order.as_str(),
            self.created_at.as_str(),
            self.updated_at.as_str(),
            self.archived_at.as_deref().unwrap_or(""),
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

impl From<crate::client::queries::ProjectMilestoneNode> for Milestone {
    fn from(node: crate::client::queries::ProjectMilestoneNode) -> Self {
        let status = match node.status {
            crate::client::queries::ProjectMilestoneStatus::Done => "done",
            crate::client::queries::ProjectMilestoneStatus::Next => "next",
            crate::client::queries::ProjectMilestoneStatus::Overdue => "overdue",
            crate::client::queries::ProjectMilestoneStatus::Unstarted => "unstarted",
        }
        .to_string();

        Milestone {
            id: node.id.inner().to_string(),
            name: node.name,
            description: node.description,
            status,
            progress: node.progress,
            sort_order: node.sort_order,
            target_date: node.target_date.map(|date| date.0),
            project: MilestoneProject {
                id: node.project.id.inner().to_string(),
                name: node.project.name,
                slug_id: node.project.slug_id,
            },
            created_at: node.created_at.0,
            updated_at: node.updated_at.0,
            archived_at: node.archived_at.map(|date| date.0),
        }
    }
}

impl Formattable for MilestoneList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record(["id", "name", "project", "status", "progress", "target_date"])
            .csv_err("Failed to write CSV header")?;

        for milestone in &self.0 {
            let progress = format_progress_percent(milestone.progress);
            wtr.write_record([
                milestone.id.as_str(),
                milestone.name.as_str(),
                milestone.project.name.as_str(),
                milestone.status.as_str(),
                progress.as_str(),
                milestone.target_date.as_deref().unwrap_or(""),
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;
        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        let mut output = String::new();
        writeln!(output, "## Milestones ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;
        for milestone in &self.0 {
            writeln!(
                output,
                "### {}\n\n**Project:** {} | **Status:** {} | **Progress:** {}\n",
                milestone.name,
                milestone.project.name,
                milestone.status,
                format_progress_percent(milestone.progress)
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown milestone: {e}")))?;
        }
        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_list_formatter(
            &self.0,
            &["ID", "Name", "Project", "Status", "Progress", "Target Date"],
            |milestone| {
                vec![
                    milestone.id.clone(),
                    milestone.name.clone(),
                    milestone.project.name.clone(),
                    milestone.status.clone(),
                    format_progress_percent(milestone.progress),
                    milestone
                        .target_date
                        .clone()
                        .unwrap_or_else(|| "—".to_string()),
                ]
            },
        )
    }
}
