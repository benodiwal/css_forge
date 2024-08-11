use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CssParseError {
    InvalidSelector,
    InvalidDeclaration,
    InvalidValue,
    InvalidColor,
    UnexpectedToken(char),
    UnexpectedEOF,
    InvalidUnit
}

impl fmt::Display for CssParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CssParseError::InvalidSelector => write!(f, "Invalid CSS selector"),
            CssParseError::InvalidDeclaration => write!(f, "Invalid CSS declaration"),
            CssParseError::InvalidValue => write!(f, "Invalid CSS value"),
            CssParseError::UnexpectedToken(c) => write!(f, "Unexpected token: {}", c),
            CssParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            CssParseError::InvalidColor => write!(f, "Invalid color value"),
            CssParseError::InvalidUnit => write!(f, "Invalid unit"),
        }
    }
}

impl Error for CssParseError {}
