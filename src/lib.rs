//! # Cassander
//!
//! The `cassander` crate provides idiomatic Rust bindings to the DataStax C++
//! driver for Apache Cassandra. The crate is a work in progress and is not
//! feature complete. The crate is not yet ready for production use.

mod cluster;
mod config;
mod consistency;
mod error;
pub(crate) mod ffi;
pub(crate) mod future;
mod load_balancing_policy;
mod retry_policy;
mod routing;
mod session;
mod speculative_execution_policy;
mod ssl;
mod ssl_verify_flags;
mod timestamp_gen;
mod version;

pub mod allocator;
pub mod authenticator;
pub mod cql;
pub mod logging;

pub(crate) use cluster::Cluster;
pub use config::*;
pub use consistency::*;
pub use error::*;
pub use load_balancing_policy::*;
pub(crate) use retry_policy::CassRetryPolicy;
pub use retry_policy::*;
pub use routing::*;
pub use session::*;
pub use speculative_execution_policy::*;
pub use ssl::*;
pub use ssl_verify_flags::*;
pub use timestamp_gen::*;
pub use version::*;

// TODO: How do we support different versions of the C++ driver? Some functions
//       are only available in newer versions of the driver. We need to be able
//       to conditionally compile code based on the version of the driver.
