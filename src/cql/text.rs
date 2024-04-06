use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `text` or `varchar` type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CqlText(String);

impl CqlText {
    /// Creates a new [`CqlText`] from the given [`String`] value.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the inner [`String`] value as a slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the inner [`String`] value.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for CqlText {
    /// Converts the given [`String`] into a [`CqlText`].
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for CqlText {
    /// Converts the given [`&str`] into a [`CqlText`].
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<CqlText> for String {
    /// Converts the given [`CqlText`] into a [`String`].
    fn from(value: CqlText) -> Self {
        value.into_string()
    }
}

impl AsRef<str> for CqlText {
    /// Returns the inner [`String`] value as a reference.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for CqlText {
    /// Returns the inner [`String`] value as a reference.
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Display for CqlText {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
