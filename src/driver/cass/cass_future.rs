use crate::driver::ffi::{
    cass_future_free,
    struct_CassFuture_,
};

/// The future result of a DataStax C++ driver operation.
///
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed.
#[must_use]
pub struct CassFuture {
    inner: *mut struct_CassFuture_,
}

impl CassFuture {
    /// Returns the raw pointer to the future object.
    pub fn as_raw(&self) -> *mut struct_CassFuture_ {
        self.inner
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
