use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `boolean` type.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct CqlBoolean(bool);

impl CqlBoolean {
    /// Creates a new [`CqlBoolean`] from the given boolean value.
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the inner boolean value.
    pub fn as_bool(&self) -> bool {
        self.0
    }
}

impl From<bool> for CqlBoolean {
    /// Converts the given boolean value into a [`CqlBoolean`].
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<CqlBoolean> for bool {
    /// Converts the given [`CqlBoolean`] into a boolean value.
    fn from(value: CqlBoolean) -> Self {
        value.as_bool()
    }
}

impl AsRef<bool> for CqlBoolean {
    /// Returns the inner boolean value as a reference.
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

impl Display for CqlBoolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
