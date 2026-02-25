use crate::auth::config::ConfigProvider;
use std::io::IsTerminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Csv,
    Markdown,
    Table,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonStyle {
    Compact,
    Pretty,
}

/// Detect format from environment variable using ConfigProvider
/// Falls back to Auto if not set
pub fn detect_format_with_provider(config: &dyn ConfigProvider) -> OutputFormat {
    if let Some(format_str) = config.get_var("LINEAR_CLI_FORMAT") {
        return match format_str.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            "markdown" | "md" => OutputFormat::Markdown,
            "table" => OutputFormat::Table,
            "auto" => OutputFormat::Auto,
            _ => OutputFormat::Auto,
        };
    }

    // Default to Auto (will be resolved based on TTY)
    OutputFormat::Auto
}

/// Resolve JSON rendering style from environment + terminal context.
///
/// Precedence:
/// 1) `LINEAR_CLI_JSON_STYLE` env var (`pretty` or `compact`)
/// 2) Interactive fallback (`pretty` when stdout is a TTY)
/// 3) Non-interactive fallback (`compact`)
pub fn resolve_json_style_with_provider(config: &dyn ConfigProvider) -> JsonStyle {
    resolve_json_style_for_tty_with_provider(config, std::io::stdout().is_terminal())
}

/// Deterministic helper for tests/callers that already know terminal context.
#[must_use]
pub fn resolve_json_style_for_tty_with_provider(
    config: &dyn ConfigProvider,
    is_tty: bool,
) -> JsonStyle {
    match config
        .get_var("LINEAR_CLI_JSON_STYLE")
        .as_deref()
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("pretty") => JsonStyle::Pretty,
        Some("compact") => JsonStyle::Compact,
        _ if is_tty => JsonStyle::Pretty,
        _ => JsonStyle::Compact,
    }
}

/// Legacy helper using real environment variables.
#[must_use]
pub fn resolve_json_style() -> JsonStyle {
    use crate::auth::config::EnvConfigProvider;
    resolve_json_style_with_provider(&EnvConfigProvider)
}

/// Legacy function for backward compatibility (uses real env vars)
/// Prefer detect_format_with_provider for testability
#[must_use]
pub fn detect_format() -> OutputFormat {
    use crate::auth::config::EnvConfigProvider;
    detect_format_with_provider(&EnvConfigProvider)
}

/// Get the final output format with proper precedence:
/// 1. Explicit format from CLI flags (highest priority)
/// 2. Environment variable (LINEAR_CLI_FORMAT)
/// 3. Auto-detection based on TTY (lowest priority)
pub fn get_format_with_provider(
    cli_format: Option<OutputFormat>,
    config: &dyn ConfigProvider,
) -> OutputFormat {
    cli_format
        .unwrap_or_else(|| detect_format_with_provider(config))
        .resolve()
}

/// Legacy function for backward compatibility (uses real env vars)
/// Prefer get_format_with_provider for testability
#[must_use]
pub fn get_format(cli_format: Option<OutputFormat>) -> OutputFormat {
    use crate::auth::config::EnvConfigProvider;
    get_format_with_provider(cli_format, &EnvConfigProvider)
}

impl OutputFormat {
    /// Resolve Auto format based on whether stdout is a TTY
    #[must_use]
    pub fn resolve(self) -> OutputFormat {
        match self {
            OutputFormat::Auto => {
                if std::io::stdout().is_terminal() {
                    OutputFormat::Table
                } else {
                    OutputFormat::Json
                }
            }
            other => other,
        }
    }
}
