use std::sync::Arc;

use crate::ffi::{
    cass_session_free,
    cass_session_new,
    struct_CassSession_,
};

/// A session object is used to execute queries and maintains cluster state
/// through the control connection.
///
/// The control connection is used to auto-discover nodes and monitor cluster
/// changes (topology and schema). Each session also maintains multiple pools of
/// connections to cluster nodes which are used to query the cluster.
#[derive(Clone)]
pub struct Session(Arc<SessionWrapper>);

impl Session {
    /// Creates a new Cassandra session.
    pub(crate) fn new() -> Self {
        let session = unsafe { cass_session_new() };

        Self(Arc::new(SessionWrapper(session)))
    }

    /// Returns the raw pointer to the session object.
    pub(crate) fn inner(&self) -> *mut struct_CassSession_ {
        self.0.inner()
    }
}

impl Default for Session {
    /// Creates a new Cassandra session.
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
struct SessionWrapper(*mut struct_CassSession_);

impl SessionWrapper {
    /// Returns the raw pointer to the session object.
    pub fn inner(&self) -> *mut struct_CassSession_ {
        self.0
    }
}

impl Drop for SessionWrapper {
    fn drop(&mut self) {
        unsafe { cass_session_free(self.inner()) }
    }
}

unsafe impl Send for SessionWrapper {}
unsafe impl Sync for SessionWrapper {}
