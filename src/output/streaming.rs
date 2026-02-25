use crate::error::CliError;
use crate::output::{Formattable, OutputFormat, format_output};
use std::io::Write;

/// Writer-based output API for parity with string-based formatting.
///
/// This is an initial compatibility layer: it guarantees identical bytes to
/// `format_output` by delegating to the existing formatters, then writing to
/// the provided writer.
pub fn format_output_to_writer<T: Formattable, W: Write>(
    data: &T,
    format: OutputFormat,
    writer: &mut W,
) -> Result<(), CliError> {
    let output = format_output(data, format)?;
    writer
        .write_all(output.as_bytes())
        .map_err(|e| CliError::General(format!("Failed to write formatted output: {e}")))
}
