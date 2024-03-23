use std::fmt::{
    Debug,
    Display,
    Formatter,
};

use crate::ffi::struct_CassVersion_;

/// The version of the connected Cassandra cluster.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ClusterVersion(struct_CassVersion_);

impl ClusterVersion {
    /// Creates a new `ClusterVersion` from the driver object.
    pub(crate) fn from_driver(value: struct_CassVersion_) -> Self {
        Self(value)
    }

    /// Returns the wrapped `struct_CassVersion_` value.
    pub(crate) fn inner(&self) -> &struct_CassVersion_ {
        &self.0
    }

    /// Returns the major version number.
    pub fn major(&self) -> usize {
        let n = self.inner().major_version;

        usize::try_from(n).unwrap_or(0)
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> usize {
        let n = self.inner().minor_version;

        usize::try_from(n).unwrap_or(0)
    }

    /// Returns the patch version number.
    pub fn patch(&self) -> usize {
        let n = self.inner().patch_version;

        usize::try_from(n).unwrap_or(0)
    }
}

impl Debug for ClusterVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CassVersion")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .field("patch", &self.patch())
            .finish()
    }
}

impl Display for ClusterVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}
