pub mod csv_ext;
pub mod format;
pub mod formatter;
pub mod generic_formatters;
pub mod streaming;
pub mod traits;

pub use csv_ext::CsvResultExt;
pub use format::{
    JsonStyle, OutputFormat, detect_format, detect_format_with_provider, get_format,
    get_format_with_provider, resolve_json_style, resolve_json_style_for_tty_with_provider,
    resolve_json_style_with_provider,
};
pub use formatter::{Formattable, format_output};
pub use generic_formatters::{
    fast_markdown_formatter, generic_json_formatter, generic_json_list_formatter,
    generic_table_formatter, generic_table_list_formatter,
};
pub use streaming::format_output_to_writer;
pub use traits::{MarkdownFormatter, MarkdownSection, StructuredData, TableFormatter};
