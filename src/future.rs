use std::slice;

use crate::cql::CqlUuid;
use crate::ffi::{
    cass_future_error_code,
    cass_future_error_message,
    cass_future_free,
    cass_future_get_error_result,
    cass_future_ready,
    cass_future_tracing_id,
    cass_future_wait,
    cass_future_wait_timed,
    enum_cass_bool_t_cass_false as CASS_FALSE,
    struct_CassFuture_,
    struct_CassUuid_,
};
use crate::{
    to_result,
    DriverError,
    DriverErrorDetails,
    DriverErrorKind,
    Session,
};

// TODO: cass_future_get_prepared
// TODO: cass_future_custom_payload_item_count
// TODO: cass_future_custom_payload_item
// TODO: cass_future_coordinator

// The driver's future has a `cass_future_set_callback` function that allows
// setting a callback to be called when the future is set. This is what we use
// to implement the `Future` trait for the `CassFuture` type.
//
// -----------------------------------------------------------------------------
// Callbacks
// -----------------------------------------------------------------------------
//
// The callback is called with the future and the user data in the following
// cases:
// - When the future is set with a result or error.
// - When the future is set with a result or error and the future is already
//   ready.
//
// The second case requires a bit more of explanation:
// https://github.com/datastax/cpp-driver/blob/2.17.1/src/future.cpp#L176-L188.
// The driver uses a mutex to set a callback and checks if the future is ready.
// If the future is ready, the driver releases the mutex and calls the callback.
// In other words, when calling `cass_future_set_callback` the callback is
// called immediately if the future is already set.
//
// -----------------------------------------------------------------------------
// Future States
// -----------------------------------------------------------------------------
//
// The `CassFuture` can have the following states:
// - `Created`. We created a future from the driver's future object. The future
//   may not be ready yet but it already has background work scheduled and maybe
//   executing it.
// - `NotSet`. The future is not ready yet and the background work is not done.
// - `Set`. The future is ready and the background work is done. The future may
//   have a result or an error.
//
// -----------------------------------------------------------------------------
// Future State Transitions
// -----------------------------------------------------------------------------
//
// Every state transition is done in the `Future::poll` method.
//
// We enter the `Created` state when calling `poll` for the first time. This is
// the only time we can call `cass_future_set_callback` to set the callback.
// Remember that the callback is called immediately if the future is already
// set.
//
// If the future is not ready, we enter the `NotSet` state which keeps the waker
// and return `Poll::Pending`.
//
// If the state is `Set`, we return `Poll::Ready` with the future result.
//
// The callback is called when the future is set and this is the place where we
// transition to the `Set` state and call the waker.
//
// There are implementation details related to multihreading and synchronization
// but in general, this is how the `Future` trait is implemented.
//
// -----------------------------------------------------------------------------
// Future Results
// -----------------------------------------------------------------------------
//
// The driver future can have the following results:
// - `CassResult`. The future has been set with a successfull query result.
// - `CassPrepared`. The future has been set with a prepared statement.
// - `CassPayload`. The future has been set with a custom payload.
// - `()` (unit). The future has been set with a void result.
// - `CassError`. The future has been set with a client-side error.
// - `CassErrorResult`. The future has been set with a server-side error.
//
// The problem is that the `Future` trait can only return one type of result.
// So we need the type returned by the `poll` method to be able to costruct
// itself from the druver's future. Which means a custom trait. The method(-s)
// of this trait will be called from the `poll` method in case of a successful

/// The future result of a DataStax C++ driver operation.
///
/// It can represent a result if the operation completed successfully or an
/// error if the operation failed.
#[must_use]
pub struct DriverFuture {
    /// The driver's future object.
    inner:   *mut struct_CassFuture_,
    /// The session that created the future.
    ///
    /// The future must not outlive the session.
    session: Session,
}

