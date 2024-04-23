use crate::cql::driver::DriverDataType;
use crate::cql::{
    ValueKind,
    ValueType,
};
use crate::{
    DriverError,
    DriverErrorKind,
};

/// A native data type.
///
/// A native CQL type is one of the following:
///
/// - `ascii`
/// - `bigint`
/// - `blob`
/// - `boolean`
/// - `counter`
/// - `date`
/// - `decimal`
/// - `double`
/// - `duration`
/// - `float`
/// - `inet`
/// - `int`
/// - `smallint`
/// - `text`
/// - `time`
/// - `timestamp`
/// - `timeuuid`
/// - `tinyint`
/// - `uuid`
/// - `varchar`
/// - `varint`
///
/// See [Cassandra documention](https://cassandra.apache.org/doc/stable/cql/types.html#native-types)
/// for more information.
pub struct NativeDataType(DriverDataType);

impl NativeDataType {
    /// Creates a new native data type using the provided value type.
    ///
    /// Returns an error if the value type is not a native type.
    pub fn new(value_type: ValueType) -> Result<Self, DriverError> {
        if value_type.kind() != ValueKind::Native {
            return Err(DriverError::with_message(
                DriverErrorKind::LibBadParams,
                format!("the {:?} is not a native type", value_type),
            ));
        }

        Ok(Self(DriverDataType::new(value_type)))
    }

    /// Returns the `ascii` native data type.
    pub fn ascii() -> Self {
        Self(DriverDataType::new(ValueType::Ascii))
    }

    /// Returns the `bigint` native data type.
    pub fn bigint() -> Self {
        Self(DriverDataType::new(ValueType::BigInt))
    }

    /// Returns the `blob` native data type.
    pub fn blob() -> Self {
        Self(DriverDataType::new(ValueType::Blob))
    }

    /// Returns the `boolean` native data type.
    pub fn boolean() -> Self {
        Self(DriverDataType::new(ValueType::Boolean))
    }

    /// Returns the `counter` native data type.
    pub fn counter() -> Self {
        Self(DriverDataType::new(ValueType::Counter))
    }

    /// Returns the `date` native data type.
    pub fn date() -> Self {
        Self(DriverDataType::new(ValueType::Date))
    }

    /// Returns the `decimal` native data type.
    pub fn decimal() -> Self {
        Self(DriverDataType::new(ValueType::Decimal))
    }

    /// Returns the `double` native data type.
    pub fn double() -> Self {
        Self(DriverDataType::new(ValueType::Double))
    }

    /// Returns the `duration` native data type.
    pub fn duration() -> Self {
        Self(DriverDataType::new(ValueType::Duration))
    }

    /// Returns the `float` native data type.
    pub fn float() -> Self {
        Self(DriverDataType::new(ValueType::Float))
    }

    /// Returns the `inet` native data type.
    pub fn inet() -> Self {
        Self(DriverDataType::new(ValueType::Inet))
    }

    /// Returns the `int` native data type.
    pub fn int() -> Self {
        Self(DriverDataType::new(ValueType::Int))
    }

    /// Returns the `smallint` native data type.
    pub fn smallint() -> Self {
        Self(DriverDataType::new(ValueType::SmallInt))
    }

    /// Returns the `text` native data type.
    pub fn text() -> Self {
        Self(DriverDataType::new(ValueType::Text))
    }

    /// Returns the `time` native data type.
    pub fn time() -> Self {
        Self(DriverDataType::new(ValueType::Time))
    }

    /// Returns the `timeuuid` native data type.
    pub fn timeuuid() -> Self {
        Self(DriverDataType::new(ValueType::TimeUuid))
    }

    /// Returns the `timestamp` native data type.
    pub fn timestamp() -> Self {
        Self(DriverDataType::new(ValueType::Timestamp))
    }

    /// Returns the `tinyint` native data type.
    pub fn tinyint() -> Self {
        Self(DriverDataType::new(ValueType::TinyInt))
    }

    /// Returns the `uuid` native data type.
    pub fn uuid() -> Self {
        Self(DriverDataType::new(ValueType::Uuid))
    }

    /// Returns the `varchar` native data type.
    pub fn varchar() -> Self {
        Self(DriverDataType::new(ValueType::VarChar))
    }

    /// Returns the value type of the native data type.
    pub fn value_type(&self) -> ValueType {
        self.inner().value_type()
    }

    pub(crate) fn inner(&self) -> &DriverDataType {
        &self.0
    }

    pub(crate) fn inner_mut(&mut self) -> &mut DriverDataType {
        &mut self.0
    }
}
