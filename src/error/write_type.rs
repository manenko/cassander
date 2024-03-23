#[rustfmt::skip]
use crate::ffi::{
    enum_CassWriteType_,
    enum_CassWriteType__CASS_WRITE_TYPE_BATCH          as CASS_WRITE_TYPE_BATCH,
    enum_CassWriteType__CASS_WRITE_TYPE_BATCH_LOG      as CASS_WRITE_TYPE_BATCH_LOG,
    enum_CassWriteType__CASS_WRITE_TYPE_CAS            as CASS_WRITE_TYPE_CAS,
    enum_CassWriteType__CASS_WRITE_TYPE_CDC            as CASS_WRITE_TYPE_CDC,
    enum_CassWriteType__CASS_WRITE_TYPE_COUNTER        as CASS_WRITE_TYPE_COUNTER,
    enum_CassWriteType__CASS_WRITE_TYPE_SIMPLE         as CASS_WRITE_TYPE_SIMPLE,
    enum_CassWriteType__CASS_WRITE_TYPE_UNKNOWN        as CASS_WRITE_TYPE_UNKNOWN,
    enum_CassWriteType__CASS_WRITE_TYPE_UNLOGGED_BATCH as CASS_WRITE_TYPE_UNLOGGED_BATCH,
    enum_CassWriteType__CASS_WRITE_TYPE_VIEW           as CASS_WRITE_TYPE_VIEW,
};

/// A type of a Cassandra write query.
///
/// This information is returned by Cassandra when a write timeout is raised to
/// indicate what type of write timed out. This information is useful to decide
/// which retry policy to adopt.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriteType {
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

impl WriteType {
    /// Creates a new `WriteType` from a driver object.
    ///
    /// Returns [`None`] if the `write_type` is `CASS_WRITE_TYPE_UNKNOWN`.
    #[rustfmt::skip]
    pub(crate) fn from_driver(write_type: enum_CassWriteType_) -> Option<Self> {
        use WriteType::*;

        match write_type {
            CASS_WRITE_TYPE_SIMPLE         => Some(Simple),
            CASS_WRITE_TYPE_BATCH          => Some(Batch),
            CASS_WRITE_TYPE_UNLOGGED_BATCH => Some(UnloggedBatch),
            CASS_WRITE_TYPE_COUNTER        => Some(Counter),
            CASS_WRITE_TYPE_BATCH_LOG      => Some(BatchLog),
            CASS_WRITE_TYPE_CAS            => Some(Cas),
            CASS_WRITE_TYPE_VIEW           => Some(View),
            CASS_WRITE_TYPE_CDC            => Some(Cdc),
            CASS_WRITE_TYPE_UNKNOWN        => None,
            unknown                        => Some(Other(unknown)),
        }
    }
}
