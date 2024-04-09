use crate::cql::{
    CqlDecimal,
    CqlDuration,
    CqlInet,
    CqlUuid,
};
use crate::ffi::{
    cass_collection_append_bool,
    cass_collection_append_bytes,
    cass_collection_append_collection,
    cass_collection_append_decimal,
    cass_collection_append_double,
    cass_collection_append_duration,
    cass_collection_append_float,
    cass_collection_append_inet,
    cass_collection_append_int16,
    cass_collection_append_int32,
    cass_collection_append_int64,
    cass_collection_append_int8,
    cass_collection_append_string_n,
    cass_collection_append_uint32,
    cass_collection_append_uuid,
    cass_collection_free,
    cass_collection_new,
    enum_CassCollectionType_,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_LIST as LIST,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_MAP as MAP,
    enum_CassCollectionType__CASS_COLLECTION_TYPE_SET as SET,
    enum_cass_bool_t_cass_false as CASS_FALSE,
    enum_cass_bool_t_cass_true as CASS_TRUE,
    struct_CassCollection_,
};
use crate::{
    to_result,
    DriverError,
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

    /// Appends an `i8` value to the collection.
    pub fn append_i8(&mut self, value: i8) -> Result<(), DriverError> {
        let error = unsafe { cass_collection_append_int8(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `i16` value to the collection.
    pub fn append_i16(&mut self, value: i16) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_int16(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `i32` value to the collection.
    pub fn append_i32(&mut self, value: i32) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_int32(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `i64` value to the collection.
    pub fn append_i64(&mut self, value: i64) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_int64(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `u32` value to the collection.
    pub fn append_u32(&mut self, value: u32) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_uint32(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `f32` value to the collection.
    pub fn append_f32(&mut self, value: f32) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_float(self.inner(), value) };

        to_result(error)
    }

    /// Appends an `f64` value to the collection.
    pub fn append_f64(&mut self, value: f64) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_double(self.inner(), value) };

        to_result(error)
    }

    /// Appends a `bool` value to the collection.
    pub fn append_bool(&mut self, value: bool) -> Result<(), DriverError> {
        let value = if value { CASS_TRUE } else { CASS_FALSE };
        let error = unsafe { cass_collection_append_bool(self.inner(), value) };

        to_result(error)
    }

    /// Appends a `String` value to the collection.
    pub fn append_string<S: AsRef<str>>(
        &mut self,
        value: S,
    ) -> Result<(), DriverError> {
        let value = value.as_ref();

        let error = unsafe {
            cass_collection_append_string_n(
                self.inner(),
                value.as_ptr() as _,
                value.len(),
            )
        };

        to_result(error)
    }

    /// Appends raw bytes to the collection.
    pub fn append_bytes<B>(&mut self, value: B) -> Result<(), DriverError>
    where
        B: AsRef<[u8]>,
    {
        let value = value.as_ref();

        let error = unsafe {
            cass_collection_append_bytes(
                self.inner(),
                value.as_ptr(),
                value.len(),
            )
        };

        to_result(error)
    }

    /// Appends a `CqlUuid` value to the collection.
    pub fn append_uuid(&mut self, value: CqlUuid) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_uuid(self.inner(), value.inner()) };

        to_result(error)
    }

    /// Appends a `CqlInet` value to the collection.
    pub fn append_inet(&mut self, value: CqlInet) -> Result<(), DriverError> {
        let error =
            unsafe { cass_collection_append_inet(self.inner(), value.into()) };

        to_result(error)
    }

    /// Appends a `CqlDecimal` value to the collection.
    pub fn append_decimal(
        &mut self,
        value: CqlDecimal,
    ) -> Result<(), DriverError> {
        let (varint, scale) = value.into_signed_bytes_be_and_exponent();
        let error = unsafe {
            cass_collection_append_decimal(
                self.inner(),
                varint.as_ptr(),
                varint.len(),
                scale,
            )
        };

        to_result(error)
    }

    /// Appends a `CqlDuration` value to the collection.
    pub fn append_duration(
        &mut self,
        value: CqlDuration,
    ) -> Result<(), DriverError> {
        let error = unsafe {
            cass_collection_append_duration(
                self.inner(),
                value.months(),
                value.days(),
                value.nanoseconds(),
            )
        };

        to_result(error)
    }

    /// Appends a collection to the collection.
    pub fn append_collection(
        &mut self,
        value: CassCollection,
    ) -> Result<(), DriverError> {
        let error = unsafe {
            cass_collection_append_collection(self.inner(), value.inner())
        };

        to_result(error)
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
