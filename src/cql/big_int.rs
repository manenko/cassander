use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `bigint` type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CqlBigInt(i64);

impl CqlBigInt {
    /// Creates a new [`CqlBigInt`] from the given [`i64`] value.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner [`i64`] value.
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CqlBigInt {
    /// Converts the given [`i64`] into a [`CqlBigInt`].
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<CqlBigInt> for i64 {
    /// Converts the given [`CqlBigInt`] into an [`i64`].
    fn from(value: CqlBigInt) -> Self {
        value.as_i64()
    }
}

impl AsRef<i64> for CqlBigInt {
    /// Returns the inner [`i64`] value as a reference.
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl Display for CqlBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
