use crate::cql::driver::DriverDataType;
use crate::cql::ValueType;

/// A custom data type.
///
/// A custom type is a string that contains the name of Java class that
/// extends the server side `AbstractType` class and that can be loaded by
/// Cassandra.
///
/// <div class="warning">
/// Custom types exists mostly for backward compatibility purposes and their
/// usage is discouraged. Their usage is complex, not user friendly and the
/// other provided types, particularly user-defined types, should almost
/// always be enough.
/// </div>
///
/// See [Cassandra documention](https://cassandra.apache.org/doc/stable/cql/types.html#custom-types)
/// for more information.
pub struct CustomDataType(DriverDataType);

impl CustomDataType {
    /// Creates a new custom data type with the given Java class name.
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        let mut data_type = Self(DriverDataType::new(ValueType::Custom));
        data_type.set_class_name(name);

        data_type
    }

    /// Returns the Java class name of the custom data type.
    pub fn name(&self) -> &str {
        self.inner().class_name().expect(
            "driver returns an error only when the value type is not Custom",
        )
    }

    /// Creates a new custom data type from the given `DriverDataType`.
    pub(crate) fn from_driver(data_type: DriverDataType) -> Self {
        assert_eq!(
            data_type.value_type(),
            ValueType::Custom,
            "invalid data type"
        );

        Self(data_type)
    }

    /// Returns a reference to the wrapped `DriverDataType`.
    pub(crate) fn inner(&self) -> &DriverDataType {
        &self.0
    }

    /// Returns a mutable reference to the wrapped `DriverDataType`.
    pub(crate) fn inner_mut(&mut self) -> &mut DriverDataType {
        &mut self.0
    }

    /// Sets the Java class name of the custom data type.
    fn set_class_name<S>(&mut self, name: S)
    where
        S: AsRef<str>,
    {
        self.inner_mut().set_class_name(name).expect(
            "driver returns an error only when the value type is not Custom",
        );
    }
}