impl DriverFuture {
    /// Creates a new future object.
    pub fn new(inner: *mut struct_CassFuture_, session: Session) -> Self {
        assert!(
            !inner.is_null(),
            "the driver's future object must not be null"
        );
        Self {
            inner,
            session,
        }
    }

    /// Returns the raw pointer to the future object.
    pub(crate) fn inner(&self) -> *mut struct_CassFuture_ {
        self.inner
    }

    /// Checks whether the future has been completed.
    pub fn is_ready(&self) -> bool {
        let ready = unsafe { cass_future_ready(self.inner()) };

        ready != CASS_FALSE
    }

    /// Waits for the future to be set with either a result or error.
    ///
    /// This will block the current thread.
    pub fn wait(&self) {
        unsafe { cass_future_wait(self.inner()) }
    }

    /// The same as [`CassFuture::wait`] but timeouts after the given number of
    /// microseconds.
    ///
    /// Returns Ok(`false`) if returned due to timeout.
    ///
    /// Returns an error if the `timeout` overflows.
    pub fn wait_with_timeout(&self, timeout: i64) -> Result<bool, DriverError> {
        let timeout = timeout.try_into().map_err(|_| {
            DriverError::with_kind(crate::DriverErrorKind::LibBadParams)
        })?;
        let completed =
            unsafe { cass_future_wait_timed(self.inner(), timeout) };

        Ok(completed != CASS_FALSE)
    }

    /// Gets the error details from a future that failed as a result of a server
    /// error.
    ///
    /// If the future is not ready this method will block the current thread and
    /// wait for the future to be set.
    ///
    /// Returns [`None`] if the request was successful or the failure was not
    /// caused by a server error.
    pub fn get_error_details(&self) -> Option<DriverErrorDetails> {
        let result = unsafe { cass_future_get_error_result(self.inner()) };

        DriverErrorDetails::from_driver(result)
    }

    /// Gets the error category from future.
    ///
    /// If the future is not ready this method will block the current thread and
    /// wait for the future to be set.
    ///
    /// Returns [`None`] if the future has been completed successfully.
    pub fn get_error_kind(&self) -> Option<DriverErrorKind> {
        let code = unsafe { cass_future_error_code(self.inner()) };

        DriverErrorKind::from_driver(code)
    }

    /// Gets the error message from future.
    ///
    /// If the future is not ready this method will block the current thread and
    /// wait for the future to be set.
    ///
    /// Returns `None` if the future has been completed successfully.
    pub fn get_error_message(&self) -> Option<String> {
        let mut string = std::ptr::null();
        let mut string_len = 0;
        unsafe {
            cass_future_error_message(
                self.inner(),
                &mut string,
                &mut string_len,
            )
        };

        if string.is_null() {
            return None;
        }

        let ptr = string as *const u8;
        let slice = unsafe { slice::from_raw_parts(ptr, string_len) };
        let string = String::from_utf8_lossy(slice);

        if string.is_empty() {
            None
        } else {
            // The error string is owned by the future so here we copy it to
            // simplify memory management, instead of returning a slice.
            //
            // It also makes error handling easier as we send error objects that
            // own all of their data including the error message.
            Some(string.into_owned())
        }
    }

    /// Gets the tracing ID associated with the request.
    ///
    /// Returns an error if there is no tracing ID associated with the request,
    /// or if the future does not represent a request sent to a Cassandra
    /// server.
    pub fn get_tracing_id(&self) -> Result<CqlUuid, DriverError> {
        let mut id = struct_CassUuid_ {
            clock_seq_and_node: 0,
            time_and_version:   0,
        };
        let code = unsafe { cass_future_tracing_id(self.inner(), &mut id) };

        to_result::<()>(code).map(|_| CqlUuid::from_driver(id))
    }
}

impl Drop for DriverFuture {
    /// Frees the future instance.
    ///
    /// A future can be freed anytime.
    fn drop(&mut self) {
        unsafe { cass_future_free(self.inner()) };
    }
}

unsafe impl Send for DriverFuture {}
unsafe impl Sync for DriverFuture {}
