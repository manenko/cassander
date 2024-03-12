use crate::convert::MaybeFrom;
use crate::driver::ffi::{
    enum_CassConsistency_,
    enum_CassConsistency__CASS_CONSISTENCY_ALL as CASS_CONSISTENCY_ALL,
    enum_CassConsistency__CASS_CONSISTENCY_ANY as CASS_CONSISTENCY_ANY,
    enum_CassConsistency__CASS_CONSISTENCY_EACH_QUORUM as CASS_CONSISTENCY_EACH_QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_ONE as CASS_CONSISTENCY_LOCAL_ONE,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_QUORUM as CASS_CONSISTENCY_LOCAL_QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_LOCAL_SERIAL as CASS_CONSISTENCY_LOCAL_SERIAL,
    enum_CassConsistency__CASS_CONSISTENCY_ONE as CASS_CONSISTENCY_ONE,
    enum_CassConsistency__CASS_CONSISTENCY_QUORUM as CASS_CONSISTENCY_QUORUM,
    enum_CassConsistency__CASS_CONSISTENCY_SERIAL as CASS_CONSISTENCY_SERIAL,
    enum_CassConsistency__CASS_CONSISTENCY_THREE as CASS_CONSISTENCY_THREE,
    enum_CassConsistency__CASS_CONSISTENCY_TWO as CASS_CONSISTENCY_TWO,
    enum_CassConsistency__CASS_CONSISTENCY_UNKNOWN as CASS_CONSISTENCY_UNKNOWN,
};

/// Consistency levels for Cassandra read and write operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serialize_cluster_config",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CassConsistency {
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

impl MaybeFrom<enum_CassConsistency_> for CassConsistency {
    #[rustfmt::skip]
    fn maybe_from(value: enum_CassConsistency_) -> Option<Self> {
        match value {
            CASS_CONSISTENCY_ANY          => Some(CassConsistency::Any),
            CASS_CONSISTENCY_ONE          => Some(CassConsistency::One),
            CASS_CONSISTENCY_TWO          => Some(CassConsistency::Two),
            CASS_CONSISTENCY_THREE        => Some(CassConsistency::Three),
            CASS_CONSISTENCY_QUORUM       => Some(CassConsistency::Quorum),
            CASS_CONSISTENCY_ALL          => Some(CassConsistency::All),
            CASS_CONSISTENCY_LOCAL_QUORUM => Some(CassConsistency::LocalQuorum),
            CASS_CONSISTENCY_EACH_QUORUM  => Some(CassConsistency::EachQuorum),
            CASS_CONSISTENCY_SERIAL       => Some(CassConsistency::Serial),
            CASS_CONSISTENCY_LOCAL_ONE    => Some(CassConsistency::LocalOne),
            CASS_CONSISTENCY_LOCAL_SERIAL => Some(CassConsistency::LocalSerial),
            CASS_CONSISTENCY_UNKNOWN      => None,
            unknown                       => unreachable!("unexpected value CassConsistency {}", unknown),
        }
    }
}

impl From<CassConsistency> for enum_CassConsistency_ {
    #[rustfmt::skip]
    fn from(value: CassConsistency) -> Self {
        match value {
            CassConsistency::Any          => CASS_CONSISTENCY_ANY,
            CassConsistency::One          => CASS_CONSISTENCY_ONE,
            CassConsistency::Two          => CASS_CONSISTENCY_TWO,
            CassConsistency::Three        => CASS_CONSISTENCY_THREE,
            CassConsistency::Quorum       => CASS_CONSISTENCY_QUORUM,
            CassConsistency::All          => CASS_CONSISTENCY_ALL,
            CassConsistency::LocalQuorum  => CASS_CONSISTENCY_LOCAL_QUORUM,
            CassConsistency::EachQuorum   => CASS_CONSISTENCY_EACH_QUORUM,
            CassConsistency::Serial       => CASS_CONSISTENCY_SERIAL,
            CassConsistency::LocalOne     => CASS_CONSISTENCY_LOCAL_ONE,
            CassConsistency::LocalSerial  => CASS_CONSISTENCY_LOCAL_SERIAL,
        }
    }
}
