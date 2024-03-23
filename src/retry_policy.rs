use crate::ffi::{
    cass_retry_policy_default_new,
    cass_retry_policy_fallthrough_new,
    cass_retry_policy_free,
    cass_retry_policy_logging_new,
    struct_CassRetryPolicy_,
};

/// A retry policy that defines a retry schedule for a query.
#[repr(transparent)]
pub struct RetryPolicy(*mut struct_CassRetryPolicy_);

impl RetryPolicy {
    /// Creates a new default retry policy.
    ///
    /// This policy retries queries in the following cases:
    ///
    /// - on a read timeout, if enough replicas replied but data was not
    /// received;
    /// - on a write timeout, if a timeout occurs while writing the distributed
    /// batch log;
    /// - on unavailable, it will move to the next host.
    ///
    /// In all other cases the error will be returned.
    ///
    /// This policy always uses the query's original consistency level.
    pub fn new() -> Self {
        let policy = unsafe { cass_retry_policy_default_new() };

        Self(policy)
    }

    /// Creates a new fallthrough retry policy.
    ///
    /// This policy never retries or ignores a server-side failure. The error
    /// is always returned.
    pub fn fallthrough() -> Self {
        let policy = unsafe { cass_retry_policy_fallthrough_new() };

        Self(policy)
    }

    /// Creates a new logging retry policy.
    ///
    /// This policy logs the retry decision. The driver will log the decisions
    /// using the `CASS_LOG_INFO` log level.
    ///
    /// The function returns `None` if the `child_policy` is a logging retry
    /// policy.
    pub fn logging(child_policy: &RetryPolicy) -> Option<Self> {
        let policy =
            unsafe { cass_retry_policy_logging_new(child_policy.inner()) };

        if policy.is_null() {
            None
        } else {
            Some(Self(policy))
        }
    }

    /// Returns a raw pointer to the retry policy driver object.
    pub(crate) fn inner(&self) -> *mut struct_CassRetryPolicy_ {
        self.0
    }
}

impl Default for RetryPolicy {
    /// Creates a new default retry policy.
    ///
    /// This policy retries queries in the following cases:
    ///
    /// - on a read timeout, if enough replicas replied but data was not
    /// received;
    /// - on a write timeout, if a timeout occurs while writing the distributed
    /// batch log;
    /// - on unavailable, it will move to the next host.
    ///
    /// In all other cases the error will be returned.
    ///
    /// This policy always uses the query's original consistency level.
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RetryPolicy {
    fn drop(&mut self) {
        unsafe { cass_retry_policy_free(self.inner()) }
    }
}
