use std::ffi::c_char;
use std::fmt::{
    self,
    Debug,
    Display,
    Formatter,
};
use std::slice;

use crate::ffi::{
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
    enum_cass_bool_t_cass_false as CASS_FALSE,
    struct_CassErrorResult_,
};
use crate::{
    Consistency,
    DriverErrorKind,
    WriteType,
};

/// Additional details about a driver error.
///
/// It is available for server errors only.
#[repr(transparent)]
pub struct DriverErrorDetails(CassErrorResult);

impl DriverErrorDetails {
    /// Creates a `DriverErrorDetails` from the driver object.
    ///
    /// Returns `None` if the driver object is null.
    pub(crate) fn from_driver(
        error: *const struct_CassErrorResult_,
    ) -> Option<Self> {
        if error.is_null() {
            None
        } else {
            Some(Self(CassErrorResult::from_driver(error)))
        }
    }

    /// Returns the raw pointer to the driver's error result object.
    pub(crate) fn inner(&self) -> *const struct_CassErrorResult_ {
        self.0.inner()
    }

    /// Returns an error code for this error result.
    ///
    /// This error code will always have an server error source
    /// (`DriverErrorKind::Server*`).
    ///
    /// The method is internal because the error code is available through the
    /// [`DriverError`] struct and duplication it here would make API more
    /// confusing.
    pub(crate) fn code(&self) -> DriverErrorKind {
        let code = unsafe { cass_error_result_code(self.inner()) };

        DriverErrorKind::from_driver(code)
            .expect("unexpected success error code")
    }

    /// Returns the consistency level.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerReadTimeout`]
    /// - [`DriverErrorKind::ServerWriteTimeout`]
    /// - [`DriverErrorKind::ServerReadFailure`]
    /// - [`DriverErrorKind::ServerWriteFailure`]
    /// - [`DriverErrorKind::ServerUnavailable`]
    pub fn consistency(&self) -> Option<Consistency> {
        let consistency =
            unsafe { cass_error_result_consistency(self.inner()) };

        Consistency::from_driver(consistency)
    }

    /// Returns the actual number of received responses, received
    /// acknowledgments or alive nodes.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerReadTimeout`]
    /// - [`DriverErrorKind::ServerWriteTimeout`]
    /// - [`DriverErrorKind::ServerReadFailure`]
    /// - [`DriverErrorKind::ServerWriteFailure`]
    /// - [`DriverErrorKind::ServerUnavailable`]
    pub fn responses_received(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_responses_received(self.inner()) };

        usize::try_from(n).ok()
    }

    /// Returns required responses, required acknowledgments or required alive
    /// nodes needed to successfully complete the request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerReadTimeout`]
    /// - [`DriverErrorKind::ServerWriteTimeout`]
    /// - [`DriverErrorKind::ServerReadFailure`]
    /// - [`DriverErrorKind::ServerWriteFailure`]
    /// - [`DriverErrorKind::ServerUnavailable`]
    pub fn responses_required(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_responses_required(self.inner()) };

        usize::try_from(n).ok()
    }

    /// Returns the number of nodes that experienced a failure while attempting
    /// to satisfy the request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerReadFailure`]
    /// - [`DriverErrorKind::ServerWriteFailure`]
    pub fn failures_count(&self) -> Option<usize> {
        let n = unsafe { cass_error_result_num_failures(self.inner()) };

        usize::try_from(n).ok()
    }

    /// Returns `true` if the actual data was present in the responses from the
    /// replicas.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerReadTimeout`]
    /// - [`DriverErrorKind::ServerReadFailure`]
    pub fn is_data_present(&self) -> Option<bool> {
        if self.code() != DriverErrorKind::ServerReadTimeout
            && self.code() != DriverErrorKind::ServerReadFailure
        {
            return None;
        }

        let present = unsafe { cass_error_result_data_present(self.inner()) };

        Some(present != CASS_FALSE)
    }

    /// Returns the write type of a request.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerWriteTimeout`]
    /// - [`DriverErrorKind::ServerWriteFailure`]
    pub fn write_type(&self) -> Option<WriteType> {
        let write_type = unsafe { cass_error_result_write_type(self.inner()) };

        WriteType::from_driver(write_type)
    }

    /// Returns the affected keyspace.
    ///
    /// This is only available for the following error codes:
    ///
    /// - [`DriverErrorKind::ServerAlreadyExists`]
    /// - [`DriverErrorKind::ServerFunctionFailure`]
    pub fn keyspace(&self) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_keyspace(self.inner(), s, l)
        })
    }

    /// Returns the affected table.
    ///
    /// This is only available for the [`DriverErrorKind::ServerAlreadyExists`]
    /// error.
    pub fn table(&self) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_table(self.inner(), s, l)
        })
    }

    /// Returns the affected function name and its argument types.
    ///
    /// This is only available for the
    /// [`DriverErrorKind::ServerFunctionFailure`] error.
    pub fn function(&self) -> Option<FunctionErrorDetails> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_function(self.inner(), s, l)
        })
        .map(|name| FunctionErrorDetails::new(name, self.arg_types()))
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
        unsafe { cass_error_num_arg_types(self.inner()) }
    }

    /// Returns the argument type at the specified index for the function
    /// failure error.
    fn arg_type(&self, index: usize) -> Option<String> {
        get_string_lossy(|s, l| unsafe {
            cass_error_result_arg_type(self.inner(), index, s, l)
        })
    }
}

impl Display for DriverErrorDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Debug for DriverErrorDetails {
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

/// Details of a function error.
#[derive(Debug, Clone)]
pub struct FunctionErrorDetails {
    /// The name of the function.
    pub name:      String,
    /// The argument types of the function.
    pub arg_types: Vec<String>,
}

impl FunctionErrorDetails {
    /// Creates a new `FunctionErrorDetails` object.
    pub fn new(name: String, arg_types: Vec<String>) -> Self {
        Self {
            name,
            arg_types,
        }
    }
}

impl Display for FunctionErrorDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.arg_types.join(", "))
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

#[repr(transparent)]
struct CassErrorResult(*const struct_CassErrorResult_);

impl CassErrorResult {
    pub(crate) fn from_driver(error: *const struct_CassErrorResult_) -> Self {
        Self(error)
    }

    pub(crate) fn inner(&self) -> *const struct_CassErrorResult_ {
        self.0
    }
}

impl Drop for CassErrorResult {
    fn drop(&mut self) {
        unsafe { cass_error_result_free(self.0) }
    }
}

// TODO: Check if this is correct.
// The object is read-only and can be safely shared between threads.
unsafe impl Send for CassErrorResult {}
unsafe impl Sync for CassErrorResult {}
