use crate::error::CliError;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Trait for terminal I/O operations
pub trait Io: Send + Sync {
    /// Read a secret (password/token) from user with optional prompt
    fn read_secret(&self, prompt: &str) -> Result<String, CliError>;

    /// Print a message to stdout
    fn print(&self, message: &str);

    /// Print raw bytes to stdout (streaming path).
    ///
    /// Default implementation falls back to UTF-8 string printing.
    fn print_bytes(&self, bytes: &[u8]) {
        let message = String::from_utf8_lossy(bytes);
        self.print(&message);
    }

    /// Print an error message to stderr
    fn print_error(&self, message: &str);
}

/// Production implementation using real stdin/stdout/stderr
pub struct RealIo;

impl Io for RealIo {
    fn read_secret(&self, prompt: &str) -> Result<String, CliError> {
        eprint!("{}", prompt);
        io::stderr().flush().ok();

        // Use rpassword to read without echoing to terminal
        rpassword::read_password()
            .map_err(|e| CliError::General(format!("Failed to read password: {}", e)))
    }

    fn print(&self, message: &str) {
        println!("{}", message);
    }

    fn print_bytes(&self, bytes: &[u8]) {
        let mut stdout = io::stdout().lock();
        let _ = stdout.write_all(bytes);
        let _ = stdout.write_all(b"\n");
        let _ = stdout.flush();
    }

    fn print_error(&self, message: &str) {
        eprintln!("{}", message);
    }
}

/// Mock implementation for testing
pub struct MockIo {
    pub input: String,
    pub stdout: Arc<Mutex<Vec<String>>>,
    pub stderr: Arc<Mutex<Vec<String>>>,
}

impl Default for MockIo {
    fn default() -> Self {
        Self::new()
    }
}

impl MockIo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            input: String::new(),
            stdout: Arc::new(Mutex::new(Vec::new())),
            stderr: Arc::new(Mutex::new(Vec::new())),
        }
    }

    #[must_use]
    pub fn with_input(input: String) -> Self {
        Self {
            input,
            stdout: Arc::new(Mutex::new(Vec::new())),
            stderr: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get captured stdout lines for assertions
    #[must_use]
    pub fn stdout_lines(&self) -> Vec<String> {
        self.stdout.lock().unwrap().clone()
    }

    /// Get captured stderr lines for assertions
    #[must_use]
    pub fn stderr_lines(&self) -> Vec<String> {
        self.stderr.lock().unwrap().clone()
    }
}

impl Io for MockIo {
    fn read_secret(&self, _prompt: &str) -> Result<String, CliError> {
        Ok(self.input.clone())
    }

    fn print(&self, message: &str) {
        self.stdout.lock().unwrap().push(message.to_string());
    }

    fn print_bytes(&self, bytes: &[u8]) {
        self.stdout
            .lock()
            .unwrap()
            .push(String::from_utf8_lossy(bytes).to_string());
    }

    fn print_error(&self, message: &str) {
        self.stderr.lock().unwrap().push(message.to_string());
    }
}
