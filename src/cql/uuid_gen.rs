use crate::cql::CqlUuid;
use crate::ffi::{
    cass_uuid_gen_free,
    cass_uuid_gen_from_time,
    cass_uuid_gen_new,
    cass_uuid_gen_new_with_node,
    cass_uuid_gen_random,
    cass_uuid_gen_time,
    struct_CassUuidGen_,
};

/// A UUID generator.
///
/// It is best practice to create and reuse a single object per application.
#[repr(transparent)]
pub struct CqlUuidGen(*mut struct_CassUuidGen_);

impl CqlUuidGen {
    /// Creates a new UUID generator.
    pub fn new() -> Self {
        Self(unsafe { cass_uuid_gen_new() })
    }

    /// Creates a new UUID generator with custom node information.
    pub fn new_with_node(node: u64) -> Self {
        Self(unsafe { cass_uuid_gen_new_with_node(node) })
    }

    /// Returns the raw pointer to the underlying driver object.
    pub(crate) fn inner(&self) -> *mut struct_CassUuidGen_ {
        self.0
    }

    /// Generates a new V1 (time-based) UUID for the current time.
    pub fn generate_time_uuid(&self) -> CqlUuid {
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_time(self.inner(), &mut uuid) };

        uuid.into()
    }

    /// Generates a new V4 (random) UUID.
    pub fn generate_random_uuid(&self) -> CqlUuid {
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_random(self.inner(), &mut uuid) };

        uuid.into()
    }

    /// Generates a V1 (time) UUID for the specified timestamp.
    ///
    /// The `timestamp` is measured in milliseconds since the Unix epoch.
    pub fn generate_uuid_from_timestamp(&self, timestamp: u64) -> CqlUuid {
        let mut uuid = unsafe { std::mem::zeroed() };
        unsafe { cass_uuid_gen_from_time(self.inner(), timestamp, &mut uuid) };

        uuid.into()
    }
}

impl Drop for CqlUuidGen {
    fn drop(&mut self) {
        unsafe { cass_uuid_gen_free(self.inner()) }
    }
}

impl Default for CqlUuidGen {
    /// Creates a new UUID generator.
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for CqlUuidGen {}
unsafe impl Send for CqlUuidGen {}
