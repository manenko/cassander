use crate::driver::cass::{
    to_driver_timestamp,
    CassUuid,
};
use crate::driver::ffi::{
    cass_uuid_gen_free,
    cass_uuid_gen_from_time,
    cass_uuid_gen_new,
    cass_uuid_gen_new_with_node,
    cass_uuid_gen_random,
    cass_uuid_gen_time,
    struct_CassUuidGen_,
};

/// A UUID generator.
pub struct CassUuidGen(*mut struct_CassUuidGen_);

impl CassUuidGen {
    /// Creates a new UUID generator.
    pub fn new() -> Self {
        Self(unsafe { cass_uuid_gen_new() })
    }

    /// Creates a new UUID generator with custom node information.
    pub fn new_with_node(node: u64) -> Self {
        Self(unsafe { cass_uuid_gen_new_with_node(node) })
    }

    /// Returns the raw pointer to the underlying driver object.
    pub fn as_raw(&self) -> *mut struct_CassUuidGen_ {
        self.0
    }

    /// Generates a new V1 (time-based) UUID for the current time.
    pub fn generate_time_uuid(&self) -> CassUuid {
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_time(self.as_raw(), &mut uuid) };

        uuid.into()
    }

    /// Generates a new V4 (random) UUID.
    pub fn generate_random_uuid(&self) -> CassUuid {
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_random(self.as_raw(), &mut uuid) };

        uuid.into()
    }

    /// Generates a V1 (time) UUID for the specified timestamp.
    ///
    /// The `timestamp` is measured in milliseconds since the Unix epoch.
    ///
    /// The function returns `None` if the timestamp is negative or is too large
    /// to fit in an `i64` value.
    pub fn gerate_uuid_from_timestamp(
        &self,
        timestamp: i64,
    ) -> Option<CassUuid> {
        let timestamp = to_driver_timestamp(timestamp)?;
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_from_time(self.as_raw(), timestamp, &mut uuid) };

        Some(uuid.into())
    }
}

impl Drop for CassUuidGen {
    fn drop(&mut self) {
        unsafe { cass_uuid_gen_free(self.as_raw()) }
    }
}

impl Default for CassUuidGen {
    /// Creates a new UUID generator.
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for CassUuidGen {}
unsafe impl Send for CassUuidGen {}
