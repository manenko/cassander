use std::fmt::{
    Debug,
    Display,
    Formatter,
};

use crate::driver::ffi::struct_CassVersion_;

/// The version of the connected Cassandra cluster.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CassVersion(struct_CassVersion_);

impl CassVersion {
    /// Wraps a raw `struct_CassVersion_` value.
    pub fn new(value: struct_CassVersion_) -> Self {
        Self(value)
    }

    /// Returns the wrapped `struct_CassVersion_` value.
    pub fn as_raw(&self) -> struct_CassVersion_ {
        self.0
    }

    /// Returns the major version number.
    pub fn major(&self) -> usize {
        let n = self.as_raw().major_version;

        usize::try_from(n).unwrap_or(0)
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> usize {
        let n = self.as_raw().minor_version;

        usize::try_from(n).unwrap_or(0)
    }

    /// Returns the patch version number.
    pub fn patch(&self) -> usize {
        let n = self.as_raw().patch_version;

        usize::try_from(n).unwrap_or(0)
    }
}

impl Debug for CassVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CassVersion")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .finish()
    }
}

impl Display for CassVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}
