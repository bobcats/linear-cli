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

/// User from Linear API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub active: bool,
    pub admin: bool,
    pub guest: bool,
}

impl User {
    fn role(&self) -> &'static str {
        if self.admin {
            "Admin"
        } else if self.guest {
            "Guest"
        } else {
            "Member"
        }
    }
}

impl TableFormatter for User {
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
        vec![
            (Cow::Borrowed("Name"), Cow::Borrowed(self.name.as_str())),
            (
                Cow::Borrowed("Display Name"),
                Cow::Borrowed(self.display_name.as_str()),
            ),
            (Cow::Borrowed("Email"), Cow::Borrowed(self.email.as_str())),
            (Cow::Borrowed("Role"), Cow::Borrowed(self.role())),
            (
                Cow::Borrowed("Active"),
                Cow::Borrowed(if self.active { "Yes" } else { "No" }),
            ),
            (Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())),
        ]
    }
}

impl MarkdownFormatter for User {
    fn markdown_capacity_hint(&self) -> usize {
        150 + self.name.len() + self.display_name.len() + self.email.len()
    }

    fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
        writeln!(output, "# {}\n", self.name)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        writeln!(output, "**Display Name:** {}", self.display_name)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**Email:** {}", self.email)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**Role:** {}", self.role())
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(
            output,
            "**Active:** {}",
            if self.active { "Yes" } else { "No" }
        )
        .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        writeln!(output, "**ID:** {}", self.id)
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        Ok(())
    }
}

impl Formattable for User {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_formatter(self)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["name", "display_name", "email", "role", "active", "id"])
            .csv_err("Failed to write CSV header")?;

        wtr.write_record([
            self.name.as_str(),
            self.display_name.as_str(),
            self.email.as_str(),
            self.role(),
            if self.active { "true" } else { "false" },
            self.id.as_str(),
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

/// Wrapper for a list of users
pub struct UserList(pub Vec<User>);

impl Formattable for UserList {
    fn to_json(&self) -> Result<String, CliError> {
        generic_json_list_formatter(&self.0)
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        wtr.write_record(["name", "display_name", "email", "role", "active"])
            .csv_err("Failed to write CSV header")?;

        for user in &self.0 {
            wtr.write_record([
                user.name.as_str(),
                user.display_name.as_str(),
                user.email.as_str(),
                user.role(),
                if user.active { "true" } else { "false" },
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
            .map(|u| 100 + u.name.len() + u.email.len())
            .sum();
        let mut output = String::with_capacity(capacity);

        writeln!(output, "## Users ({})\n", self.0.len())
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;

        for user in &self.0 {
            writeln!(output, "---\n")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(output, "### {}\n", user.name)
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
            writeln!(
                output,
                "**Email:** {} | **Role:** {} | **Active:** {}\n",
                user.email,
                user.role(),
                if user.active { "Yes" } else { "No" }
            )
            .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        if !self.0.is_empty() {
            writeln!(output, "---")
                .map_err(|e| CliError::General(format!("Failed to write markdown: {e}")))?;
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        generic_table_list_formatter(&self.0, &["Name", "Email", "Role", "Active"], |user| {
            vec![
                user.name.clone(),
                user.email.clone(),
                user.role().to_string(),
                if user.active { "Yes" } else { "No" }.to_string(),
            ]
        })
    }
}

impl From<queries::UserNode> for User {
    fn from(node: queries::UserNode) -> Self {
        User {
            id: node.id.inner().to_string(),
            name: node.name,
            display_name: node.display_name,
            email: node.email,
            active: node.active,
            admin: node.admin,
            guest: node.guest,
        }
    }
}
