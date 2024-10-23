//! Defines error type for crate

/// Categories of errors
#[derive(Debug)]
pub enum ErrorKind {
    Other,
    ValidationError,
    IOError,
    ParseError,
}

/// Error type for constrained inputs
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}: {}", self.kind, self.message))?;
        Ok(())
    }
}

/// Result type for constrained inputs
pub type Result<T> = std::result::Result<T, Error>;
