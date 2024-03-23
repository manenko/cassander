use std::ffi::CStr;
use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::ffi::{
    cass_uuid_from_string_n,
    cass_uuid_max_from_time,
    cass_uuid_min_from_time,
    cass_uuid_string,
    cass_uuid_timestamp,
    cass_uuid_version,
    struct_CassUuid_,
};
use crate::{
    to_result,
    DriverError,
    DriverErrorKind,
};

/// A UUID version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CqlUuidVersion {
    /// Time-based version 1 UUID.
    V1,
    /// Randomly generated version 4 UUID.
    V4,
    /// The version is unknown or invalid.
    Other(u8),
}

/// Version 1 (time-based) or version 4 (random) UUID.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct CqlUuid(struct_CassUuid_);

impl CqlUuid {
    /// Creates a new UUID from the given components.
    ///
    /// The `time_and_version` parameter encodes the time and version part of
    /// the UUID. The most significant 4 bits represent the version and the
    /// bottom 60 bits representing the time part. For version 1 the time part
    /// represents the number of 100 nanosecond periods since 00:00:00 UTC,
    /// January 1, 1970 (the Epoch). For version 4 the time part is randomly
    /// generated.
    ///
    /// The `clock_seq_and_node` parameter encodes the clock sequence and node
    /// part of the UUID. The most significant 16 bits represent the clock
    /// sequence (except for the most significant bit which is always set) and
    /// the bottom 48 bits represent the node part. For version 1 (time-based)
    /// the clock sequence part is randomly generated and the node part can be
    /// explicitly set, otherwise, it's generated from node unique information.
    /// For version 4 both the clock sequence and the node parts are randomly
    /// generated.
    pub fn from_components(
        time_and_version: u64,
        clock_seq_and_node: u64,
    ) -> Self {
        Self(struct_CassUuid_ {
            time_and_version,
            clock_seq_and_node,
        })
    }

    /// Creates a new `Uuid` from the given driver object.
    pub(crate) fn from_driver(value: struct_CassUuid_) -> Self {
        Self(value)
    }

    /// Sets the UUID to the minimum V1 value for the specified timestamp.
    ///
    /// The `timestamp` is in milliseconds since the Unix epoch (1970-01-01).
    pub fn min_from_time(timestamp: u64) -> Self {
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_min_from_time(timestamp, &mut uuid) };

        Self(uuid)
    }

    /// Sets the UUID to the maximum V1 value for the specified timestamp.
    ///
    /// The `timestamp` is in milliseconds since the Unix epoch (1970-01-01).
    pub fn max_from_time(timestamp: u64) -> Self {
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_max_from_time(timestamp, &mut uuid) };

        Self(uuid)
    }

    /// Returns the underlying [`struct_CassUuid_`] driver object.
    pub(crate) fn inner(&self) -> struct_CassUuid_ {
        self.0
    }

    /// Returns the time and version part of a UUID.
    ///
    /// The most significant 4 bits represent the version and the bottom 60 bits
    /// representing the time part. For version 1 the time part represents the
    /// number of 100 nanosecond periods since 00:00:00 UTC, January 1, 1970
    /// (the Epoch). For version 4 the time part is randomly generated.
    pub fn time_and_version(&self) -> u64 {
        self.inner().time_and_version
    }

    /// Returns the clock sequence and node part of a UUID.
    ///
    /// The most significant 16 bits represent the clock sequence (except for
    /// the most significant bit which is always set) and the bottom 48 bits
    /// represent the node part. For version 1 (time-based) the clock sequence
    /// part is randomly generated and the node part can be explicitly set,
    /// otherwise, it's generated from node unique information. For version 4
    /// both the clock sequence and the node parts are randomly generated.
    pub fn clock_seq_and_node(&self) -> u64 {
        self.inner().clock_seq_and_node
    }

    /// Returns timestamp for a V1 UUID.
    ///
    /// Returns `None` if the UUID is not a V1 UUID.
    pub fn timestamp(&self) -> Option<u64> {
        if self.version() == CqlUuidVersion::V1 {
            let timestamp = unsafe { cass_uuid_timestamp(self.inner()) };

            Some(timestamp)
        } else {
            None
        }
    }

    /// Return the version of this UUID.
    pub fn version(&self) -> CqlUuidVersion {
        match unsafe { cass_uuid_version(self.inner()) } {
            1 => CqlUuidVersion::V1,
            4 => CqlUuidVersion::V4,
            v => CqlUuidVersion::Other(v),
        }
    }
}

