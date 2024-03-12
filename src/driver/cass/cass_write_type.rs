use crate::convert::MaybeFrom;
use crate::driver::ffi::{
    enum_CassWriteType_,
    enum_CassWriteType__CASS_WRITE_TYPE_BATCH as CASS_WRITE_TYPE_BATCH,
    enum_CassWriteType__CASS_WRITE_TYPE_BATCH_LOG as CASS_WRITE_TYPE_BATCH_LOG,
    enum_CassWriteType__CASS_WRITE_TYPE_CAS as CASS_WRITE_TYPE_CAS,
    enum_CassWriteType__CASS_WRITE_TYPE_CDC as CASS_WRITE_TYPE_CDC,
    enum_CassWriteType__CASS_WRITE_TYPE_COUNTER as CASS_WRITE_TYPE_COUNTER,
    enum_CassWriteType__CASS_WRITE_TYPE_SIMPLE as CASS_WRITE_TYPE_SIMPLE,
    enum_CassWriteType__CASS_WRITE_TYPE_UNKNOWN as CASS_WRITE_TYPE_UNKNOWN,
    enum_CassWriteType__CASS_WRITE_TYPE_UNLOGGED_BATCH as CASS_WRITE_TYPE_UNLOGGED_BATCH,
    enum_CassWriteType__CASS_WRITE_TYPE_VIEW as CASS_WRITE_TYPE_VIEW,
};

/// A type of a Cassandra write query.
///
/// This information is returned by Cassandra when a write timeout is raised to
/// indicate what type of write timed out. This information is useful to decide
/// which retry policy to adopt.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CassWriteType {
    /// A write to a single partition key.
    Simple,
    /// A write to a multiple partition key that used the distributed batch log
    /// to ensure atomicity (atomicity meaning that if any statement in the
    /// batch succeeds, all will eventually succeed).
    Batch,
    /// A write to a multiple partition key that doesn't use the distributed
    /// batch log.
    UnloggedBatch,
    /// A counter write (that can be for one or multiple partition key).
    Counter,
    /// An initial write to the distributed batch log that Cassandra performs
    /// internally before a [`CassWriteType::Batch`] write.
    BatchLog,
    /// A conditional write.
    Cas,
    /// Indicates that the timeout was related to acquiring locks needed for
    /// updating materialized views affected by write operation.
    View,
    /// Indicates that the timeout was related to acquiring space for change
    /// data capture logs for CDC (Change Data Capture) tracked tables.
    Cdc,
    /// The driver returned a write type unknown to this crate.
    Other(u32),
}

impl MaybeFrom<enum_CassWriteType_> for CassWriteType {
    #[rustfmt::skip]
    fn maybe_from(value: enum_CassWriteType_) -> Option<Self> {
        match value {
            CASS_WRITE_TYPE_SIMPLE         => Some(CassWriteType::Simple),
            CASS_WRITE_TYPE_BATCH          => Some(CassWriteType::Batch),
            CASS_WRITE_TYPE_UNLOGGED_BATCH => Some(CassWriteType::UnloggedBatch),
            CASS_WRITE_TYPE_COUNTER        => Some(CassWriteType::Counter),
            CASS_WRITE_TYPE_BATCH_LOG      => Some(CassWriteType::BatchLog),
            CASS_WRITE_TYPE_CAS            => Some(CassWriteType::Cas),
            CASS_WRITE_TYPE_VIEW           => Some(CassWriteType::View),
            CASS_WRITE_TYPE_CDC            => Some(CassWriteType::Cdc),
            CASS_WRITE_TYPE_UNKNOWN        => None,
            unknown                        => Some(CassWriteType::Other(unknown)),
        }
    }
}

impl From<CassWriteType> for enum_CassWriteType_ {
    #[rustfmt::skip]
    fn from(value: CassWriteType) -> Self {
        match value {
            CassWriteType::Simple        => CASS_WRITE_TYPE_SIMPLE,
            CassWriteType::Batch         => CASS_WRITE_TYPE_BATCH,
            CassWriteType::UnloggedBatch => CASS_WRITE_TYPE_UNLOGGED_BATCH,
            CassWriteType::Counter       => CASS_WRITE_TYPE_COUNTER,
            CassWriteType::BatchLog      => CASS_WRITE_TYPE_BATCH_LOG,
            CassWriteType::Cas           => CASS_WRITE_TYPE_CAS,
            CassWriteType::View          => CASS_WRITE_TYPE_VIEW,
            CassWriteType::Cdc           => CASS_WRITE_TYPE_CDC,
            CassWriteType::Other(other)  => other,
        }
    }
}
