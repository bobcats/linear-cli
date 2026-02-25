/// Generic formatter implementations using the trait infrastructure
///
/// These generic functions can be used by any type that implements the appropriate traits,
/// eliminating the need for each type to write its own formatter boilerplate.
use crate::auth::config::EnvConfigProvider;
use crate::error::CliError;
use crate::output::format::JsonStyle;
use crate::output::resolve_json_style_with_provider;
use crate::output::traits::{MarkdownFormatter, StructuredData, TableFormatter};
use comfy_table::{Cell, Table, presets::UTF8_FULL};

/// Generic JSON formatter for any type that implements StructuredData
///
/// This eliminates the need for each type to implement to_json() manually.
///
/// # Example
/// ```ignore
/// impl Formattable for Issue {
///     fn to_json(&self) -> Result<String, CliError> {
///         generic_json_formatter(self)
///     }
/// }
/// ```
pub fn generic_json_formatter<T: StructuredData>(data: &T) -> Result<String, CliError> {
    let config = EnvConfigProvider;
    let style = resolve_json_style_with_provider(&config);

    match style {
        JsonStyle::Compact => serde_json::to_string(data)
            .map_err(|e| CliError::General(format!("Failed to serialize to JSON: {e}"))),
        JsonStyle::Pretty => serde_json::to_string_pretty(data)
            .map_err(|e| CliError::General(format!("Failed to serialize to JSON: {e}"))),
    }
}

/// Generic table formatter for any type that implements TableFormatter
///
/// Creates a vertical table (2 columns) with field names and values.
///
/// # Example
/// ```ignore
/// impl Formattable for Issue {
///     fn to_table(&self) -> Result<String, CliError> {
///         generic_table_formatter(self)
///     }
/// }
/// ```
pub fn generic_table_formatter<T: TableFormatter>(data: &T) -> Result<String, CliError> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // Add each row from the data
    for (field, value) in data.table_rows() {
        table.add_row(vec![Cell::new(field), Cell::new(value)]);
    }

    Ok(table.to_string())
}

/// Generic table formatter for list/collection layouts with explicit headers.
///
/// The row builder closure enables low-overhead row projection for each item.
pub fn generic_table_list_formatter<T, F>(
    items: &[T],
    headers: &[&str],
    mut row_builder: F,
) -> Result<String, CliError>
where
    F: FnMut(&T) -> Vec<String>,
{
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(headers.iter().copied().map(Cell::new));

    for item in items {
        table.add_row(row_builder(item));
    }

    Ok(table.to_string())
}

/// Fast generic markdown formatter using writer pattern (zero intermediate allocations)
///
/// This formatter achieves identical performance to hand-written formatters
/// by using a writer pattern that avoids intermediate allocations.
///
/// # Example
/// ```ignore
/// impl Formattable for Issue {
///     fn to_markdown(&self) -> Result<String, CliError> {
///         fast_markdown_formatter(self)
///     }
/// }
/// ```
pub fn fast_markdown_formatter<T: MarkdownFormatter>(data: &T) -> Result<String, CliError> {
    let capacity = data.markdown_capacity_hint();
    let mut output = String::with_capacity(capacity);
    data.write_markdown(&mut output)?;
    Ok(output)
}

/// Generic JSON formatter for list types
///
/// Handles Vec<T> where T implements Serialize.
/// This is a specialized version for collections.
///
/// # Example
/// ```ignore
/// impl Formattable for IssueList {
///     fn to_json(&self) -> Result<String, CliError> {
///         generic_json_list_formatter(&self.0)
///     }
/// }
/// ```
pub fn generic_json_list_formatter<T: serde::Serialize>(items: &[T]) -> Result<String, CliError> {
    let config = EnvConfigProvider;
    let style = resolve_json_style_with_provider(&config);

    match style {
        JsonStyle::Compact => serde_json::to_string(items)
            .map_err(|e| CliError::General(format!("Failed to serialize list to JSON: {e}"))),
        JsonStyle::Pretty => serde_json::to_string_pretty(items)
            .map_err(|e| CliError::General(format!("Failed to serialize list to JSON: {e}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use std::borrow::Cow;

    #[derive(Serialize)]
    struct TestData {
        id: String,
        value: i32,
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
    fn test_generic_json_formatter() {
        let data = TestData {
            id: "test-123".to_string(),
            value: 42,
        };

        let json = generic_json_formatter(&data).unwrap();
        assert!(json.contains("\"id\":\"test-123\""));
        assert!(json.contains("\"value\":42"));
    }

    #[test]
    fn test_generic_json_list_formatter() {
        let items = vec![
            TestData {
                id: "item-1".to_string(),
                value: 10,
            },
            TestData {
                id: "item-2".to_string(),
                value: 20,
            },
        ];

        let json = generic_json_list_formatter(&items).unwrap();
        assert!(json.contains("\"id\":\"item-1\""));
        assert!(json.contains("\"id\":\"item-2\""));
        assert!(json.contains("\"value\":10"));
        assert!(json.contains("\"value\":20"));
    }

    #[test]
    fn test_generic_table_formatter() {
        let data = TestData {
            id: "test-456".to_string(),
            value: 99,
        };

        let table = generic_table_formatter(&data).unwrap();
        assert!(table.contains("ID"));
        assert!(table.contains("test-456"));
        assert!(table.contains("Value"));
        assert!(table.contains("99"));
    }

    #[test]
    fn test_generic_json_formatter_with_nested_structure() {
        #[derive(Serialize)]
        struct NestedData {
            outer: TestData,
        }

        let data = NestedData {
            outer: TestData {
                id: "nested".to_string(),
                value: 999,
            },
        };

        let json = generic_json_formatter(&data).unwrap();
        assert!(json.contains("\"outer\""));
        assert!(json.contains("\"id\":\"nested\""));
        assert!(json.contains("\"value\":999"));
    }
}
