//! Extension trait for CSV Result types to reduce boilerplate error mapping

use crate::error::CliError;

/// Extension trait for CSV Writer Results to simplify error mapping
pub trait CsvResultExt<T> {
    /// Map CSV errors to CliError::General with a custom message
    fn csv_err(self, msg: &str) -> Result<T, CliError>;
}

impl<T, E: std::fmt::Display> CsvResultExt<T> for Result<T, E> {
    fn csv_err(self, msg: &str) -> Result<T, CliError> {
        self.map_err(|e| CliError::General(format!("{msg}: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_err_ok() {
        let result: Result<i32, String> = Ok(42);
        assert_eq!(result.csv_err("test").unwrap(), 42);
    }

    #[test]
    fn test_csv_err_err() {
        let result: Result<i32, String> = Err("write failed".to_string());
        let err = result.csv_err("Failed to write CSV").unwrap_err();

        match err {
            CliError::General(msg) => {
                assert_eq!(msg, "Failed to write CSV: write failed");
            }
            _ => panic!("Expected General error"),
        }
    }
}
