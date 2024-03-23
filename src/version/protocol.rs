#[rustfmt::skip]
use crate::ffi::{
    enum_CassProtocolVersion_,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_DSEV1 as VERSION_DSEV1,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_DSEV2 as VERSION_DSEV2,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V1    as VERSION_V1,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V2    as VERSION_V2,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V3    as VERSION_V3,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V4    as VERSION_V4,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V5    as VERSION_V5,
};

/// Apache Cassandra protocol version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ProtocolVersion {
    /// Deprecated.
    #[deprecated(note = "This version is deprecated.")]
    V1,
    /// Deprecated.
    #[deprecated(note = "This version is deprecated.")]
    V2,
    /// Apache Cassandra protocol version 3.
    V3,
    /// Apache Cassandra protocol version 4.
    V4,
    /// Apache Cassandra protocol version 5.
    V5,
    /// Only supported when using the DSE driver with DataStax Enterprise.
    Dsev1,
    /// Only supported when using the DSE driver with DataStax Enterprise.
    Dsev2,
    /// The protocol version is unknown to this crate.
    Unknown(u32),
}

impl ProtocolVersion {
    /// Creates a new `ProtocolVersion` from the driver object.
    #[rustfmt::skip]
    pub(crate) fn from_driver(version: enum_CassProtocolVersion_) -> Self {
        use ProtocolVersion::*;

        #[allow(deprecated)]
        match version {
            VERSION_V1      => V1,
            VERSION_V2      => V2,
            VERSION_V3      => V3,
            VERSION_V4      => V4,
            VERSION_V5      => V5,
            VERSION_DSEV1   => Dsev1,
            VERSION_DSEV2   => Dsev2,
            unknown         => Unknown(unknown),
        }
    }
}
