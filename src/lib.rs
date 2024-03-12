//! # Cassander
//!
//! The `cassander` crate provides idiomatic Rust bindings to the DataStax C++
//! driver for Apache Cassandra. The crate is a work in progress and is not
//! feature complete. The crate is not yet ready for production use.
//!
//! # Architecture
//!
//! ```text
//!       +---------+
//!       |cassander|
//!       +---------+
//!            |
//!            |  +----------+
//!            +->|mod driver|
//!            |  +----------+
//!            |        |
//!            |        |  +-------+
//!            |        +->|mod ffi|
//!            |        |  +-------+
//!            |        |      |  +--------------------+
//!            |        |      +->|struct_CassStatement|
//!            |        |    B |  +--------------------+
//!            |        |    I |  +--------------------+
//!            |        |    N +->|        ...         |
//!            |        |    D |  +--------------------+
//!            |        |    I |  +--------------------+
//!            |        |    N +->|   enum_CassError   |
//!            |        |    G |  +--------------------+
//!            |        |    S |  +--------------------+
//!            |        |      +->|cass_session_prepare|
//!            |        |         +--------------------+
//!            |        |
//!            |        |  +--------+
//!            |        +->|mod cass|
//!            |           +--------+
//!            |                |  +-------------+
//!            |                +->|  CassError  |
//!            |                |  +-------------+
//!            |                |  +-------------+
//!            |              S +->| CassFuture  |
//!            |              A |  +-------------+
//!            |              F |  +-------------+
//!            |              E +->|CassStatement|
//!            |                |  +-------------+
//!            |              W |  +-------------+
//!            |              R +->|     ...     |
//!            |              A |  +-------------+
//!            |              P |  +-------------+
//!            |              P +->|  newtypes   |
//!            |              E |  +-------------+
//!            |              R |  +-------------+
//!            |              S +->|   traits    |
//!            |                |  +-------------+
//!            |                |  +-------------+
//!            |                +->|   methods   |
//!            |                   +-------------+
//!            |
//! -----------+---------------------------------------------
//!            |
//!            v
//!     +-------------+
//!     |Rustified API|
//!     +-------------+
//!            |
//!            |  +---------------+
//!            +->|   Statement   |
//!            |  +---------------+
//!            |  +---------------+
//!            +->|CachedStatement|
//!            |  +---------------+
//!            |  +---------------+
//!            +->|     Query     |
//!            |  +---------------+
//!            |  +---------------+
//!            +->|   CqlValue    |
//!            |  +---------------+
//!            |  +---------------+
//!            +->| ClusterConfig |
//!            |  +---------------+
//!            |  +---------------+
//!            +->|      ...      |
//!               +---------------+
//! ```

pub mod convert;
pub mod driver;

// TODO: How do we support different versions of the C++ driver? Some functions
//       are only available in newer versions of the driver. We need to be able
//       to conditionally compile code based on the version of the driver.
