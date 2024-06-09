//! Module with constraint configs to apply to inputs
//! 
//! This module provides various constraints that can be applied to user inputs,
//! including constraints for strings and numbers. These constraints help ensure that
//! input data meets specific criteria before it is accepted.
//! 
//! # Example
//! 
//! ```
//! use constrained_inputs::constraints::{StringConstraint, NumberConstraint, Constraint};
//! 
//! fn main() {
//!     let string_constraint = StringConstraint {
//!         max_len: Some(10),
//!         min_len: Some(5),
//!         blacklist_chars: vec!['a', 'e', 'i', 'o', 'u'],
//!     };
//! 
//!     let result = string_constraint.validate(&"hello");
//!     assert_eq!(result, ConstraintResult::Err(ConstraintError::BlacklistedChar));
//! 
//!     let number_constraint = NumberConstraint {
//!         max: Some(100),
//!         min: Some(10),
//!     };
//! 
//!     let result = number_constraint.validate(&50);
//!     assert_eq!(result, ConstraintResult::Valid);
//! }
//! ```

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

/// Implementing method to apply constraints on strings
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
    pub max: Option<i32>,
    pub min: Option<i32>,
}

/// Implementing method to apply constraints on numbers
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

        if let Some(max) = self.max {
            if data > max {
                return ConstraintResult::Err(ConstraintError::TooLarge);
            }
        }
        if let Some(min) = self.min {
            if data < min {
                return ConstraintResult::Err(ConstraintError::TooSmall);
            }
        }

        ConstraintResult::Valid
    }
}
