/// Traits for generic formatter infrastructure
///
/// These traits separate data extraction from formatting, allowing us to
/// eliminate ~1,500 lines of duplicated formatter code across domain types.
use crate::error::CliError;
use serde::Serialize;
use std::borrow::Cow;

/// Trait for types that can be represented as structured JSON data
///
/// This trait is automatically implemented for all types that implement Serialize.
/// It provides the data layer for JSON formatting.
pub trait StructuredData: Serialize {
    /// Convert to a serde_json::Value for flexible JSON manipulation
    fn to_json_value(&self) -> Result<serde_json::Value, CliError> {
        serde_json::to_value(self)
            .map_err(|e| CliError::General(format!("Failed to serialize to JSON value: {e}")))
    }
}

/// Blanket implementation: any Serialize type is StructuredData
impl<T: Serialize> StructuredData for T {}

/// Trait for types that can be represented as table rows (key-value pairs)
///
/// This trait defines how a type should be displayed in a vertical table format,
/// where each row is a field name and its value.
///
/// Uses `Cow<str>` to avoid allocations when possible:
/// - Field names can use `Cow::Borrowed("Name")` (static strings, zero allocation)
/// - String values can use `Cow::Borrowed(&self.field)` (borrowed, zero allocation)
/// - Formatted values can use `Cow::Owned(format!(...))` (only allocate when necessary)
///
/// # Example
/// ```ignore
/// impl TableFormatter for Issue {
///     fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
///         vec![
///             (Cow::Borrowed("ID"), Cow::Borrowed(&self.identifier)),
///             (Cow::Borrowed("Title"), Cow::Borrowed(&self.title)),
///             (Cow::Borrowed("Priority"), Cow::Owned(format!("{} {}", self.priority.emoji(), self.priority.as_str()))),
///             // ...
///         ]
///     }
/// }
/// ```
pub trait TableFormatter {
    /// Return a list of (field_name, field_value) pairs for table display
    ///
    /// Use `Cow::Borrowed` for zero-allocation references, `Cow::Owned` only when formatting is needed.
    fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)>;
}

/// Represents a section in a Markdown document
#[derive(Debug, Clone)]
pub struct MarkdownSection {
    /// Section heading (without ## prefix)
    pub heading: String,
    /// Section content (can be multiple paragraphs, lists, etc.)
    pub content: String,
}

/// Trait for types that can be represented as structured Markdown documents
///
/// This trait uses a writer pattern to avoid intermediate allocations,
/// achieving performance identical to hand-written formatters.
///
/// # Example
/// ```ignore
/// impl MarkdownFormatter for Issue {
///     fn markdown_capacity_hint(&self) -> usize {
///         300 + self.identifier.len() + self.title.len()
///     }
///
///     fn write_markdown(&self, output: &mut String) -> Result<(), CliError> {
///         writeln!(output, "# {}: {}", self.identifier, self.title)?;
///         writeln!(output, "**State:** {}", self.state.name)?;
///         Ok(())
///     }
/// }
/// ```
pub trait MarkdownFormatter {
    /// Hint for pre-allocating output capacity (avoids reallocations)
    ///
    /// Default is 256 bytes. Override with sum of expected content lengths
    /// for optimal performance.
    fn markdown_capacity_hint(&self) -> usize {
        256
    }

    /// Write markdown directly to output string
    ///
    /// This method should write the complete markdown representation
    /// directly to the provided String buffer using write!/writeln! macros.
    ///
    /// Returns CliError on write failures.
    fn write_markdown(&self, output: &mut String) -> Result<(), CliError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        id: String,
        value: i32,
    }

    #[test]
    fn test_structured_data_blanket_impl() {
        let data = TestData {
            id: "test-123".to_string(),
            value: 42,
        };

        let json_value = data.to_json_value().unwrap();
        assert_eq!(json_value["id"], "test-123");
        assert_eq!(json_value["value"], 42);
    }

    #[test]
    fn test_structured_data_handles_nested_structures() {
        #[derive(Serialize)]
        struct Nested {
            inner: TestData,
        }

        let data = Nested {
            inner: TestData {
                id: "nested".to_string(),
                value: 100,
            },
        };

        let json_value = data.to_json_value().unwrap();
        assert_eq!(json_value["inner"]["id"], "nested");
        assert_eq!(json_value["inner"]["value"], 100);
    }

    impl TableFormatter for TestData {
        fn table_rows(&self) -> Vec<(Cow<'_, str>, Cow<'_, str>)> {
            vec![
                (Cow::Borrowed("ID"), Cow::Borrowed(self.id.as_str())),
                (Cow::Borrowed("Value"), Cow::Owned(self.value.to_string())),
            ]
        }
    }

    #[test]
    fn test_table_formatter_returns_rows() {
        let data = TestData {
            id: "test-456".to_string(),
            value: 99,
        };

        let rows = data.table_rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "ID");
        assert_eq!(rows[0].1, "test-456");
        assert_eq!(rows[1].0, "Value");
        assert_eq!(rows[1].1, "99");
    }

    #[test]
    fn test_markdown_section_clone() {
        let section = MarkdownSection {
            heading: "Test".to_string(),
            content: "Content".to_string(),
        };

        let cloned = section.clone();
        assert_eq!(cloned.heading, "Test");
        assert_eq!(cloned.content, "Content");
    }
}
