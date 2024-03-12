use crate::driver::ffi::{
    enum_CassProtocolVersion_,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_DSEV1 as CASS_PROTOCOL_VERSION_DSEV1,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_DSEV2 as CASS_PROTOCOL_VERSION_DSEV2,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V1 as CASS_PROTOCOL_VERSION_V1,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V2 as CASS_PROTOCOL_VERSION_V2,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V3 as CASS_PROTOCOL_VERSION_V3,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V4 as CASS_PROTOCOL_VERSION_V4,
    enum_CassProtocolVersion__CASS_PROTOCOL_VERSION_V5 as CASS_PROTOCOL_VERSION_V5,
};

/// Apache Cassandra protocol version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CassProtocolVersion {
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
    DSEV1,
    /// Only supported when using the DSE driver with DataStax Enterprise.
    DSEV2,
}

impl From<CassProtocolVersion> for enum_CassProtocolVersion_ {
    #[rustfmt::skip]
    fn from(version: CassProtocolVersion) -> Self {
        #[allow(deprecated)]
        match version {
            CassProtocolVersion::V1    => CASS_PROTOCOL_VERSION_V1,
            CassProtocolVersion::V2    => CASS_PROTOCOL_VERSION_V2,
            CassProtocolVersion::V3    => CASS_PROTOCOL_VERSION_V3,
            CassProtocolVersion::V4    => CASS_PROTOCOL_VERSION_V4,
            CassProtocolVersion::V5    => CASS_PROTOCOL_VERSION_V5,
            CassProtocolVersion::DSEV1 => CASS_PROTOCOL_VERSION_DSEV1,
            CassProtocolVersion::DSEV2 => CASS_PROTOCOL_VERSION_DSEV2,
        }
    }
}

impl From<enum_CassProtocolVersion_> for CassProtocolVersion {
    #[rustfmt::skip]
    fn from(version: enum_CassProtocolVersion_) -> Self {
        #[allow(deprecated)]
        match version {
            CASS_PROTOCOL_VERSION_V1    => CassProtocolVersion::V1,
            CASS_PROTOCOL_VERSION_V2    => CassProtocolVersion::V2,
            CASS_PROTOCOL_VERSION_V3    => CassProtocolVersion::V3,
            CASS_PROTOCOL_VERSION_V4    => CassProtocolVersion::V4,
            CASS_PROTOCOL_VERSION_V5    => CassProtocolVersion::V5,
            CASS_PROTOCOL_VERSION_DSEV1 => CassProtocolVersion::DSEV1,
            CASS_PROTOCOL_VERSION_DSEV2 => CassProtocolVersion::DSEV2,
            unknown                     => unreachable!("unknown Apache Cassandra protocol version {}", unknown),
        }
    }
}
