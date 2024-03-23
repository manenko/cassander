#[rustfmt::skip]
use crate::ffi::{
    enum_CassConsistency_,
    enum_CassConsistency__CASS_CONSISTENCY_ALL          as ALL,
    enum_CassConsistency__CASS_CONSISTENCY_ANY          as ANY,
    enum_CassConsistency__CASS_CONSISTENCY_EACH_QUORUM  as EACH_QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_ONE    as LOCAL_ONE,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_QUORUM as LOCAL_QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_SERIAL as LOCAL_SERIAL,
    enum_CassConsistency__CASS_CONSISTENCY_ONE          as ONE,
    enum_CassConsistency__CASS_CONSISTENCY_QUORUM       as QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_SERIAL       as SERIAL,
    enum_CassConsistency__CASS_CONSISTENCY_THREE        as THREE,
    enum_CassConsistency__CASS_CONSISTENCY_TWO          as TWO,
    enum_CassConsistency__CASS_CONSISTENCY_UNKNOWN      as UNKNOWN,
};

/// Consistency levels for Cassandra read and write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Consistency {
    /// A single replica acknowledges the read or write operation.
    One,
    /// Two replicas acknowledge the read or write operation.
    Two,
    /// Three replicas acknowledge the read or write operation.
    Three,
    /// The majority of replicas in the data center acknowledge the read or
    /// write operation.
    Quorum,
    /// All replicas in the data center acknowledge the read or write
    /// operation.
    All,
    /// A quorum of replicas in all data centers acknowledge the read or write
    /// operation.
    EachQuorum,
    /// A quorum of replicas in the local data center acknowledge the write
    /// operation. No reads.
    LocalQuorum,
    /// Only the local replica (coordinator) acknowledges the read or write
    /// operation.
    LocalOne,
    /// Similar to [`LocalQuorum`](Self::LocalQuorum) but confined to the
    /// replicas holding the partition in the local data center. No writes.
    LocalSerial,
    /// Used in lightweight transactions to ensure all replicas sequentially
    /// agree on the write.
    Serial,
    /// A write must be written to at least one node. No reads.
    Any,
}

impl Consistency {
    /// Creates a new `Consistency` from the driver's object.
    ///
    /// Returns [`None`] if `consistency` is `CASS_CONSISTENCY_UNKNOWN`.
    #[rustfmt::skip]
    pub(crate) fn from_driver(consistency: enum_CassConsistency_) -> Option<Self> {
        use Consistency::*;

        match consistency {
            ANY          => Some(Any),
            ONE          => Some(One),
            TWO          => Some(Two),
            THREE        => Some(Three),
            QUORUM       => Some(Quorum),
            ALL          => Some(All),
            LOCAL_QUORUM => Some(LocalQuorum),
            EACH_QUORUM  => Some(EachQuorum),
            SERIAL       => Some(Serial),
            LOCAL_ONE    => Some(LocalOne),
            LOCAL_SERIAL => Some(LocalSerial),
            UNKNOWN      => None,
            unknown      => unreachable!("unexpected value of enum_CassConsistency_ {}", unknown),
        }
    }

    /// Converts this `Consistency` to the driver's object.
    #[rustfmt::skip]
    pub(crate) fn to_driver(self) -> enum_CassConsistency_ {
        use Consistency::*;

        match self {
            Any          => ANY,
            One          => ONE,
            Two          => TWO,
            Three        => THREE,
            Quorum       => QUORUM,
            All          => ALL,
            LocalQuorum  => LOCAL_QUORUM,
            EachQuorum   => EACH_QUORUM,
            Serial       => SERIAL,
            LocalOne     => LOCAL_ONE,
            LocalSerial  => LOCAL_SERIAL,
        }
    }
}
