use crate::error::CliError;
use crate::output::Formattable;
use comfy_table::{Cell, Table, presets::UTF8_FULL};
use serde::{Deserialize, Serialize};
use std::fmt::Write as FmtWrite;

/// Token source for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenSource {
    LinearToken,    // LINEAR_TOKEN env var
    LinearApiToken, // LINEAR_API_TOKEN env var
    Keyring,        // System keyring
}

impl TokenSource {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LinearToken => "environment variable (LINEAR_TOKEN)",
            Self::LinearApiToken => "environment variable (LINEAR_API_TOKEN)",
            Self::Keyring => "keyring",
        }
    }
}

/// Authentication status output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    pub logged_in: bool,
    pub user_name: String,
    pub user_email: String,
    pub token: String, // May be redacted
    pub token_source: TokenSource,
    pub show_full_token: bool,
}

impl Formattable for AuthStatus {
    fn to_json(&self) -> Result<String, CliError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| CliError::General(format!("Failed to serialize auth status to JSON: {e}")))
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Header
        wtr.write_record([
            "logged_in",
            "user_name",
            "user_email",
            "token",
            "token_source",
        ])
        .map_err(|e| CliError::General(format!("Failed to write CSV header: {e}")))?;

        // Data row
        wtr.write_record([
            &self.logged_in.to_string(),
            &self.user_name,
            &self.user_email,
            &self.token,
            self.token_source.as_str(),
        ])
        .map_err(|e| CliError::General(format!("Failed to write CSV data: {e}")))?;

        let data = wtr
            .into_inner()
            .map_err(|e| CliError::General(format!("Failed to finalize CSV: {e}")))?;

        String::from_utf8(data)
            .map_err(|e| CliError::General(format!("Failed to convert CSV to UTF-8: {e}")))
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        let mut output = String::new();

        // Title
        output.push_str("# Linear Authentication Status\n\n");

        // Status indicator
        if self.logged_in {
            output.push_str("✓ **Logged in**\n\n");
        } else {
            output.push_str("✗ **Not logged in**\n\n");
        }

        // User info
        output.push_str("## User\n\n");
        writeln!(output, "- **Name:** {}", self.user_name)
            .map_err(|e| CliError::General(format!("Failed to write markdown name: {e}")))?;
        writeln!(output, "- **Email:** {}\n", self.user_email)
            .map_err(|e| CliError::General(format!("Failed to write markdown email: {e}")))?;

        // Token info
        output.push_str("## Token\n\n");
        writeln!(output, "- **Value:** `{}`", self.token)
            .map_err(|e| CliError::General(format!("Failed to write markdown token: {e}")))?;
        writeln!(output, "- **Source:** {}", self.token_source.as_str())
            .map_err(|e| CliError::General(format!("Failed to write markdown source: {e}")))?;

        // Warning if showing full token
        if self.show_full_token {
            output.push_str(
                "\n> ⚠️ **Warning:** Your authentication token is shown above. Keep it secret!\n",
            );
        }

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);

        // Vertical layout
        table.add_row(vec![
            Cell::new("Status"),
            Cell::new(if self.logged_in {
                "✓ Logged in"
            } else {
                "✗ Not logged in"
            }),
        ]);
        table.add_row(vec![Cell::new("User"), Cell::new(&self.user_name)]);
        table.add_row(vec![Cell::new("Email"), Cell::new(&self.user_email)]);
        table.add_row(vec![Cell::new("Token"), Cell::new(&self.token)]);
        table.add_row(vec![
            Cell::new("Source"),
            Cell::new(self.token_source.as_str()),
        ]);

        let mut output = table.to_string();

        // Add warning if showing full token
        if self.show_full_token {
            output.push_str(
                "\n\n⚠ Warning: Your authentication token is shown above. Keep it secret!",
            );
        }

        Ok(output)
    }
}

/// Logout result output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResult {
    pub success: bool,
    pub message: String,
}

impl Formattable for LogoutResult {
    fn to_json(&self) -> Result<String, CliError> {
        serde_json::to_string_pretty(self).map_err(|e| {
            CliError::General(format!("Failed to serialize logout result to JSON: {e}"))
        })
    }

    fn to_csv(&self) -> Result<String, CliError> {
        let mut wtr = csv::Writer::from_writer(vec![]);

        // Header
        wtr.write_record(["success", "message"])
            .map_err(|e| CliError::General(format!("Failed to write CSV header: {e}")))?;

        // Data row
        wtr.write_record([&self.success.to_string(), &self.message])
            .map_err(|e| CliError::General(format!("Failed to write CSV data: {e}")))?;

        let data = wtr
            .into_inner()
            .map_err(|e| CliError::General(format!("Failed to finalize CSV: {e}")))?;

        String::from_utf8(data)
            .map_err(|e| CliError::General(format!("Failed to convert CSV to UTF-8: {e}")))
    }

    fn to_markdown(&self) -> Result<String, CliError> {
        let mut output = String::new();

        // Title
        output.push_str("# Logout\n\n");

        // Status
        if self.success {
            output.push_str("✓ **Success**\n\n");
        } else {
            output.push_str("✗ **Failed**\n\n");
        }

        // Message
        writeln!(output, "{}", self.message)
            .map_err(|e| CliError::General(format!("Failed to write markdown message: {e}")))?;

        Ok(output)
    }

    fn to_table(&self) -> Result<String, CliError> {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);

        table.add_row(vec![
            Cell::new("Status"),
            Cell::new(if self.success {
                "✓ Success"
            } else {
                "✗ Failed"
            }),
        ]);
        table.add_row(vec![Cell::new("Message"), Cell::new(&self.message)]);

        Ok(table.to_string())
    }
}
