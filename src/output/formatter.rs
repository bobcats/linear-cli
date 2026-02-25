use crate::error::CliError;
use crate::output::OutputFormat;

/// Trait that enforces all data types to implement formatters for all supported output formats.
///
/// This trait uses Rust's type system to ensure compile-time enforcement:
/// if a type doesn't implement all four methods, the code will not compile.
pub trait Formattable {
    /// Format as JSON (pretty-printed)
    fn to_json(&self) -> Result<String, CliError>;

    /// Format as CSV (with headers)
    fn to_csv(&self) -> Result<String, CliError>;

    /// Format as Markdown (GitHub Flavored Markdown)
    fn to_markdown(&self) -> Result<String, CliError>;

    /// Format as Table (UTF-8 box drawing for terminal display)
    fn to_table(&self) -> Result<String, CliError>;
}

/// Format output using the specified format.
///
/// This function routes to the appropriate formatter method based on the OutputFormat.
/// The format should already be resolved (no Auto variant).
///
/// # Example
/// ```ignore
/// let issue = Issue { ... };
/// let output = format_output(&issue, OutputFormat::Json)?;
/// io.print(&output);
/// ```
pub fn format_output<T: Formattable>(data: &T, format: OutputFormat) -> Result<String, CliError> {
    match format {
        OutputFormat::Json => data.to_json(),
        OutputFormat::Csv => data.to_csv(),
        OutputFormat::Markdown => data.to_markdown(),
        OutputFormat::Table => data.to_table(),
        OutputFormat::Auto => {
            // This should never happen if get_format() is used properly,
            // but provide a fallback just in case
            data.to_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test implementation of Formattable
    struct TestData {
        value: String,
    }

    impl Formattable for TestData {
        fn to_json(&self) -> Result<String, CliError> {
            Ok(format!(r#"{{"value":"{}"}}"#, self.value))
        }

        fn to_csv(&self) -> Result<String, CliError> {
            Ok(format!("value\n{}", self.value))
        }

        fn to_markdown(&self) -> Result<String, CliError> {
            Ok(format!("# {}", self.value))
        }

        fn to_table(&self) -> Result<String, CliError> {
            Ok(format!("┌─────┐\n│ {} │\n└─────┘", self.value))
        }
    }

    #[test]
    fn test_format_output_routes_to_json() {
        let data = TestData {
            value: "test".to_string(),
        };
        let output = format_output(&data, OutputFormat::Json).unwrap();
        assert_eq!(output, r#"{"value":"test"}"#);
    }

    #[test]
    fn test_format_output_routes_to_csv() {
        let data = TestData {
            value: "test".to_string(),
        };
        let output = format_output(&data, OutputFormat::Csv).unwrap();
        assert_eq!(output, "value\ntest");
    }

    #[test]
    fn test_format_output_routes_to_markdown() {
        let data = TestData {
            value: "test".to_string(),
        };
        let output = format_output(&data, OutputFormat::Markdown).unwrap();
        assert_eq!(output, "# test");
    }

    #[test]
    fn test_format_output_routes_to_table() {
        let data = TestData {
            value: "test".to_string(),
        };
        let output = format_output(&data, OutputFormat::Table).unwrap();
        assert_eq!(output, "┌─────┐\n│ test │\n└─────┘");
    }

    #[test]
    fn test_format_output_auto_falls_back_to_json() {
        let data = TestData {
            value: "test".to_string(),
        };
        let output = format_output(&data, OutputFormat::Auto).unwrap();
        assert_eq!(output, r#"{"value":"test"}"#);
    }
}
