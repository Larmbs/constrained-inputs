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
#[derive(Debug, PartialEq)]
pub enum ConstraintResult {
    Valid,
    Err(ConstraintError),
}

/// Error types for constraints
#[derive(Debug, PartialEq)]
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
    pub max_len: Option<usize>,
    pub min_len: Option<usize>,
    pub blacklist_chars: Vec<char>,
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
    pub max_size: Option<i32>,
    pub min_size: Option<i32>,
}
/// Applying constraint trait to determine if data is valid
impl<T> Constraint<T> for NumberConstraint
where
    T: Into<i32> + Clone + PartialOrd,
{
    fn validate(&self, data: &T) -> ConstraintResult {
        let res = i32::try_from(data.clone()).map_err(|_| ConstraintError::InvalidConstraint);
        if let Err(err) = res {
            return ConstraintResult::Err(err);
        }
        let data = res.unwrap();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_constraints_test1() {
        let string_constraint = StringConstraint {
            max_len: Some(20),
            min_len: Some(2),
            blacklist_chars: vec!['a', 'b']
        };

        let input = "Hello my name is dog"; // Should fail, has the letter A

        assert_ne!(ConstraintResult::Valid, string_constraint.validate(&input));

        let input = String::from("This is another sentence that is too long and has blacklisted chars");

        assert_ne!(ConstraintResult::Valid, string_constraint.validate(&input));
    }

    #[test]
    fn number_constraint() {
        let number_constraint = NumberConstraint {
            max_size: Some(20),
            min_size: Some(-10),
        };

        let input: u8 = 15;

        assert_eq!(ConstraintResult::Valid, number_constraint.validate(&input));

        let input: u16 = 50;

        assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));

        let input: i16 = 50;

        assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));
        
        let input: i32 = -1000;

        assert_ne!(ConstraintResult::Valid, number_constraint.validate(&input));
    }
}