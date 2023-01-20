use super::{NodeId, name, Name};

use core::fmt;
use std::{str::FromStr, error::Error};

#[derive(PartialEq, Eq)]
pub struct LinkByName {
    pub a: Name,
    pub b: Name,
}

#[derive(Debug)]
pub enum ParseError {
    MissingField,
    InvalidName(name::ParseError)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingField => write!(f, "Missing field"),
            ParseError::InvalidName(name_error) => write!(f, "Invalid name: {name_error}"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseError::InvalidName(ref invalid_name_error) => Some(invalid_name_error),
            _ => None,
        }
    }
}

impl From<name::ParseError> for ParseError {
    fn from(error: name::ParseError) -> Self {
        Self::InvalidName(error)
    }
}

impl FromStr for LinkByName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-')
            .ok_or(ParseError::MissingField)?;
        Ok(Self {
            a: a.parse()?,
            b: b.parse()?,
        })
    }
}