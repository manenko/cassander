use crate::ffi::{
    cass_collection_free,
    cass_collection_new,
    enum_CassCollectionType_,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_LIST as LIST,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_MAP as MAP,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_SET as SET,
    struct_CassCollection_,
};

/// A collection of CQL values.
#[repr(transparent)]
pub(crate) struct CassCollection(*mut struct_CassCollection_);

/// The type of a CQL collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CassCollectionKind {
    /// A list collection.
    List,
    /// A set collection.
    Set,
    /// A map collection.
    Map,
}

impl From<CassCollectionKind> for enum_CassCollectionType_ {
    #[rustfmt::skip]
    fn from(kind: CassCollectionKind) -> Self {
        use CassCollectionKind::*;

        match kind {
            List => LIST,
            Set  => SET,
            Map  => MAP,
        }
    }
}

impl From<enum_CassCollectionType_> for CassCollectionKind {
    #[rustfmt::skip]
    fn from(kind: enum_CassCollectionType_) -> Self {
        use CassCollectionKind::*;

        match kind {
            LIST    => List,
            SET     => Set,
            MAP     => Map,
            unknown => unreachable!("invalid collection kind, {}", unknown),
        }
    }
}

impl CassCollection {
    /// Creates a new `CassCollection` from the given driver object.
    pub fn from_driver(collection: *mut struct_CassCollection_) -> Self {
        Self(collection)
    }

    /// Creates a new `CqlCollection` with the given kind and capacity.
    pub fn new(kind: CassCollectionKind, capacity: usize) -> Self {
        let collection = unsafe { cass_collection_new(kind.into(), capacity) };

        Self::from_driver(collection)
    }

    /// Returns the inner driver object.
    pub fn inner(&self) -> *mut struct_CassCollection_ {
        self.0
    }
}

impl Drop for CassCollection {
    fn drop(&mut self) {
        unsafe { cass_collection_free(self.inner()) }
    }
}
