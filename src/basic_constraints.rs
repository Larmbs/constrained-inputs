//! Implements simple constraint types
use crate::constraints::Constraint;
use crate::error::{Error, ErrorKind, Result};

/// Simple constraint to be applied on Strings
pub struct StringConstraint {
    exclude_char: Vec<char>,
    include_char: Vec<char>,
    max_len: usize,
    min_len: usize,
}

impl Constraint<String> for StringConstraint {
    fn validate(&self, data: &String) -> Result<()> {
        let len = data.len();

        // Check minimum length
        if len < self.min_len {
            return Err(Error {
                kind: ErrorKind::ValidationError,
                message: format!("String must be at least {} characters long.", self.min_len),
            });
        }

        // Check maximum length
        if len > self.max_len {
            return Err(Error {
                kind: ErrorKind::ValidationError,
                message: format!(
                    "String must be no more than {} characters long.",
                    self.max_len
                ),
            });
        }

        // Check for excluded characters
        for chr in &self.exclude_char {
            if data.contains(*chr) {
                return Err(Error {
                    kind: ErrorKind::ValidationError,
                    message: format!("String must not contain the character '{}'.", chr),
                });
            }
        }

        // Check for included characters
        for chr in &self.include_char {
            if !data.contains(*chr) {
                return Err(Error {
                    kind: ErrorKind::ValidationError,
                    message: format!("String must contain the character '{}'.", chr),
                });
            }
        }

        Ok(())
    }
}

/// Simple constraint to apply to numbers
pub struct NumberConstraint {
    min_value: f64,
    max_value: f64,
}

impl<T> Constraint<T> for NumberConstraint
where
    T: PartialOrd + Copy + Into<f64>,
{
    fn validate(&self, data: &T) -> Result<()> {
        let value: f64 = (*data).into();

        // Check minimum value
        if value < self.min_value {
            return Err(Error {
                kind: ErrorKind::ValidationError,
                message: format!("Number must be at least {}.", self.min_value),
            });
        }

        // Check maximum value
        if value > self.max_value {
            return Err(Error {
                kind: ErrorKind::ValidationError,
                message: format!("Number must be no more than {}.", self.max_value),
            });
        }

        Ok(())
    }
}
