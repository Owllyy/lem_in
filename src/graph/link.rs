use super::{Id, name, Name};

use std::str::FromStr;

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

pub struct Link {
    pub a: Id,
    pub b: Id,
}

