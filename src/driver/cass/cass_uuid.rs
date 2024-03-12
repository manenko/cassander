use crate::driver::ffi::{
    cass_uuid_gen_from_time,
    cass_uuid_gen_random,
    cass_uuid_gen_time,
    cass_uuid_max_from_time,
    cass_uuid_min_from_time,
    cass_uuid_string,
    cass_uuid_timestamp,
    cass_uuid_version,
    struct_CassUuid_,
};

/// Version 1 (time-based) or version 4 (random) UUID.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct CassUuid(struct_CassUuid_);

impl CassUuid {
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

    /// Sets the UUID to the minimum V1 (time) value for the specified time.
    ///
    /// The `time` is in milliseconds since the Unix epoch (1970-01-01).
    ///
    /// Returns `None` if the `time` is negative.
    pub fn min_from_time(time: i64) -> Option<Self> {
        let time = time.try_into().ok()?;
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_min_from_time(time, &mut uuid) };

        Some(Self(uuid))
    }

    /// Sets the UUID to the maximum V1 (time) value for the specified time.
    ///
    /// The time is in milliseconds since the Unix epoch (1970-01-01).
    ///
    /// Returns `None` if the `time` is negative.
    pub fn max_from_time(time: i64) -> Option<Self> {
        let time = time.try_into().ok()?;
        let mut uuid: struct_CassUuid_ = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_max_from_time(time, &mut uuid) };
        Some(Self(uuid))
    }

    /// Returns the underlying [`struct_CassUuid_`] driver object.
    pub fn as_raw(&self) -> &struct_CassUuid_ {
        &self.0
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
