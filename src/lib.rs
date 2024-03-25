//! # Cassander
//!
//! The `cassander` crate provides idiomatic Rust bindings to the DataStax C++
//! driver for Apache Cassandra. The crate is a work in progress and is not
//! feature complete. The crate is not yet ready for production use.

mod cluster;
mod consistency;
mod error;
pub(crate) mod ffi;
pub(crate) mod future;
mod retry_policy;
mod session;
mod ssl;
mod ssl_verify_flags;
mod timestamp_gen;
mod version;

pub mod allocator;
pub mod cql;
pub mod logging;

pub use cluster::*;
pub use consistency::*;
pub use error::*;
pub use retry_policy::*;
pub use session::*;
pub use ssl::*;
pub use ssl_verify_flags::*;
pub use timestamp_gen::*;
pub use version::*;

// TODO: How do we support different versions of the C++ driver? Some functions
//       are only available in newer versions of the driver. We need to be able
//       to conditionally compile code based on the version of the driver.
