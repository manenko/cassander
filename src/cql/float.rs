use core::fmt;
use std::fmt::{
    Display,
    Formatter,
};

/// A CQL `float` type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CqlFloat(f32);

impl CqlFloat {
    /// Creates a new [`CqlFloat`] from the given [`f32`] value.
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    /// Returns the inner [`f32`] value.
    pub fn as_f32(&self) -> f32 {
        self.0
    }
}

impl From<f32> for CqlFloat {
    /// Converts the given [`f32`] into a [`CqlFloat`].
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<CqlFloat> for f32 {
    /// Converts the given [`CqlFloat`] into an [`f32`].
    fn from(value: CqlFloat) -> Self {
        value.as_f32()
    }
}

impl AsRef<f32> for CqlFloat {
    /// Returns the inner [`f32`] value as a reference.
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

impl Display for CqlFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
