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

/// Issue label from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_group: bool,
    pub parent_name: Option<String>,
}

impl TableFormatter for IssueLabel {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (Cow::Borrowed("Color"), Cow::Borrowed(self.color.as_str())),
            (
                Cow::Borrowed("Group"),
                Cow::Borrowed(if self.is_group { "Yes" } else { "No" }),
            ),
            (Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())),
        ];

        if let Some(desc) = &self.description {
            rows.push((Cow::Borrowed("Description"), Cow::Borrowed(desc.as_str())));
        }
        if let Some(parent) = &self.parent_name {
            rows.push((Cow::Borrowed("Parent"), Cow::Borrowed(parent.as_str())));
        }

        rows
    }
}

impl MarkdownFormatter for IssueLabel {
    fn markdown_capacity_hint(&self) -> usize {
        100 + self.name.len()
            + self.color.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.parent_name.as_ref().map_or(0, |p| p.len())
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        writeln!(output, "# {}\n", self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        writeln!(output, "**Color:** {}", self.color)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(
            output,
            "**Group:** {}",
            if self.is_group { "Yes" } else { "No" }
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        if let Some(desc) = &self.description {
            writeln!(output, "**Description:** {}", desc)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }
        if let Some(parent) = &self.parent_name {
            writeln!(output, "**Parent:** {}", parent)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(())
    }
}

impl Formattable for IssueLabel {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["name", "color", "id", "description", "is_group", "parent"])
            .csv_err("Failed to write CSV header")?;

        let is_group = if self.is_group { "true" } else { "false" };
        wtr.write_record([
            self.name.as_str(),
            self.color.as_str(),
            self.id.as_str(),
            self.description.as_deref().unwrap_or(""),
            is_group,
            self.parent_name.as_deref().unwrap_or(""),
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

/// Wrapper for a list of labels
pub struct IssueLabelList(pub Vec<IssueLabel>);

impl Formattable for IssueLabelList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["name", "color", "description", "is_group", "parent"])
            .csv_err("Failed to write CSV header")?;

        for label in &self.0 {
            let is_group = if label.is_group { "true" } else { "false" };
            wtr.write_record([
                label.name.as_str(),
                label.color.as_str(),
                label.description.as_deref().unwrap_or(""),
                is_group,
                label.parent_name.as_deref().unwrap_or(""),
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
            .map(|l| 100 + l.name.len() + l.color.len())
            .sum();
        let mut output = String::with_capacity(capacity);

        writeln!(output, "## Labels ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        for label in &self.0 {
            writeln!(output, "---\n")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(output, "### {}\n", label.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(
                output,
                "**Color:** {} | **Group:** {}\n",
                label.color,
                if label.is_group { "Yes" } else { "No" }
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

            if let Some(desc) = &label.description {
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
            &["Name", "Color", "Group", "Description"],
            |label| {
                vec![
                    label.name.clone(),
                    label.color.clone(),
                    if label.is_group { "Yes" } else { "No" }.to_string(),
                    label.description.clone().unwrap_or_else(|| "—".to_string()),
                ]
            },
        )
    }
}

impl From<queries::IssueLabelNode> for IssueLabel {
    fn from(node: queries::IssueLabelNode) -> Self {
        IssueLabel {
            id: node.id.inner().to_string(),
            name: node.name,
            color: node.color,
            description: node.description,
            is_group: node.is_group,
            parent_name: node.parent.map(|p| p.name),
        }
    }
}