impl Display for CqlUuid {
    /// Formats the UUID as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = [0u8; 37];
        unsafe {
            cass_uuid_string(self.inner(), output.as_mut_ptr() as *mut _);
        }

        let uuid_str = CStr::from_bytes_with_nul(&output)
            .map_err(|_| std::fmt::Error)?
            .to_str()
            .map_err(|_| std::fmt::Error)?;

        write!(f, "{}", uuid_str)
    }
}

impl PartialEq for CqlUuid {
    /// Compares two UUIDs for equality.
    fn eq(&self, other: &Self) -> bool {
        self.time_and_version() == other.time_and_version()
            && self.clock_seq_and_node() == other.clock_seq_and_node()
    }
}

impl Eq for CqlUuid {}

impl Ord for CqlUuid {
    /// Compares two UUIDs.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time_and_version()
            .cmp(&other.time_and_version())
            .then_with(|| {
                self.clock_seq_and_node().cmp(&other.clock_seq_and_node())
            })
    }
}

impl PartialOrd for CqlUuid {
    /// Compares two UUIDs.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for CqlUuid {
    type Err = DriverError;

    /// Parses a UUID from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        let cstr = std::ffi::CString::new(s).map_err(|_| {
            DriverError::with_kind(DriverErrorKind::LibBadParams)
        })?;
        let str_length = cstr.as_bytes().len();
        let code = unsafe {
            cass_uuid_from_string_n(cstr.as_ptr(), str_length, &mut uuid)
        };

        to_result::<()>(code).map(|_| Self(uuid))
    }
}

impl From<struct_CassUuid_> for CqlUuid {
    /// Converts a driver UUID to a Rust UUID.
    fn from(value: struct_CassUuid_) -> Self {
        Self(value)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for CqlUuid {
    fn from(value: uuid::Uuid) -> Self {
        let input = value.as_bytes();

        let mut time_and_version = 0u64;
        time_and_version |= input[3] as u64;
        time_and_version |= (input[2] as u64) << 8;
        time_and_version |= (input[1] as u64) << 16;
        time_and_version |= (input[0] as u64) << 24;

        time_and_version |= (input[5] as u64) << 32;
        time_and_version |= (input[4] as u64) << 40;

        time_and_version |= (input[7] as u64) << 48;
        time_and_version |= (input[6] as u64) << 56;

        let mut clock_seq_and_node = 0u64;
        for i in 0..8 {
            clock_seq_and_node |= (input[15 - i] as u64) << (8 * i);
        }

        let uuid = struct_CassUuid_ {
            time_and_version,
            clock_seq_and_node,
        };

        Self(uuid)
    }
}

#[cfg(feature = "uuid")]
impl From<CqlUuid> for uuid::Uuid {
    fn from(value: CqlUuid) -> Self {
        let mut output = [0u8; 16];
        let value = value.inner();

        output[3] = value.time_and_version as u8;
        output[2] = (value.time_and_version >> 8) as u8;
        output[1] = (value.time_and_version >> 16) as u8;
        output[0] = (value.time_and_version >> 24) as u8;

        output[5] = (value.time_and_version >> 32) as u8;
        output[4] = (value.time_and_version >> 40) as u8;

        output[7] = (value.time_and_version >> 48) as u8;
        output[6] = (value.time_and_version >> 56) as u8;

        for i in 0..8 {
            output[15 - i] = (value.clock_seq_and_node >> (8 * i)) as u8;
        }

        uuid::Uuid::from_bytes(output)
    }
}
