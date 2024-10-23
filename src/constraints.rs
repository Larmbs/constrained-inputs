//! Constraint trait definition
use crate::error::Result;

/// Trait which represents a object which can verify and validate data
pub trait Constraint<T> {
    fn validate(&self, data: &T) -> Result<()>;
}
