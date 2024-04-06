use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `counter` type.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct CqlCounter(i64);

impl CqlCounter {
    /// Creates a new [`CqlCounter`] from the given counter value.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner counter value.
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CqlCounter {
    /// Converts the given [`i64`] into a [`CqlCounter`].
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<CqlCounter> for i64 {
    /// Converts the given [`CqlCounter`] into an [`i64`].
    fn from(value: CqlCounter) -> Self {
        value.as_i64()
    }
}

impl Display for CqlCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
