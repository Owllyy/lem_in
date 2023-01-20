use core::fmt;
use std::str::FromStr;

/// A valid node name
/// See [`is_invalid_name_char`]
#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug)]
pub enum ParseError {
    InvalidCharacter(char),
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidCharacter(c) => writeln!(f, "Invalid character '{c}'"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Invalid characters are `'-'` `' '` & `'#'`
/// ```
/// # use lem_in::is_invalid_name_char;
/// assert_eq!(is_invalid_name_char(&'a'), false);
/// assert_eq!(is_invalid_name_char(&'-'), true);
/// assert_eq!(is_invalid_name_char(&'#'), true);
/// ```
pub fn is_invalid_name_char(c: &char) -> bool {
    ['-', ' ', '#'].contains(c)
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().find(is_invalid_name_char) {
            return Err(ParseError::InvalidCharacter(c));
        }
        Ok(Self(s.to_owned()))
    }
}
