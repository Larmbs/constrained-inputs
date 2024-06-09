//! Crate for easy input parsing and prompting
//! This allows for easy input constraints

use std::io;
use lazy_static::lazy_static;

// Creating a single Stdin object
lazy_static! {
    static ref IO_IN: io::Stdin = io::stdin();
}

/// Error type that accounts for both io and parsing errors
#[derive(Debug)]
pub enum PromptError<T> where T: std::str::FromStr {
    IoError(io::Error),
    ParseError(T::Err),
    BreaksConstraint(ConstraintError),
}

/// Prompts a typed input from the terminal
pub fn prompt_input<T>() -> Result<T, PromptError<T>>  where T: std::str::FromStr {
    let mut buf = String::new();
    IO_IN.read_line(&mut buf).map_err(|io_err| PromptError::IoError(io_err))?;
    let input = buf.trim();
    input.parse::<T>().map_err(|parse_err| PromptError::ParseError(parse_err))
}

/// Prompts user input in terminal with an added constraint

/// Error type for constraints
#[derive(Debug)]
pub enum ConstraintError {
    ToHigh,
    ToLow,
    Blacklisted,
    ToLong,
    ToShort,
}
/// Constraint trait
pub trait Constraint {
    fn valid(&self, T) -> ConstraintError;
}
/// String constraint config for applying constraints to what a string can be
pub struct StringConstraint {
    max_len: Option<usize>,
    min_len: Option<usize>,
    blacklist_chars: Vec<char>,
}
impl Constraint for StringConstraint {
    fn valid(&self) -> ConstraintError {
        todo!()
    }
}
/// Number constraint config for applying constraints onto some number
pub struct NumberConstraint {
    max_size: Option<isize>,
    min_size: Option<isize>,
}
