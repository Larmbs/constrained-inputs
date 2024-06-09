//! Crate for easy input parsing and prompting.
//! 
//! This crate allows for easy input constraints, making it simpler to handle user input
//! with specific type constraints and validations.
//! 
//! # Example
//! 
//! ```
//! use constrained_inputs::input;
//! 
//! fn main() {
//!     let int = input::<i32>().expect("Input was invalid");
//!     println!("Your input integer: {}", int);
//! }
//! ```

use lazy_static::lazy_static;
use std::io;

mod constraints;
pub use constraints::*;

// Just a method to avoid creating multiple stdin objects
lazy_static! {
    static ref IO_IN: io::Stdin = io::stdin();
}

/// Error gotten either when input is invalid or some I/O error occurs
#[derive(Debug)]
pub enum InputError<T>
where
    T: std::str::FromStr,
{
    IoError(io::Error),
    ParseError(T::Err),
    BreaksConstraint(ConstraintError),
}

/// This function is able to take in an input with a type constraint.
/// 
/// # Example
/// 
/// ```
/// use constrained_inputs::input;
/// 
/// fn main() {
///     let int = input::<i32>().expect("Input was invalid");
///     println!("Your input integer: {}", int);
/// }
/// ```
/// 
/// # Errors
/// 
/// This function returns an `InputError` if the input is invalid, if a parsing error occurs,
/// or if the input does not meet the specified constraints.
pub fn input<T>() -> Result<T, InputError<T>>
where
    T: std::str::FromStr,
{
    input_stream(IO_IN.lock())
}

/// Reads an input from a `BufRead` reader.
/// 
/// # Errors
/// 
/// This function returns an `InputError` if the input is invalid, if a parsing error occurs,
/// or if the input does not meet the specified constraints.
pub fn input_stream<T, R>(mut reader: R) -> Result<T, InputError<T>>
where 
    R: io::BufRead,
    T: std::str::FromStr,
{
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .map_err(|io_err| InputError::IoError(io_err))?;
    let input = buf.trim();
    input
        .parse::<T>()
        .map_err(|parse_err| InputError::ParseError(parse_err))
}

/// Prompts user input in the terminal with an added type constraint.
/// 
/// # WARNING
/// Make sure that the constraint provided is of the same type as the one you expect.
/// 
/// # Example
/// 
/// ```
/// use constrained_inputs::{constrained_input, NumberConstraint};
/// 
/// fn main() {
///     let constraint = NumberConstraint{
///         max: Some(20), 
///         min: Some(10),
///     };
///     let int = constrained_input::<i32, _>(constraint).expect("Input was invalid or out of range");
///     println!("Your constrained input integer: {}", int);
/// }
/// ```
/// 
/// # Errors
/// 
/// This function returns an `InputError` if the input is invalid, if a parsing error occurs,
/// or if the input does not meet the specified constraints.
pub fn constrained_input<T, C>(constraint: C) -> Result<T, InputError<T>>
where
    T: std::str::FromStr,
    C: Constraint<T>,
{
    let input = input()?;
    match constraint.validate(&input) {
        ConstraintResult::Valid => Ok(input),
        ConstraintResult::Err(err) => Err(InputError::BreaksConstraint(err)),
    }
}

#[cfg(test)]
mod test;
