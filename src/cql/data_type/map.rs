use crate::cql::driver::DriverDataType;
use crate::cql::{
    DataType,
    ValueType,
};

pub struct MapDataType(DriverDataType);

impl MapDataType {
    /// Creates a new CQL `map` data type.
    pub fn new<K, V>(key_type: K, value_type: V) -> Self
    where
        K: AsRef<DataType>,
        V: AsRef<DataType>,
    {
        let key_dt = DriverDataType::from_existing(key_type.as_ref().inner());
        let value_dt = DriverDataType::from_existing(value_type);

        let mut data_type = DriverDataType::new(ValueType::Map);
        data_type.add_sub_type(key_dt);
        data_type.add_sub_type(value_dt);

        data_type
    }

    /// Returns the key type of the map data type.
    pub fn key_type(&self) -> &DriverDataType {
        self.inner().sub_data_type_by_index(0).expect(
            "driver returns an error only when the value type is not Map",
        )
    }

    /// Returns the value type of the map data type.
    pub fn value_type(&self) -> &DriverDataType {
        self.inner().sub_data_type_by_index(1).expect(
            "driver returns an error only when the value type is not Map",
        )
    }

    /// Creates a new map data type from the given `DriverDataType`.
    pub(crate) fn from_driver(data_type: DriverDataType) -> Self {
        assert_eq!(data_type.value_type(), ValueType::Map, "invalid data type");

        Self(data_type)
    }

    /// Returns a reference to the wrapped `DriverDataType`.
    pub(crate) fn inner(&self) -> &DriverDataType {
        &self.0
    }
}
