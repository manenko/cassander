use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `tinyint` type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CqlTinyInt(i8);

impl CqlTinyInt {
    /// Creates a new [`CqlTinyInt`] from the given [`i8`] value.
    pub fn new(value: i8) -> Self {
        Self(value)
    }

    /// Returns the inner [`i8`] value.
    pub fn as_i8(&self) -> i8 {
        self.0
    }
}

impl From<i8> for CqlTinyInt {
    /// Converts the given [`i8`] into a [`CqlTinyInt`].
    fn from(value: i8) -> Self {
        Self::new(value)
    }
}

impl From<CqlTinyInt> for i8 {
    /// Converts the given [`CqlTinyInt`] into an [`i8`].
    fn from(value: CqlTinyInt) -> Self {
        value.as_i8()
    }
}

impl AsRef<i8> for CqlTinyInt {
    /// Returns the inner [`i8`] value as a reference.
    fn as_ref(&self) -> &i8 {
        &self.0
    }
}

impl Display for CqlTinyInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
