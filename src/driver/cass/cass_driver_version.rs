use std::ffi::CStr;
use std::fmt::{
    Display,
    Formatter,
};

use crate::driver::ffi::{
    CASS_VERSION_MAJOR,
    CASS_VERSION_MINOR,
    CASS_VERSION_PATCH,
    CASS_VERSION_SUFFIX,
};

/// The version of the DataStax C++ driver for Apache Cassandra.
#[derive(Debug, Clone)]
pub struct CassDriverVersion {
    /// Major version number.
    pub major:  u32,
    /// Minor version number.
    pub minor:  u32,
    /// Patch version number.
    pub patch:  u32,
    /// Optional version suffix.
    pub suffix: Option<String>,
}

impl CassDriverVersion {
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

    /// Returns the version of the DataStax C++ driver for Apache Cassandra.
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

impl Display for CassDriverVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(suffix) = &self.suffix {
            write!(f, "-{}", suffix)?;
        }

        Ok(())
    }
}
