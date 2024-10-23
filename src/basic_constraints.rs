//! Implements simple constraint types
use crate::constraints::Constraint;
use crate::error::{Error, ErrorKind, Result};

/// Simple constraint to be applied on Strings
pub struct StringConstraint {
    pub exclude_char: Vec<char>,
    pub include_char: Vec<char>,
    pub max_len: usize,
    pub min_len: usize,
}

impl Constraint<String> for StringConstraint {
    fn validate(&self, data: &String) -> Result<()> {
        // Check minimum length
        if self.min_len > data.len() {
            return Err(Error {
                kind: ErrorKind::ValidationError,
                message: format!("String must be at least {} characters long.", self.min_len),
            });
        }

        // Check maximum length
        if self.max_len < data.len() {
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
            if !data.contains(&chr.to_string()) {
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
    pub min_value: f64,
    pub max_value: f64,
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
