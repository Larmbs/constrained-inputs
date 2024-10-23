//! Crate for easy import parsing and constraint applying
use core::str;
use lazy_static::lazy_static;
use std::io::{self, BufRead};

pub mod constraints;
pub mod error;
pub mod basic_constraints;
pub mod prelude;

use constraints::*;
use error::{Error, Result, ErrorKind};

lazy_static! {
    /// Constant reference to Stdin reader
    static ref IO_IN: io::Stdin = io::stdin();
}

/// Read from Stdin, string is parsed into T
pub fn input<T>() -> Result<T>
where
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
{
    read_stream(IO_IN.lock())
}

/// Read from Stdin, string is parsed into T, then a constraint is applied
pub fn cinput<T, C>(constraint: C) -> Result<T>
where
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
    C: Constraint<T>,
{
    let value = input()?;
    constraint.validate(&value)?;
    Ok(value)
}

/// String is parsed into T
pub fn string_input<T>(string: &String) -> Result<T>
where
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
{
    string.parse::<T>().map_err(|err| Error {
        kind: ErrorKind::IOError,
        message: err.to_string(),
    })
}

/// No no a CString, string is parsed into T, then a constraint is applied
pub fn cstring_input<T, C>(string: &String, constraint: C) -> Result<T>
where
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
    C: Constraint<T>,
{
    let value: T = string_input(string)?;
    constraint.validate(&value)?;
    Ok(value)
}

/// Read from a BufReader one line, string is parsed into T
pub fn read_stream<R, T>(mut reader: R) -> Result<T>
where
    R: BufRead,
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
{
    let mut buf = String::new();
    reader.read_line(&mut buf).map_err(|err| Error {
        kind: ErrorKind::IOError,
        message: err.to_string(),
    })?;

    string_input(&buf)
}

/// Read from a BufReader one line, string is parsed into T, then a constraint is applied
pub fn cread_stream<R, T, C>(mut reader: R, constraint: C) -> Result<T>
where
    R: BufRead,
    T: std::str::FromStr,
    <T as str::FromStr>::Err: std::fmt::Display,
    C: Constraint<T>,
{
    let mut buf = String::new();
    reader.read_line(&mut buf).map_err(|err| Error {
        kind: ErrorKind::IOError,
        message: err.to_string(),
    })?;

    cstring_input(&buf, constraint)
}
