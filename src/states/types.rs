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

/// Workflow state from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    pub state_type: String,
    pub color: String,
    pub position: f64,
    pub description: Option<String>,
    pub team_name: Option<String>,
}

impl TableFormatter for WorkflowState {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (
                Cow::Borrowed("Type"),
                Cow::Borrowed(self.state_type.as_str()),
            ),
            (Cow::Borrowed("Color"), Cow::Borrowed(self.color.as_str())),
            (
                Cow::Borrowed("Position"),
                Cow::Owned(self.position.to_string()),
            ),
            (Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())),
        ];

        if let Some(desc) = &self.description {
            rows.push((Cow::Borrowed("Description"), Cow::Borrowed(desc.as_str())));
        }
        if let Some(team) = &self.team_name {
            rows.push((Cow::Borrowed("Team"), Cow::Borrowed(team.as_str())));
        }

        rows
    }
}

impl MarkdownFormatter for WorkflowState {
    fn markdown_capacity_hint(&self) -> usize {
        150 + self.name.len()
            + self.state_type.len()
            + self.color.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.team_name.as_ref().map_or(0, |t| t.len())
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        writeln!(output, "# {}\n", self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        writeln!(output, "**Type:** {}", self.state_type)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**Color:** {}", self.color)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**Position:** {}", self.position)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        if let Some(desc) = &self.description {
            writeln!(output, "**Description:** {}", desc)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }
        if let Some(team) = &self.team_name {
            writeln!(output, "**Team:** {}", team)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(())
    }
}

impl Formattable for WorkflowState {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record([
            "name",
            "type",
            "color",
            "position",
            "id",
            "description",
            "team",
        ])
        .csv_err("Failed to write CSV header")?;

        wtr.write_record([
            &self.name,
            &self.state_type,
            &self.color,
            &self.position.to_string(),
            &self.id,
            self.description.as_deref().unwrap_or(""),
            self.team_name.as_deref().unwrap_or(""),
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

/// Wrapper for a list of workflow states
pub struct WorkflowStateList(pub Vec<WorkflowState>);

impl Formattable for WorkflowStateList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["name", "type", "color", "position", "description", "team"])
            .csv_err("Failed to write CSV header")?;

        for state in &self.0 {
            wtr.write_record([
                &state.name,
                &state.state_type,
                &state.color,
                &state.position.to_string(),
                state.description.as_deref().unwrap_or(""),
                state.team_name.as_deref().unwrap_or(""),
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;
        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        let capacity = self
            .0
            .iter()
            .map(|s| 150 + s.name.len() + s.state_type.len() + s.color.len())
            .sum();
        let mut output = String::with_capacity(capacity);

        writeln!(output, "## Workflow States ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        for state in &self.0 {
            writeln!(output, "---\n")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(output, "### {}\n", state.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(
                output,
                "**Type:** {} | **Color:** {} | **Position:** {}\n",
                state.state_type, state.color, state.position
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

            if let Some(desc) = &state.description {
                writeln!(output, "{}\n", desc)
                    .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            }
        }

        if !self.0.is_empty() {
            writeln!(output, "---")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_list_formatter(
            &self.0,
            &["Name", "Type", "Color", "Description"],
            |state| {
                vec![
                    state.name.clone(),
                    state.state_type.clone(),
                    state.color.clone(),
                    state.description.clone().unwrap_or_else(|| "—".to_string()),
                ]
            },
        )
    }
}

impl From<queries::WorkflowStateNode> for WorkflowState {
    fn from(node: queries::WorkflowStateNode) -> Self {
        WorkflowState {
            id: node.id.inner().to_string(),
            name: node.name,
            state_type: node.state_type,
            color: node.color,
            position: node.position,
            description: node.description,
            team_name: Some(node.team.name),
        }
    }
}
