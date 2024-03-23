use std::ffi::CStr;
use std::fmt::{
    Display,
    Formatter,
};

use crate::ffi::{
    CASS_VERSION_MAJOR,
    CASS_VERSION_MINOR,
    CASS_VERSION_PATCH,
    CASS_VERSION_SUFFIX,
};

/// The version of the DataStax C++ driver for Apache Cassandra.
#[derive(Debug, Clone)]
pub struct DriverVersion {
    /// Major version number.
    pub major:  u32,
    /// Minor version number.
    pub minor:  u32,
    /// Patch version number.
    pub patch:  u32,
    /// Optional version suffix.
    pub suffix: Option<String>,
}

impl DriverVersion {
    /// Creates a new version.
    pub fn new(
        major: u32,
        minor: u32,
        patch: u32,
        suffix: Option<String>,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
            suffix,
        }
    }

    /// Returns the version of the DataStax C++ driver for Apache Cassandra this
    /// crate was build against.
    pub fn current() -> Self {
        Self::new(
            CASS_VERSION_MAJOR,
            CASS_VERSION_MINOR,
            CASS_VERSION_PATCH,
            Self::get_current_suffix(),
        )
    }

    fn get_current_suffix() -> Option<String> {
        let suffix = CStr::from_bytes_with_nul(CASS_VERSION_SUFFIX)
            .map(|cstr| cstr.to_string_lossy().into_owned())
            .unwrap_or_default();

        if suffix.is_empty() {
            None
        } else {
            Some(suffix)
        }
    }
}

impl Display for DriverVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(suffix) = &self.suffix {
            write!(f, "-{}", suffix)?;
        }

        Ok(())
    }
}
