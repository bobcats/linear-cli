use crate::client::queries;
use crate::comments::types::Comment;
use crate::error::CliError;
use crate::output::{
    CsvResultExt, Formattable, MarkdownFormatter, TableFormatter, fast_markdown_formatter,
    generic_json_formatter, generic_json_list_formatter, generic_table_formatter,
};
use comfy_table::{Table, presets::UTF8_FULL};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Write as FmtWrite;

/// Format priority with emoji without allocation in hot paths
#[inline]
fn format_priority(priority: &Priority) -> String {
    // Pre-allocate: emoji (4 bytes) + space + max label length (~8 bytes) = ~15 bytes
    let mut buf = String::with_capacity(15);
    write!(buf, "{} {}", priority.emoji(), priority.as_str()).unwrap();
    buf
}

/// Priority level for an issue
/// Linear uses: 0=None, 1=Urgent, 2=High, 3=Medium, 4=Low
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    None = 0,
    Urgent = 1,
    High = 2,
    Medium = 3,
    Low = 4,
}

impl Priority {
    /// Convert from i32 (from GraphQL API) to Priority enum
    #[must_use]
    pub fn from_i32(value: i32) -> Self {
        match value {
            1 => Priority::Urgent,
            2 => Priority::High,
            3 => Priority::Medium,
            4 => Priority::Low,
            _ => Priority::None,
        }
    }

    /// Get human-readable string for priority
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::None => "None",
            Priority::Urgent => "Urgent",
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        }
    }

    /// Get emoji representation for visual clarity
    #[must_use]
    pub fn emoji(&self) -> &'static str {
        match self {
            Priority::None => "⚪",
            Priority::Urgent => "🔴",
            Priority::High => "🟠",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        }
    }

    /// Get numeric value
    #[must_use]
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// User information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// Issue state information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueState {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssueProject {
    pub id: String,
    pub name: String,
    pub slug_id: String,
}

/// Issue details returned from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub state: IssueState,
    pub priority: Priority,
    pub assignee: Option<User>,
    pub creator: User,
    pub project: Option<IssueProject>,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<Comment>>,
}

/// Wrapper type for a list of issues
/// Provides different formatting for collections vs single items
pub struct IssueList(pub Vec<Issue>);

impl Formattable for IssueList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row (same as single issue)
        wtr.write_record([
            "identifier",
            "title",
            "state",
            "priority",
            "assignee",
            "creator",
            "created_at",
            "updated_at",
            "url",
        ])
        .csv_err("Failed to write CSV header")?;

        // Write each issue as a row
        for issue in &self.0 {
            wtr.write_record([
                &issue.identifier,
                &issue.title,
                &issue.state.name,
                issue.priority.as_str(),
                issue
                    .assignee
                    .as_ref()
                    .map(|u| u.name.as_str())
                    .unwrap_or(""),
                &issue.creator.name,
                &issue.created_at,
                &issue.updated_at,
                &issue.url,
            ])
            .csv_err("Failed to write CSV row")?;
        }

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        // Pre-allocate: ~250 bytes per issue for structure + actual content
        let capacity = self
            .0
            .iter()
            .map(|i| {
                250 + i.identifier.len()
                    + i.title.len()
                    + i.state.name.len()
                    + i.description.as_ref().map_or(0, |d| d.len())
                    + i.creator.name.len()
            })
            .sum();
        let mut output = String::with_capacity(capacity);

        // Header with count
        writeln!(output, "## Issues ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown header: {e}")))?;

        // Output each issue as a summary card
        for issue in &self.0 {
            write!(output, "---\n\n### {}: {}\n", issue.identifier, issue.title)
                .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

            // Metadata on one line with emoji for priority
            writeln!(
                output,
                "**State:** {} | **Priority:** {} {} | **Assignee:** {}\n",
                issue.state.name,
                issue.priority.emoji(),
                issue.priority.as_str(),
                issue
                    .assignee
                    .as_ref()
                    .map(|u| u.name.as_str())
                    .unwrap_or("Unassigned")
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown metadata: {e}")))?;

            // Description
            if let Some(desc) = &issue.description {
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
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);

        // Horizontal layout: column headers
        table.set_header(vec!["ID", "Title", "State", "Priority", "Assignee"]);

        // Each issue is a row
        for issue in &self.0 {
            let priority_str = format_priority(&issue.priority);
            table.add_row(vec![
                &issue.identifier,
                &issue.title,
                &issue.state.name,
                &priority_str,
                issue
                    .assignee
                    .as_ref()
                    .map(|u| u.name.as_str())
                    .unwrap_or("—"),
            ]);
        }

        Ok(table.to_string())
    }
}

