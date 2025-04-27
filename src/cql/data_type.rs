pub(crate) mod driver;

mod custom;
mod indexed;
mod map;
mod native;
mod tuple;
mod udt;

use std::marker::PhantomData;

pub use custom::*;
pub use indexed::*;
pub use map::*;
pub use native::*;
pub use tuple::*;
pub use udt::*;

use self::driver::DriverDataTypeRef;
use crate::cql::driver::DriverDataType;
use crate::cql::ValueType;
use crate::ffi::{
    cass_data_type_free,
    cass_data_type_new,
    cass_data_type_new_from_existing,
    struct_CassDataType_,
};

pub enum DataType {
    /// A custom data type.
    ///
    /// See the [`CustomDataType`] documentation for more information.
    Custom(CustomDataType),
    Indexed(IndexedDataType),
    Map(MapDataType),
    /// A native data type.
    ///
    /// See the [`NativeDataType`] documentation for more information.
    Native(NativeDataType),
    Tuple(TupleDataType),
    Udt(UdtDataType),
}

impl DataType {
    /// Returns the `ascii` native data type.
    pub fn ascii() -> Self {
        Self::Native(NativeDataType::ascii())
    }

    /// Returns the `bigint` native data type.
    pub fn bigint() -> Self {
        Self::Native(NativeDataType::bigint())
    }

    /// Returns the `blob` native data type.
    pub fn blob() -> Self {
        Self::Native(NativeDataType::blob())
    }

    /// Returns the `boolean` native data type.
    pub fn boolean() -> Self {
        Self::Native(NativeDataType::boolean())
    }

    /// Returns the `counter` native data type.
    pub fn counter() -> Self {
        Self::Native(NativeDataType::counter())
    }

    /// Returns the `date` native data type.
    pub fn date() -> Self {
        Self::Native(NativeDataType::date())
    }

    /// Returns the `decimal` native data type.
    pub fn decimal() -> Self {
        Self::Native(NativeDataType::decimal())
    }

    /// Returns the `double` native data type.
    pub fn double() -> Self {
        Self::Native(NativeDataType::double())
    }

    /// Returns the `duration` native data type.
    pub fn duration() -> Self {
        Self::Native(NativeDataType::duration())
    }

    /// Returns the `float` native data type.
    pub fn float() -> Self {
        Self::Native(NativeDataType::float())
    }

    /// Returns the `inet` native data type.
    pub fn inet() -> Self {
        Self::Native(NativeDataType::inet())
    }

    /// Returns the `int` native data type.
    pub fn int() -> Self {
        Self::Native(NativeDataType::int())
    }

    /// Returns the `smallint` native data type.
    pub fn smallint() -> Self {
        Self::Native(NativeDataType::smallint())
    }

    /// Returns the `text` native data type.
    pub fn text() -> Self {
        Self::Native(NativeDataType::text())
    }

    /// Returns the `time` native data type.
    pub fn time() -> Self {
        Self::Native(NativeDataType::time())
    }

    /// Returns the `timeuuid` native data type.
    pub fn timeuuid() -> Self {
        Self::Native(NativeDataType::timeuuid())
    }

    /// Returns the `timestamp` native data type.
    pub fn timestamp() -> Self {
        Self::Native(NativeDataType::timestamp())
    }

    /// Returns the `tinyint` native data type.
    pub fn tinyint() -> Self {
        Self::Native(NativeDataType::tinyint())
    }

    /// Returns the `uuid` native data type.
    pub fn uuid() -> Self {
        Self::Native(NativeDataType::uuid())
    }

    /// Returns the `varchar` native data type.
    pub fn varchar() -> Self {
        Self::Native(NativeDataType::varchar())
    }

