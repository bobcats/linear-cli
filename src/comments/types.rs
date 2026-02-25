use crate::error::CliError;
use crate::output::{
    CsvResultExt, Formattable, MarkdownFormatter, TableFormatter, fast_markdown_formatter,
    generic_json_formatter, generic_json_list_formatter, generic_table_formatter,
};
use comfy_table::{Table, presets::UTF8_FULL};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;

/// Comment details returned from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub body: String,

    // Author
    pub user_name: String,
    pub user_email: String,

    // Timestamps
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,

    // Context
    pub issue_identifier: Option<String>,
}

// Trait implementations for generic formatters

impl TableFormatter for Comment {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![];

        // Add issue context if present
        if let Some(issue_id) = &self.issue_identifier {
            rows.push((Cow::Borrowed("Issue"), Cow::Borrowed(issue_id.as_str())));
        }

        rows.push((
            Cow::Borrowed("Author"),
            Cow::Owned(format!("{} ({})", self.user_name, self.user_email)),
        ));
        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));

        if let Some(edited) = &self.edited_at {
            rows.push((Cow::Borrowed("Edited"), Cow::Borrowed(edited.as_str())));
        }

        rows.push((Cow::Borrowed("Body"), Cow::Borrowed(self.body.as_str())));
        rows.push((Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())));
        rows.push((
            Cow::Borrowed("Updated"),
            Cow::Borrowed(self.updated_at.as_str()),
        ));

        rows
    }
}

impl MarkdownFormatter for Comment {
    fn markdown_capacity_hint(&self) -> usize {
        300 + self.body.len()
            + self.user_name.len()
            + self.user_email.len()
            + self.created_at.len()
            + self.updated_at.len()
            + self.edited_at.as_ref().map_or(0, |e| e.len())
            + self.issue_identifier.as_ref().map_or(0, |i| i.len())
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        // Exact same logic as original custom implementation
        // H1 title with author
        if let Some(issue_id) = &self.issue_identifier {
            writeln!(output, "# Comment on {}\n", issue_id)
        } else {
            writeln!(output, "# Comment\n")
        }
        .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        // Author info
        writeln!(
            output,
            "**Author:** {} ({})",
            self.user_name, self.user_email
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown author: {e}")))?;

        // Timestamps
        writeln!(output, "**Created:** {}", self.created_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown created: {e}")))?;

        if let Some(edited) = &self.edited_at {
            writeln!(output, "**Edited:** {}", edited)
                .map_err(|e| CliError::General(format!("Failed to write markdown edited: {e}")))?;
        }

        // Body content
        writeln!(output, "\n---\n\n{}\n", self.body)
            .map_err(|e| CliError::General(format!("Failed to write markdown body: {e}")))?;

        // Metadata
        writeln!(output, "---\n")
            .map_err(|e| CliError::General(format!("Failed to write markdown separator: {e}")))?;
        writeln!(output, "**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown ID: {e}")))?;
        writeln!(output, "**Updated:** {}", self.updated_at)
            .map_err(|e| CliError::General(format!("Failed to write markdown updated: {e}")))?;

        Ok(())
    }
}

impl Formattable for Comment {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "id",
            "user_name",
            "user_email",
            "body_preview",
            "created_at",
            "updated_at",
            "edited_at",
            "issue",
        ])
        .csv_err("Failed to write CSV header")?;

        // Truncate body for preview (first 100 chars)
        let body_preview = if self.body.len() > 100 {
            format!("{}...", &self.body[..97])
        } else {
            self.body.clone()
        };

        // Write data row
        wtr.write_record([
            &self.id,
            &self.user_name,
            &self.user_email,
            &body_preview,
            &self.created_at,
            &self.updated_at,
            self.edited_at.as_deref().unwrap_or(""),
            self.issue_identifier.as_deref().unwrap_or(""),
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

/// Wrapper type for a list of comments
/// Provides different formatting for collections vs single items
pub struct CommentList(pub Vec<Comment>);

impl Formattable for CommentList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record(["id", "user_name", "body_preview", "created_at", "edited_at"])
            .csv_err("Failed to write CSV header")?;

        // Write each comment as a row
        for comment in &self.0 {
            // Truncate body for preview (first 100 chars)
            let body_preview = if comment.body.len() > 100 {
                format!("{}...", &comment.body[..97])
            } else {
                comment.body.clone()
            };

            wtr.write_record([
                &comment.id,
                &comment.user_name,
                &body_preview,
                &comment.created_at,
                comment.edited_at.as_deref().unwrap_or(""),
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        // Pre-allocate: ~250 bytes per comment for structure + actual content
        let capacity: usize = self
            .0
            .iter()
            .map(|c| 250 + c.body.len() + c.user_name.len() + c.user_email.len())
            .sum();
        let mut output = String::with_capacity(capacity);

        // Header with count
        writeln!(output, "## Comments ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;

        // Output each comment as a summary card
        for comment in &self.0 {
            write!(output, "---\n\n### {}\n", comment.user_name)
                .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

            // Metadata on one line
            let edited_str = if let Some(edited) = &comment.edited_at {
                format!(" | **Edited:** {}", edited)
            } else {
                String::new()
            };

            writeln!(output, "**Created:** {}{}", comment.created_at, edited_str).map_err(|e| {
                CliError::General(format!("Failed to write markdown metadata: {e}"))
            })?;

            // Body content
            writeln!(output, "\n{}\n", comment.body)
                .map_err(|e| CliError::General(format!("Failed to write markdown body: {e}")))?;
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
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);

        // Horizontal layout: column headers
        table.set_header(vec!["Author", "Body Preview", "Created", "Edited"]);

        // Each comment is a row
        for comment in &self.0 {
            // Truncate body for preview (first 80 chars)
            let body_preview = if comment.body.len() > 80 {
                format!("{}...", &comment.body[..77])
            } else {
                comment.body.clone()
            };

            table.add_row(vec![
                &comment.user_name,
                &body_preview,
                &comment.created_at,
                comment.edited_at.as_deref().unwrap_or(""),
            ]);
        }

        Ok(table.to_string())
    }
}

// From trait implementation for converting Cynic types to domain types

impl From<crate::client::queries::CommentNode> for Comment {
    fn from(node: crate::client::queries::CommentNode) -> Self {
        let (user_name, user_email) = node
            .user
            .map(|u| (u.name, u.email))
            .unwrap_or_else(|| ("Unknown".to_string(), "".to_string()));

        Comment {
            id: node.id.inner().to_string(),
            body: node.body,
            user_name,
            user_email,
            created_at: node.created_at.0,
            updated_at: node.updated_at.0,
            edited_at: node.edited_at.map(|d| d.0),
            issue_identifier: node.issue.map(|i| i.identifier),
        }
    }
}