// Trait implementations for generic formatters
impl TableFormatter for Issue {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        let mut rows = vec![
            (
                Cow::Borrowed("Identifier"),
                Cow::Borrowed(self.identifier.as_str()),
            ),
            (Cow::Borrowed("Title"), Cow::Borrowed(self.title.as_str())),
            (
                Cow::Borrowed("State"),
                Cow::Borrowed(self.state.name.as_str()),
            ),
            (
                Cow::Borrowed("Priority"),
                Cow::Owned(format!(
                    "{} {}",
                    self.priority.emoji(),
                    self.priority.as_str()
                )),
            ),
            (
                Cow::Borrowed("Assignee"),
                self.assignee
                    .as_ref()
                    .map(|u| Cow::Borrowed(u.name.as_str()))
                    .unwrap_or(Cow::Borrowed("—")),
            ),
            (
                Cow::Borrowed("Creator"),
                Cow::Borrowed(self.creator.name.as_str()),
            ),
        ];

        if let Some(project) = &self.project {
            rows.push((
                Cow::Borrowed("Project"),
                Cow::Borrowed(project.name.as_str()),
            ));
        }

        if let Some(desc) = &self.description {
            rows.push((Cow::Borrowed("Description"), Cow::Borrowed(desc.as_str())));
        }

        rows.push((
            Cow::Borrowed("Created"),
            Cow::Borrowed(self.created_at.as_str()),
        ));
        rows.push((
            Cow::Borrowed("Updated"),
            Cow::Borrowed(self.updated_at.as_str()),
        ));
        rows.push((Cow::Borrowed("URL"), Cow::Borrowed(self.url.as_str())));

        // Add comment count if comments are present
        if let Some(comments) = &self.comments {
            rows.push((
                Cow::Borrowed("Comments"),
                Cow::Owned(comments.len().to_string()),
            ));
        }

        rows
    }
}

