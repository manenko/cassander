use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `int` type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CqlInt(i32);

impl CqlInt {
    /// Creates a new [`CqlInt`] from the given [`i32`] value.
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the inner [`i32`] value.
    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl From<i32> for CqlInt {
    /// Converts the given [`i32`] into a [`CqlInt`].
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<CqlInt> for i32 {
    /// Converts the given [`CqlInt`] into an [`i32`].
    fn from(value: CqlInt) -> Self {
        value.as_i32()
    }
}

impl AsRef<i32> for CqlInt {
    /// Returns the inner [`i32`] value as a reference.
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl Display for CqlInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
