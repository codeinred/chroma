use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// Chroma Error Type - type for representing error that occurs during build process
#[derive(Debug)]
pub enum Error {
    /// Str error - an error that can be represented by a short static human-readable string
    Str(&'static str),
    /// Internal Error - error that represents a failure of the program for unknown reasons
    InternalErr(Box<dyn std::error::Error>),
}

impl<T: Into<Box<dyn std::error::Error>>> From<T> for Error {
    fn from(x: T) -> Error {
        Error::InternalErr(x.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Str(msg) => write!(f, "{}", msg),
            Error::InternalErr(msg) => write!(f, "{:?}", msg),
        }
    }
}

/// Take a string literal representing an error message and convert it into a chroma::Result type holding the corresponding error
pub fn report_err<T>(msg: &'static str) -> Result<T> {
    Err(Error::Str(msg))
}
