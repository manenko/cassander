use std::sync::Arc;

use crate::driver::ffi::{
    cass_timestamp_gen_free,
    cass_timestamp_gen_monotonic_new,
    cass_timestamp_gen_monotonic_new_with_settings,
    cass_timestamp_gen_server_side_new,
    struct_CassTimestampGen_,
};

/// A generator of client-side, microsecond-precision timestamps.
///
/// Objects of this type can be shared between threads and sessions.
#[derive(Clone)]
pub struct CassTimestampGen(Arc<CassTimestampGenWrapper>);

impl CassTimestampGen {
    /// Creates a new server-side timestamp generator.
    ///
    /// This generator allows Cassandra to assign timestamps server-side.
    ///
    /// This is the default timestamp generator.
    pub fn new() -> Self {
        Self::from_raw(unsafe { cass_timestamp_gen_server_side_new() })
    }

    fn from_raw(raw: *mut struct_CassTimestampGen_) -> Self {
        Self(Arc::new(CassTimestampGenWrapper(raw)))
    }

    /// Creates a new monotonically increasing timestamp generator with
    /// microsecond precision.
    ///
    /// This implementation guarantees a monotonically increasing timestamp. If
    /// the timestamp generation rate exceeds one per microsecond or if the
    /// clock skews into the past the generator will artificially increment the
    /// previously generated timestamp until the request rate decreases or the
    /// clock skew is corrected.
    ///
    /// By default, this timestamp generator will generate warnings if more than
    /// 1 second of clock skew is detected. It will print an error every second
    /// until the clock skew is resolved. These settings can be changed by using
    /// [`CassTimestampGen::monotonic_with_settings`] to create the generator
    /// instance.
    pub fn monotonic() -> Self {
        Self::from_raw(unsafe { cass_timestamp_gen_monotonic_new() })
    }

    /// Same as [`CassTimestampGen::monotonic`] but with settings for
    /// controlling warnings about clock skew.
    ///
    /// The `warning_threshold` parameter is the number of microseconds of clock
    /// skew that will trigger a warning. A threshold less than 0 can be used to
    /// disable warnings.
    ///
    /// The `update_interval` parameter is the amount of time, in milliseconds,
    /// to wait before warning again about clock skew. An interval value less
    /// than or equal to 0 allows the warning to be triggered every millisecond.
    pub fn monotonic_with_settings(
        warning_threshold: i64,
        update_interval: i64,
    ) -> Self {
        let raw = unsafe {
            cass_timestamp_gen_monotonic_new_with_settings(
                warning_threshold,
                update_interval,
            )
        };
        Self::from_raw(raw)
    }

    /// Returns the raw pointer to the underlying driver object.
    pub fn as_raw(&self) -> *mut struct_CassTimestampGen_ {
        self.0 .0
    }
}

impl Default for CassTimestampGen {
    /// Creates a new server-side timestamp generator.
    ///
    /// This generator allows Cassandra to assign timestamps server-side.
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
struct CassTimestampGenWrapper(*mut struct_CassTimestampGen_);

impl Drop for CassTimestampGenWrapper {
    fn drop(&mut self) {
        unsafe { cass_timestamp_gen_free(self.0) }
    }
}

unsafe impl Send for CassTimestampGenWrapper {}
unsafe impl Sync for CassTimestampGenWrapper {}
