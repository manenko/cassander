use std::sync::Arc;

use crate::driver::ffi::{
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
pub struct CassSession(Arc<CassSessionWrapper>);

impl CassSession {
    /// Creates a new Cassandra session.
    pub fn new() -> Self {
        let session = unsafe { cass_session_new() };

        Self(Arc::new(CassSessionWrapper(session)))
    }

    /// Returns the raw pointer to the session object.
    pub fn as_raw(&self) -> *mut struct_CassSession_ {
        self.0.as_raw()
    }
}

impl Default for CassSession {
    /// Creates a new Cassandra session.
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
struct CassSessionWrapper(*mut struct_CassSession_);

impl CassSessionWrapper {
    /// Returns the raw pointer to the session object.
    pub fn as_raw(&self) -> *mut struct_CassSession_ {
        self.0
    }
}

impl Drop for CassSessionWrapper {
    fn drop(&mut self) {
        unsafe { cass_session_free(self.as_raw()) }
    }
}

unsafe impl Send for CassSessionWrapper {}
unsafe impl Sync for CassSessionWrapper {}

