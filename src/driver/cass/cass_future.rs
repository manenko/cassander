use crate::driver::cass::{
    CassBool,
    CassError,
};
use crate::driver::ffi::{
    cass_future_free,
    cass_future_ready,
    cass_future_wait,
    cass_future_wait_timed,
    struct_CassFuture_,
};

/// The future result of a DataStax C++ driver operation.
///
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed.
#[must_use]
pub struct CassFuture {
    /// The driver's future object.
    inner: *mut struct_CassFuture_,
}

impl CassFuture {
    /// Returns the raw pointer to the future object.
    pub fn as_raw(&self) -> *mut struct_CassFuture_ {
        self.inner
    }

    /// Checks whether the future has been completed.
    pub fn is_ready(&self) -> bool {
        CassBool::new(unsafe { cass_future_ready(self.as_raw()) }).into()
    }

    /// Waits for the future to be set with either a result or error.
    ///
    /// This will block the current thread.
    pub fn wait(&self) {
        unsafe { cass_future_wait(self.as_raw()) }
    }

    /// The same as [`CassFuture::wait`] but timeouts after the given number of
    /// microseconds.
    ///
    /// Returns Ok(`false`) if returned due to timeout.
    ///
    /// Returns an error if the `timeout` overflows.
    pub fn wait_with_timeout(&self, timeout: i64) -> Result<bool, CassError> {
        let timeout =
            timeout.try_into().map_err(|_| CassError::LibBadParams)?;
        let completed =
            unsafe { cass_future_wait_timed(self.as_raw(), timeout) };

        Ok(CassBool::new(completed).into())
    }
}

impl Drop for CassFuture {
    /// Frees the future instance.
    ///
    /// A future can be freed anytime.
    fn drop(&mut self) {
        unsafe { cass_future_free(self.as_raw()) };
    }
}

unsafe impl Send for CassFuture {}
unsafe impl Sync for CassFuture {}
