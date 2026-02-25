use crate::auth::config::ConfigProvider;
use crate::auth::output::LogoutResult;
use crate::auth::storage::TokenStorage;
use crate::error::CliError;
use crate::io::Io;
use crate::output::{OutputFormat, format_output, get_format_with_provider};

/// Handle the logout command
pub fn handle_logout(
    storage: &dyn TokenStorage,
    config: &dyn ConfigProvider,
    io: &dyn Io,
    format_flag: Option<OutputFormat>,
) -> Result<(), CliError> {
    storage.delete()?;

    // Create logout result
    let result = LogoutResult {
        success: true,
        message: "Logged out successfully. Token removed from keyring.".to_string(),
    };

    // Determine output format (CLI flag > env var > auto-detect)
    let format = get_format_with_provider(format_flag, config);

    // Format and output
    let output = format_output(&result, format)?;
    io.print(&output);

    Ok(())
}
