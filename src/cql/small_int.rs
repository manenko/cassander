/// A CQL `smallint` type.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CqlSmallInt(i16);

impl CqlSmallInt {
    /// Creates a new [`CqlSmallInt`] from the given [`i16`] value.
    pub fn new(value: i16) -> Self {
        Self(value)
    }

    /// Returns the inner [`i16`] value.
    pub fn as_i16(&self) -> i16 {
        self.0
    }
}

impl From<i16> for CqlSmallInt {
    /// Converts the given [`i16`] into a [`CqlSmallInt`].
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

impl From<CqlSmallInt> for i16 {
    /// Converts the given [`CqlSmallInt`] into an [`i16`].
    fn from(value: CqlSmallInt) -> Self {
        value.as_i16()
    }
}

impl AsRef<i16> for CqlSmallInt {
    /// Returns the inner [`i16`] value as a reference.
    fn as_ref(&self) -> &i16 {
        &self.0
    }
}
