use std::ffi::CStr;
use std::fmt::{
    Debug,
    Display,
    Formatter,
};
use std::str::FromStr;

use crate::driver::cass::CassError;
use crate::driver::ffi::{
    cass_uuid_from_string_n,
    cass_uuid_max_from_time,
    cass_uuid_min_from_time,
    cass_uuid_string,
    cass_uuid_timestamp,
    cass_uuid_version,
    struct_CassUuid_,
};

/// A UUID version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CassUuidVersion {
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
pub struct CassUuid(struct_CassUuid_);

impl CassUuid {
    /// The maximum allowed timestamp for a V1 UUID.
    ///
    /// The driver stores the time part as the number of 100 nanosecond periods.
    /// It expects that timestamps are measured in milliseconds and converts the
    /// timestamp into the 100 nanosecond periodis via
    /// `timestamp_ms * 1_000_000ns / 100`. This means the maximum allowed
    /// timestamp value, which does not overflow during this conversion, is
    /// `0x0FFFFFFFFFFFFFFF / 10_000 = 0x68DB8BAC710C` milliseconds.
    pub const MAX_TIMESTAMP_MS: i64 = 0x68DB8BAC710C;

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

    /// Sets the UUID to the minimum V1 value for the specified timestamp.
    ///
    /// The `timestamp` is in milliseconds since the Unix epoch (1970-01-01).
    ///
    /// Returns `None` if the `timestamp` is negative or larger than
    /// the [`CassUuid::MAX_TIMESTAMP_MS`].
    pub fn min_from_time(timestamp: i64) -> Option<Self> {
        let timestamp = to_driver_timestamp(timestamp)?;
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_min_from_time(timestamp, &mut uuid) };

        Some(Self(uuid))
    }

    /// Sets the UUID to the maximum V1 value for the specified timestamp.
    ///
    /// The `timestamp` is in milliseconds since the Unix epoch (1970-01-01).
    ///
    /// Returns `None` if the `timestamp` is negative or larger than
    /// the [`CassUuid::MAX_TIMESTAMP_MS`].
    pub fn max_from_time(timestamp: i64) -> Option<Self> {
        let timestamp = to_driver_timestamp(timestamp)?;
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_max_from_time(timestamp, &mut uuid) };

        Some(Self(uuid))
    }

    /// Returns the underlying [`struct_CassUuid_`] driver object.
    pub fn as_raw(&self) -> struct_CassUuid_ {
        self.0
    }

    /// Returns the time and version part of a UUID.
    ///
    /// The most significant 4 bits represent the version and the bottom 60 bits
    /// representing the time part. For version 1 the time part represents the
    /// number of 100 nanosecond periods since 00:00:00 UTC, January 1, 1970
    /// (the Epoch). For version 4 the time part is randomly generated.
    pub fn time_and_version(&self) -> u64 {
        self.as_raw().time_and_version
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
        self.as_raw().clock_seq_and_node
    }

    /// Returns timestamp for a V1 UUID.
    ///
    /// Returns `None` if the UUID is not a V1 UUID or if the timestamp is too
    /// large to fit in an `i64` value.
    pub fn timestamp(&self) -> Option<i64> {
        let timestamp = unsafe { cass_uuid_timestamp(self.as_raw()) }
            .try_into()
            .ok();

        // The driver returns 0 if the UUID is not a V1 UUID.
        match timestamp {
            Some(timestamp) if timestamp > 0 => Some(timestamp),
            Some(_) => None,
            None => None,
        }
    }

    /// Return the version of this UUID.
    pub fn version(&self) -> CassUuidVersion {
        match unsafe { cass_uuid_version(self.as_raw()) } {
            1 => CassUuidVersion::V1,
            4 => CassUuidVersion::V4,
            v => CassUuidVersion::Other(v),
        }
    }
}

impl Display for CassUuid {
    /// Formats the UUID as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = [0u8; 37];
        unsafe {
            cass_uuid_string(self.as_raw(), output.as_mut_ptr() as *mut _);
        }

        let uuid_str = CStr::from_bytes_with_nul(&output)
            .map_err(|_| std::fmt::Error)?
            .to_str()
            .map_err(|_| std::fmt::Error)?;

        write!(f, "{}", uuid_str)
    }
}

impl PartialEq for CassUuid {
    /// Compares two UUIDs for equality.
    fn eq(&self, other: &Self) -> bool {
        self.time_and_version() == other.time_and_version()
            && self.clock_seq_and_node() == other.clock_seq_and_node()
    }
}

impl Eq for CassUuid {}

impl Ord for CassUuid {
    /// Compares two UUIDs.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time_and_version()
            .cmp(&other.time_and_version())
            .then_with(|| {
                self.clock_seq_and_node().cmp(&other.clock_seq_and_node())
            })
    }
}

impl PartialOrd for CassUuid {
    /// Compares two UUIDs.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for CassUuid {
    type Err = ();

    /// Parses a UUID from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        let cstr = std::ffi::CString::new(s).map_err(|_| ())?;
        let str_length = cstr.as_bytes().len();
        let code: CassError = unsafe {
            cass_uuid_from_string_n(cstr.as_ptr(), str_length, &mut uuid)
        }
        .into();

        if code.is_ok() {
            Ok(Self(uuid))
        } else {
            Err(())
        }
    }
}

impl From<struct_CassUuid_> for CassUuid {
    /// Converts a driver UUID to a Rust UUID.
    fn from(value: struct_CassUuid_) -> Self {
        Self(value)
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for CassUuid {
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
impl From<CassUuid> for uuid::Uuid {
    fn from(value: CassUuid) -> Self {
        let mut output = [0u8; 16];
        let value = value.as_raw();

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

/// Converts a signed Unix timestamp in milliseconds to a driver timestamp
/// taking into account the value overflow.
pub(crate) fn to_driver_timestamp(timestamp: i64) -> Option<u64> {
    if !(0..=CassUuid::MAX_TIMESTAMP_MS).contains(&timestamp) {
        None
    } else {
        Some(timestamp as u64)
    }
}
