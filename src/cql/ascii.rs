use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};
use std::str::FromStr;

use thiserror::Error;

/// An error that occurs when the given string is not a valid ASCII string.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("the given string is not a valid ASCII string: '{0}'")]
pub struct NonAsciiStringError(String);

impl NonAsciiStringError {
    /// Creates a new error with the given string which is invalid ASCII string.
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(value.into())
    }
}

/// A CQL ASCII string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CqlAscii(String);

impl CqlAscii {
    /// Creates a new `CqlAscii` from the given string.
    ///
    /// Returns an error if the given string is not a valid ASCII string.
    pub fn new<S>(value: S) -> Result<Self, NonAsciiStringError>
    where
        S: Into<String>,
    {
        let value = value.into();
        if value.is_ascii() {
            Ok(Self(value))
        } else {
            Err(NonAsciiStringError::new(value))
        }
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CqlAscii {
    /// Returns the inner string as a reference.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<String> for CqlAscii {
    /// Returns the inner string as a reference.
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl FromStr for CqlAscii {
    type Err = NonAsciiStringError;

    /// Parses a string into a `CqlAscii`.
    ///
    /// Retrun
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<String> for CqlAscii {
    type Error = NonAsciiStringError;

    /// Converts a string into a `CqlAscii`.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for CqlAscii {
    type Error = NonAsciiStringError;

    /// Converts a string into a `CqlAscii`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&String> for CqlAscii {
    type Error = NonAsciiStringError;

    /// Converts a string into a `CqlAscii`.
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<CqlAscii> for String {
    /// Converts a `CqlAscii` into a string.
    fn from(value: CqlAscii) -> Self {
        value.0
    }
}

impl Display for CqlAscii {
    /// Formats the `CqlAscii` as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
