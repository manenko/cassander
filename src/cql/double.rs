use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `double` type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CqlDouble(f64);

impl CqlDouble {
    /// Creates a new [`CqlDouble`] from the given [`f64`] value.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the inner [`f64`] value.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<f64> for CqlDouble {
    /// Converts the given [`f64`] into a [`CqlDouble`].
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<CqlDouble> for f64 {
    /// Converts the given [`CqlDouble`] into an [`f64`].
    fn from(value: CqlDouble) -> Self {
        value.as_f64()
    }
}

impl AsRef<f64> for CqlDouble {
    /// Returns the inner [`f64`] value as a reference.
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Display for CqlDouble {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
