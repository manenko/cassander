use std::ffi::c_char;
use std::slice;
use std::fmt::{
    self,
    Debug,
    Formatter,
};

use crate::convert::MaybeInto;
use crate::driver::cass::{
    CassBool,
    CassConsistency,
    CassError,
    CassWriteType,
};
use crate::driver::ffi::{
    cass_error_num_arg_types,
    cass_error_result_arg_type,
    cass_error_result_code,
    cass_error_result_consistency,
    cass_error_result_data_present,
    cass_error_result_free,
    cass_error_result_function,
    cass_error_result_keyspace,
    cass_error_result_num_failures,
    cass_error_result_responses_received,
    cass_error_result_responses_required,
    cass_error_result_table,
    cass_error_result_write_type,
    enum_CassError_,
    enum_CassError__CASS_OK as CASS_OK,
    struct_CassErrorResult_,
};

/// An error result of a request.
#[repr(transparent)]
pub struct CassErrorResult(*const struct_CassErrorResult_);

impl CassErrorResult {
    /// Wraps a raw pointer to the driver's error result object.
    pub fn new(error: *const struct_CassErrorResult_) -> Option<Self> {
        if error.is_null() {
            None
        } else {
            Some(Self(error))
        }
    }

    /// Returns the raw pointer to the driver's error result object.
    pub fn as_raw(&self) -> *const struct_CassErrorResult_ {
        self.0
    }

    /// Returns an error code for this error result.
    ///
    /// This error code will always have an server error source
    /// (`CassError::Server*`).
    pub fn code(&self) -> CassError {
        unsafe { cass_error_result_code(self.as_raw()).into() }
    }

    /// Returns the consistency level.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerReadTimeout`]
    /// - [`CassError::ServerWriteTimeout`]
    /// - [`CassError::ServerReadFailure`]
    /// - [`CassError::ServerWriteFailure`]
    /// - [`CassError::ServerUnavailable`]
    pub fn consistency(&self) -> Option<CassConsistency> {
        unsafe { cass_error_result_consistency(self.as_raw()).maybe_into() }
    }

    /// Returns the actual number of received responses, received
    /// acknowledgments or alive nodes.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerReadTimeout`]
    /// - [`CassError::ServerWriteTimeout`]
    /// - [`CassError::ServerReadFailure`]
    /// - [`CassError::ServerWriteFailure`]
    /// - [`CassError::ServerUnavailable`]
    pub fn responses_received(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_responses_received(self.as_raw()) };

        usize::try_from(n).ok()
    }

    /// Returns required responses, required acknowledgments or required alive
    /// nodes needed to successfully complete the request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerReadTimeout`]
    /// - [`CassError::ServerWriteTimeout`]
    /// - [`CassError::ServerReadFailure`]
    /// - [`CassError::ServerWriteFailure`]
    /// - [`CassError::ServerUnavailable`]
    pub fn responses_required(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_responses_required(self.as_raw()) };

        usize::try_from(n).ok()
    }

    /// Returns the number of nodes that experienced a failure while attempting
    /// to satisfy the request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerReadFailure`]
    /// - [`CassError::ServerWriteFailure`]
    pub fn failures_count(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_num_failures(self.as_raw()) };

        usize::try_from(n).ok()
    }

    /// Returns `true` if the actual data was present in the responses from the
    /// replicas.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerReadTimeout`]
    /// - [`CassError::ServerReadFailure`]
    pub fn is_data_present(&self) -> Option<bool> {
        if self.code() != CassError::ServerReadTimeout
            && self.code() != CassError::ServerReadFailure
        {
            return None;
        }

        let b = unsafe { cass_error_result_data_present(self.as_raw()) };

        Some(CassBool::new(b).into())
    }

    /// Returns the write type of a request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerWriteTimeout`]
    /// - [`CassError::ServerWriteFailure`]
    pub fn write_type(&self) -> Option<CassWriteType> {
        unsafe { cass_error_result_write_type(self.as_raw()) }.maybe_into()
    }

    /// Returns the affected keyspace.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`CassError::ServerAlreadyExists`]
    /// - [`CassError::ServerFunctionFailure`]
    pub fn keyspace(&self) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_keyspace(self.as_raw(), s, l)
        })
    }

    /// Returns the affected table.
    ///
    /// This is only available for the [`CassError::ServerAlreadyExists`] error.
    pub fn table(&self) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_table(self.as_raw(), s, l)
        })
    }

    /// Returns the affected function name and its argument types.
    ///
    /// This is only available for the [`CassError::ServerFunctionFailure`]
    /// error.
    pub fn function(&self) -> Option<(String, Vec<String>)> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_function(self.as_raw(), s, l)
        })
        .map(|name| (name, self.arg_types()))
    }

    /// Returns the argument types for the function failure error.
    fn arg_types(&self) -> Vec<String> {
        (0..self.num_arg_types())
            .map(|index| {
                self.arg_type(index)
                    .unwrap_or_else(|| "<unknown>".to_string())
            })
            .collect()
    }

    /// Returns the number of argument types for the function failure error.
    fn num_arg_types(&self) -> usize {
        unsafe { cass_error_num_arg_types(self.as_raw()) }
    }

    /// Returns the argument type at the specified index for the function
    /// failure error.
    fn arg_type(&self, index: usize) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_arg_type(self.as_raw(), index, s, l)
        })
    }
}

impl Drop for CassErrorResult {
    fn drop(&mut self) {
        unsafe { cass_error_result_free(self.as_raw()) }
    }
}

impl Debug for CassErrorResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CassErrorResult")
            .field("code", &self.code())
            .field("consistency", &self.consistency())
            .field("responses_received", &self.responses_received())
            .field("responses_required", &self.responses_required())
            .field("failures_count", &self.failures_count())
            .field("is_data_present", &self.is_data_present())
            .field("write_type", &self.write_type())
            .field("keyspace", &self.keyspace())
            .field("table", &self.table())
            .field("function", &self.function())
            .finish()
    }
}

fn get_string_lossy<F>(getter: F) -> Option<String>
where
    F: Fn(*mut *const c_char, *mut usize) -> enum_CassError_,
{
    let mut string = std::ptr::null();
    let mut string_len = 0;
    if getter(&mut string, &mut string_len) != CASS_OK {
        // The driver returns an error when the getter is called on an error
        // that does not support this kind of information.
        return None;
    }

    // The driver returns a pointer to its internal data. We should not free it.
    let ptr = string as *const u8;
    let slice = unsafe { slice::from_raw_parts(ptr, string_len) };
    let string = String::from_utf8_lossy(slice);

    Some(string.into_owned())
}