    /// Returns the `custom` data type with the given Java class name.
    pub fn custom<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::Custom(CustomDataType::new(name))
    }

    #[rustfmt::skip]
    pub(crate) fn inner(&self) -> &DriverDataType {
        match self {
            DataType::Custom(custom)   => custom.inner(),
            DataType::Indexed(indexed) => indexed.inner(),
            DataType::Map(map)         => map.inner(),
            DataType::Native(native)   => native.inner(),
            DataType::Tuple(tuple)     => tuple.inner(),
            DataType::Udt(udt)         => udt.inner(),
        }
    }

    #[rustfmt::skip]
    pub(crate) fn inner_mut(&mut self) -> &mut DriverDataType {
        match self {
            DataType::Custom(custom)   => custom.inner_mut(),
            DataType::Indexed(indexed) => indexed.inner_mut(),
            DataType::Map(map)         => map.inner_mut(),
            DataType::Native(native)   => native.inner_mut(),
            DataType::Tuple(tuple)     => tuple.inner_mut(),
            DataType::Udt(udt)         => udt.inner_mut(),
        }
    }
}

impl From<DriverDataType> for DataType {
    /// Creates a new `DataType` from the given `DriverDataType`.
    #[rustfmt::skip]
    fn from(dt: DriverDataType) -> Self {
        use ValueType::*;
        match dt.value_type() {
            Custom     => Self::Custom(CustomDataType::from_driver(dt)),
            Set | List => Self::Indexed(IndexedDataType::from_driver(dt)),
            Map        => Self::Map(MapDataType::from_driver(dt)),
            Tuple      => Self::Tuple(TupleDataType::from_driver(dt)),
            Udt        => Self::Udt(UdtDataType::from_driver(dt)),
            _          => Self::Native(NativeDataType::from_driver(dt)),
        }
    }
}

pub struct DataTypeRef<'a> {
    inner:     DataType,
    _lifetime: PhantomData<&'a DataType>,
}

impl<'a> DataTypeRef<'a> {
    pub(crate) fn new(data_type: DriverDataTypeRef<'a>) -> Self {
        let dt = data_type.into_inner();
        Self {
            inner:     DataType::from(dt),
            _lifetime: PhantomData,
        }
    }

    pub(crate) fn inner(&self) -> &DataType {
        &self.inner
    }
}

/// A thin wrapper around the `CassDataType` object.
///
/// It is used to provide a safe interface for the `CassDataType` object freeing
/// its resources when it is dropped.
///
/// The objects of this type could be owned or borrowed. If the object is owned,
/// it will free the resources when it is dropped. If the object is borrowed, it
/// will do nothing when it is dropped.
pub(crate) struct CassDataType {
    inner: *mut struct_CassDataType_,
    owned: bool,
}

impl CassDataType {
    /// Creates a new owned `CassDataType` object.
    pub fn new(inner: *mut struct_CassDataType_) -> Self {
        Self {
            inner,
            owned: true,
        }
    }

    /// Creates a new borrowed `CassDataType` object.
    pub fn borrow(inner: *const struct_CassDataType_) -> Self {
        Self {
            inner: inner as _,
            owned: false,
        }
    }

    /// Creates a new `CassDataType` object from the given `ValueType`.
    pub fn from_value_type(value_type: ValueType) -> Self {
        let value_type = value_type.into();
        let data_type = unsafe { cass_data_type_new(value_type) };

        Self::new(data_type)
    }

    /// Creates a new `CassDataType` object from the existing one.
    pub fn from_existing(data_type: &CassDataType) -> Self {
        let inner = data_type.inner();
        let data_type = unsafe { cass_data_type_new_from_existing(inner) };

        Self::new(data_type)
    }

    /// Returns the inner `CassDataType` object for read-only operations.
    pub fn inner(&self) -> *const struct_CassDataType_ {
        self.inner as _
    }

    /// Returns the inner `CassDataType` object for mutable operations.
    pub fn inner_mut(&mut self) -> *mut struct_CassDataType_ {
        self.inner
    }
}

impl Drop for CassDataType {
    /// Frees the resources used by the data type unless the data type is
    /// borrowed.
    fn drop(&mut self) {
        if self.owned {
            unsafe { cass_data_type_free(self.inner) }
        }
    }
}