impl MarkdownFormatter for Issue {
    fn markdown_capacity_hint(&self) -> usize {
        // Same pre-allocation logic as original custom implementation
        300 + self.identifier.len()
            + self.title.len()
            + self.state.name.len()
            + self.description.as_ref().map_or(0, |d| d.len())
            + self.creator.name.len()
            + self.url.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        // Exact same logic as original custom implementation
        // H1 title with identifier
        writeln!(output, "# {}: {}\n", self.identifier, self.title)
            .map_err(|e| CliError::General(format!("Failed to write markdown title: {e}")))?;

        // Metadata section
        writeln!(output, "**State:** {}", self.state.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown state: {e}")))?;
        writeln!(
            output,
            "**Priority:** {} {}",
            self.priority.emoji(),
            self.priority.as_str()
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown priority: {e}")))?;
        writeln!(
            output,
            "**Assignee:** {}",
            self.assignee
                .as_ref()
                .map(|u| u.name.as_str())
                .unwrap_or("Unassigned")
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown assignee: {e}")))?;
        writeln!(output, "**Creator:** {}", self.creator.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown creator: {e}")))?;
        if let Some(project) = &self.project {
            writeln!(output, "**Project:** {}", project.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown project: {e}")))?;
        }
        writeln!(output).map_err(|e| CliError::General(format!("Failed to write newline: {e}")))?;

        // Description section
        write!(output, "## Description\n\n").map_err(|e| {
            CliError::General(format!("Failed to write markdown description header: {e}"))
        })?;
        if let Some(desc) = &self.description {
            writeln!(output, "{}\n", desc).map_err(|e| {
                CliError::General(format!("Failed to write markdown description: {e}"))
            })?;
        } else {
            write!(output, "[No description]\n\n").map_err(|e| {
                CliError::General(format!("Failed to write markdown placeholder: {e}"))
            })?;
        }

        // Details section
        writeln!(output, "## Details\n").map_err(|e| {
            CliError::General(format!("Failed to write markdown details header: {e}"))
        })?;
        writeln!(output, "- **Created:** {}", self.created_at).map_err(|e| {
            CliError::General(format!("Failed to write markdown created date: {e}"))
        })?;
        writeln!(output, "- **Updated:** {}", self.updated_at).map_err(|e| {
            CliError::General(format!("Failed to write markdown updated date: {e}"))
        })?;
        writeln!(output, "- **URL:** {}", self.url)
            .map_err(|e| CliError::General(format!("Failed to write markdown URL: {e}")))?;

        // Comments section (if present)
        if let Some(comments) = &self.comments
            && !comments.is_empty()
        {
            writeln!(output, "\n## Comments\n").map_err(|e| {
                CliError::General(format!("Failed to write markdown comments header: {e}"))
            })?;

            for comment in comments {
                writeln!(output, "**{}** ({})", comment.user_name, comment.created_at).map_err(
                    |e| CliError::General(format!("Failed to write comment author: {e}")),
                )?;
                writeln!(output, "{}\n", comment.body)
                    .map_err(|e| CliError::General(format!("Failed to write comment body: {e}")))?;
            }
        }

        Ok(())
    }
}

impl Formattable for Issue {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Write header row
        wtr.write_record([
            "identifier",
            "title",
            "state",
            "priority",
            "assignee",
            "creator",
            "created_at",
            "updated_at",
            "url",
            "comment_count",
        ])
        .csv_err("Failed to write CSV header")?;

        // Calculate comment count
        let comment_count = self
            .comments
            .as_ref()
            .map(|c| c.len().to_string())
            .unwrap_or_else(|| "0".to_string());

        // Write data row
        wtr.write_record([
            &self.identifier,
            &self.title,
            &self.state.name,
            self.priority.as_str(),
            self.assignee
                .as_ref()
                .map(|u| u.name.as_str())
                .unwrap_or(""),
            &self.creator.name,
            &self.created_at,
            &self.updated_at,
            &self.url,
            &comment_count,
        ])
        .csv_err("Failed to write CSV data")?;

        let data = wtr.into_inner().csv_err("Failed to finalize CSV")?;

        String::from_utf8(data).csv_err("Failed to convert CSV to UTF-8")
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        // Use fast generic formatter with writer pattern (zero intermediate allocations)
        fast_markdown_formatter(self)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_formatter(self)
    }
}

// From trait implementations for converting Cynic types to domain types

impl From<queries::IssueUser> for User {
    fn from(user: queries::IssueUser) -> Self {
        User {
            id: user.id.inner().to_string(),
            name: user.name,
            email: user.email,
        }
    }
}

impl From<queries::WorkflowState> for IssueState {
    fn from(state: queries::WorkflowState) -> Self {
        IssueState {
            id: state.id.inner().to_string(),
            name: state.name,
        }
    }
}

impl From<queries::IssueProject> for IssueProject {
    fn from(project: queries::IssueProject) -> Self {
        IssueProject {
            id: project.id.inner().to_string(),
            name: project.name,
            slug_id: project.slug_id,
        }
    }
}

impl TryFrom<queries::IssueNode> for Issue {
    type Error = CliError;

    fn try_from(node: queries::IssueNode) -> Result<Self, Self::Error> {
        Ok(Issue {
            id: node.id.inner().to_string(),
            identifier: node.identifier,
            title: node.title,
            description: node.description,
            state: node.state.into(),
            priority: Priority::from_i32(node.priority as i32),
            assignee: node.assignee.map(Into::into),
            creator: node
                .creator
                .map(Into::into)
                .ok_or_else(|| CliError::General("Issue creator not found".to_string()))?,
            project: node.project.map(Into::into),
            created_at: node.created_at.0,
            updated_at: node.updated_at.0,
            url: node.url,
            comments: None,
        })
    }
}

impl TryFrom<queries::SearchIssueNode> for Issue {
    type Error = CliError;

    fn try_from(node: queries::SearchIssueNode) -> Result<Self, Self::Error> {
        Ok(Issue {
            id: node.id.inner().to_string(),
            identifier: node.identifier,
            title: node.title,
            description: node.description,
            state: node.state.into(),
            priority: Priority::from_i32(node.priority as i32),
            assignee: node.assignee.map(Into::into),
            creator: node
                .creator
                .map(Into::into)
                .ok_or_else(|| CliError::General("Issue creator not found".to_string()))?,
            project: node.project.map(Into::into),
            created_at: node.created_at.0,
            updated_at: node.updated_at.0,
            url: node.url,
            comments: None,
        })
    }
}
