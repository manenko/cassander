use std::ffi::c_char;
use std::sync::Arc;

use thiserror::Error;

use crate::ffi::{
    cass_session_connect,
    cass_session_connect_keyspace_n,
    cass_session_free,
    cass_session_new,
    struct_CassSession_,
};
use crate::future::DriverFuture;
use crate::{
    Cluster,
    DriverError,
    SessionConfig,
    SessionConfigError,
};

/// An error that is raised when a session cannot be created.
#[derive(Debug, Error)]
pub enum SessionCreationError {
    /// An error that is raised when a session configuration is invalid.
    #[error(transparent)]
    Config(#[from] SessionConfigError),

    /// An error that is raised when a session cannot be established.
    #[error(transparent)]
    Connection(#[from] DriverError),
}

/// A session object is used to execute queries and maintains cluster state
/// through the control connection.
///
/// The control connection is used to auto-discover nodes and monitor cluster
/// changes (topology and schema). Each session also maintains multiple pools of
/// connections to cluster nodes which are used to query the cluster.
#[derive(Clone)]
pub struct Session {
    inner:     Arc<SessionWrapper>,
    page_size: Option<usize>,
}

impl Session {
    /// Creates a new Cassandra session.
    pub(crate) fn new() -> Self {
        let session = unsafe { cass_session_new() };

        Self {
            inner:     Arc::new(SessionWrapper(session)),
            page_size: None,
        }
    }

    /// Returns the raw pointer to the session object.
    pub(crate) fn inner(&self) -> *mut struct_CassSession_ {
        self.inner.inner()
    }

    /// Connects to the cluster and returns a session.
    pub async fn connect(
        mut config: SessionConfig,
    ) -> Result<Session, SessionCreationError> {
        let keyspace = std::mem::take(&mut config.keyspace);
        let page_size = std::mem::take(&mut config.page_size);

        let cluster: Cluster = config.try_into()?;

        let mut session = match keyspace {
            Some(keyspace) => Self::connect_keyspace(cluster, keyspace).await?,
            None => Self::connect_no_keyspace(cluster).await?,
        };

        session.page_size = page_size;

        Ok(session)
    }

    /// Connects to the cluster without specifying a keyspace.
    fn connect_no_keyspace(cluster: Cluster) -> DriverFuture<Session> {
        let session = Self::new();

        let future =
            unsafe { cass_session_connect(session.inner(), cluster.inner()) };

        DriverFuture::new(future, session)
    }

    /// Connects to the cluster and sets the default keyspace.
    fn connect_keyspace<T>(
        cluster: Cluster,
        keyspace: T,
    ) -> DriverFuture<Session>
    where
        T: AsRef<str>,
    {
        let session = Self::new();

        let keyspace = keyspace.as_ref();
        let keyspace_len = keyspace.len();
        let keyspace_ptr = keyspace.as_ptr() as *const c_char;

        let future = unsafe {
            cass_session_connect_keyspace_n(
                session.inner(),
                cluster.inner(),
                keyspace_ptr,
                keyspace_len,
            )
        };

        DriverFuture::new(future, session)
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
