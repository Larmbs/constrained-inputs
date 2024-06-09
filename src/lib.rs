//! Crate for easy input parsing and prompting
//! This allows for easy input constraints

use lazy_static::lazy_static;
use std::io;

// Creating a single Stdin object
lazy_static! {
    static ref IO_IN: io::Stdin = io::stdin();
}

/// Error type that accounts for both io and parsing errors
#[derive(Debug)]
pub enum PromptError<T>
where
    T: std::str::FromStr,
{
    IoError(io::Error),
    ParseError(T::Err),
    BreaksConstraint(ConstraintError),
}

/// Prompts a typed input from the terminal
pub fn prompt_input<T>() -> Result<T, PromptError<T>>
where
    T: std::str::FromStr,
{
    let mut buf = String::new();
    IO_IN
        .read_line(&mut buf)
        .map_err(|io_err| PromptError::IoError(io_err))?;
    let input = buf.trim();
    input
        .parse::<T>()
        .map_err(|parse_err| PromptError::ParseError(parse_err))
}

/// Prompts user input in terminal with an added constraint
pub fn prompt_string_input_with_constraint<T, C>(constraint: C) -> Result<T, PromptError<T>>
where
    T: std::str::FromStr,
    C: Constraint<T>,
{
    let input = prompt_input()?;
    match constraint.validate(&input) {
        ConstraintResult::Valid => Ok(input),
        ConstraintResult::Err(err) => Err(PromptError::BreaksConstraint(err)),
    }
}

/// Result type for constraints
#[derive(Debug)]
pub enum ConstraintResult {
    Valid,
    Err(ConstraintError),
}

/// Error types for constraints
#[derive(Debug)]
pub enum ConstraintError {
    InvalidConstraint,
    TooLarge,
    TooSmall,
    BlacklistedChar,
    TooLong,
    TooShort,
}
/// Constraint trait
pub trait Constraint<T> {
    /// Validates data to make sure it follows constraints
    fn validate(&self, data: &T) -> ConstraintResult;
}
/// String constraint config for applying constraints to what a string can be
pub struct StringConstraint {
    max_len: Option<usize>,
    min_len: Option<usize>,
    blacklist_chars: Vec<char>,
}
/// Implementing method to apply constraints
impl<T> Constraint<T> for StringConstraint
where
    T: AsRef<str>,
{
    fn validate(&self, data: &T) -> ConstraintResult {
        let data = data.as_ref();

        if let Some(max_len) = self.max_len {
            if data.len() > max_len {
                return ConstraintResult::Err(ConstraintError::TooLong);
            }
        }
        if let Some(min_len) = self.min_len {
            if data.len() < min_len {
                return ConstraintResult::Err(ConstraintError::TooShort);
            }
        }
        for ch in data.chars() {
            if self.blacklist_chars.contains(&ch) {
                return ConstraintResult::Err(ConstraintError::BlacklistedChar);
            }
        }

        ConstraintResult::Valid
    }
}

/// Number constraint config for applying constraints onto some number
pub struct NumberConstraint {
    max_size: Option<isize>,
    min_size: Option<isize>,
}
/// Applying constraint trait to determine if data is valid
impl<T> Constraint<T> for NumberConstraint
where
    T: Into<isize> + Clone + PartialOrd,
{
    fn validate(&self, data: &T) -> ConstraintResult {
        let data: isize = data.clone().into();

        if let Some(max_size) = self.max_size {
            if data > max_size {
                return ConstraintResult::Err(ConstraintError::TooLarge);
            }
        }
        if let Some(min_size) = self.min_size {
            if data < min_size {
                return ConstraintResult::Err(ConstraintError::TooSmall);
            }
        }

        ConstraintResult::Valid
    }
}
