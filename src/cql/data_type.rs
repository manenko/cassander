pub(crate) mod driver;

mod custom;
mod indexed;
mod map;
mod native;
mod tuple;
mod udt;

pub use custom::*;
pub use indexed::*;
pub use map::*;
pub use native::*;
pub use tuple::*;
pub use udt::*;

use crate::cql::driver::DriverDataType;

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

    pub(crate) fn inner(&self) -> &DriverDataType {
        match self {
            DataType::Custom(custom) => custom.inner(),
            DataType::Indexed(indexed) => indexed.inner(),
            DataType::Map(map) => map.inner(),
            DataType::Native(native) => native.inner(),
            DataType::Tuple(tuple) => tuple.inner(),
            DataType::Udt(udt) => udt.inner(),
        }
    }

    pub(crate) fn inner_mut(&mut self) -> &mut DriverDataType {
        match self {
            DataType::Custom(custom) => custom.inner_mut(),
            DataType::Indexed(indexed) => indexed.inner_mut(),
            DataType::Map(map) => map.inner_mut(),
            DataType::Native(native) => native.inner_mut(),
            DataType::Tuple(tuple) => tuple.inner_mut(),
            DataType::Udt(udt) => udt.inner_mut(),
        }
    }
}

pub struct DataTypeRef<'a>(&'a DriverDataType);

impl<'a> DataTypeRef<'a> {
    pub(crate) fn new<T>(data_type: T) -> Self
    where
        T: AsRef<DriverDataType>,
    {
        Self(data_type.as_ref())
    }

    pub(crate) fn inner(&self) -> &DriverDataType {
        self.0
    }
}
