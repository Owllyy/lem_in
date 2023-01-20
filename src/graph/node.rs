use std::{str::FromStr, num::ParseIntError};

use super::{name, Name, NodeId};

#[derive(Debug)]
pub struct Node {
    pub name: Name,
    pub pos: Position,
    pub links: Vec<NodeId>,
}

#[derive(Debug)]
pub enum ParseError {
    MissingField,
    PositionParseError(PositionParseError),
    InvalidName(name::ParseError),
}

impl From<PositionParseError> for ParseError {
    fn from(error: PositionParseError) -> Self {
        Self::PositionParseError(error)
    }
}

impl From<name::ParseError> for ParseError {
    fn from(error: name::ParseError) -> Self {
        Self::InvalidName(error)
    }
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, pos) = s.split_once(' ')
            .ok_or(ParseError::MissingField)?;
        let pos = pos.parse()?;
        Ok(Node {
            name: name.parse()?,
            pos,
            links: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub enum PositionParseError {
    MissingField,
    ParseIntError(<usize as FromStr>::Err),
}

impl From<ParseIntError> for PositionParseError {
    fn from(error: <usize as FromStr>::Err) -> Self {
        Self::ParseIntError(error)
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Position {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(' ')
            .ok_or(PositionParseError::MissingField)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}
