use crate::cql::driver::{
    DriverDataType,
    DriverDataTypeRef,
};
use crate::cql::{
    DataType,
    DataTypeRef,
    ValueType,
};

/// A `list` or `set` data type.
pub struct IndexedDataType(DriverDataType);

impl IndexedDataType {
    /// Creates a new `list` data type.
    pub fn new_list<D>(element_type: D) -> Self
    where
        D: AsRef<DataType>,
    {
        Self::new(ValueType::List, element_type)
    }

    /// Creates a new `set` data type.
    pub fn new_set<D>(element_type: D) -> Self
    where
        D: AsRef<DataType>,
    {
        Self::new(ValueType::Set, element_type)
    }

    /// Returns the value type of the indexed data type.
    pub fn value_type(&self) -> ValueType {
        self.inner().value_type()
    }

    /// Returns the element type of the indexed data type.
    pub fn element_type(&self) -> DataTypeRef<'_> {
        let element_type = self.inner().sub_data_type_by_index(0).expect(
            "driver returns an error only when the value type is not List or \
             Set",
        );

        DataTypeRef::new(element_type)
    }

    /// Creates a new indexed data type from the given `DriverDataType`.
    pub(crate) fn from_driver(data_type: DriverDataType) -> Self {
        assert!(
            matches!(data_type.value_type(), ValueType::List | ValueType::Set),
            "invalid data type",
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

    /// Creates a new indexed data type with the given value type and element
    /// type.
    fn new<D>(value_type: ValueType, element_type: D) -> Self
    where
        D: AsRef<DataType>,
    {
        let mut data_type = Self(DriverDataType::new(value_type));

        let cass_type = element_type.as_ref().inner().inner();
        let element_type = DriverDataTypeRef::new(cass_type);
        data_type.inner_mut().add_sub_type(element_type);

        data_type
    }
}
