use crate::cql::{
    CqlAscii,
    CqlCounter,
    CqlDate,
    CqlDecimal,
    CqlDuration,
    CqlInet,
    CqlTime,
    CqlTimestamp,
    CqlUuid,
    CqlValueType,
    CqlVarInt,
};

pub enum CqlValue {
    Ascii(CqlAscii),
    BigInt(i64),
    Blob(Vec<u8>),
    Boolean(bool),
    Counter(CqlCounter),
    Date(CqlDate),
    Decimal(CqlDecimal),
    Double(f64),
    Duration(CqlDuration),
    Float(f32),
    Inet(CqlInet),
    Int(i32),
    //List(CqlList),
    //Map(CqlMap),
    //Set(CqlSet),
    SmallInt(i16),
    Text(String),
    Time(CqlTime),
    TimeUuid(CqlUuid),
    Timestamp(CqlTimestamp),
    TinyInt(i8),
    //Tuple(CqlTuple),
    //Udt(CqlUdt),
    Uuid(CqlUuid),
    VarChar(String),
    VarInt(CqlVarInt),
}

impl CqlValue {
    /// Returns this value as a string slice if it is a text-based value, i.e.
    /// CQL `ascii`, `text`, or `varchar`.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            CqlValue::Text(value) => Some(value.as_str()),
            CqlValue::VarChar(value) => Some(value.as_str()),
            CqlValue::Ascii(value) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns this value as a slice of bytes if it is a CQL `blob`.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            CqlValue::Blob(value) => Some(value),
            _ => None,
        }
    }

    /// Returns this value as a [`bool`] if it is a CQL `boolean`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CqlValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlCounter`] if it is a CQL `counter`.
    pub fn as_counter(&self) -> Option<CqlCounter> {
        match self {
            CqlValue::Counter(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlDate`] if it is a CQL `date`.
    pub fn as_date(&self) -> Option<CqlDate> {
        match self {
            CqlValue::Date(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlDecimal`] if it is a CQL `decimal`.
    pub fn as_decimal(&self) -> Option<&CqlDecimal> {
        match self {
            CqlValue::Decimal(value) => Some(value),
            _ => None,
        }
    }

    /// Returns this value as an [`f32`] if it is a CQL `float`.
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            CqlValue::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as an [`f64`] if it is a CQL `double`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            CqlValue::Double(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlDuration`] if it is a CQL `duration`.
    pub fn as_duration(&self) -> Option<CqlDuration> {
        match self {
            CqlValue::Duration(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlInet`] if it is a CQL `inet`.
    pub fn as_inet(&self) -> Option<CqlInet> {
        match self {
            CqlValue::Inet(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as an [`i16`] if it is a CQL `smallint`.
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            CqlValue::SmallInt(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as an [`i32`] if it is a CQL `int`.
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            CqlValue::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as an [`i64`] if it is a CQL `bigint`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            CqlValue::BigInt(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlTime`] if it is a CQL `time`.
    pub fn as_time(&self) -> Option<CqlTime> {
        match self {
            CqlValue::Time(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlTimestamp`] if it is a CQL `timestamp`.
    pub fn as_timestamp(&self) -> Option<CqlTimestamp> {
        match self {
            CqlValue::Timestamp(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlUuid`] if it is a CQL UUID type, i.e.
    /// `timeuuid` or `uuid`.
    pub fn as_uuid(&self) -> Option<CqlUuid> {
        match self {
            CqlValue::TimeUuid(value) => Some(*value),
            CqlValue::Uuid(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns this value as a [`CqlVarInt`] if it is a CQL `varint`.
    pub fn as_var_int(&self) -> Option<&CqlVarInt> {
        match self {
            CqlValue::VarInt(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the [`CqlValueType`] of this value.
    #[rustfmt::skip]
    pub fn value_type(&self) -> CqlValueType {
        match self {
            CqlValue::Ascii(_)     => CqlValueType::Ascii,
            CqlValue::BigInt(_)    => CqlValueType::BigInt,
            CqlValue::Blob(_)      => CqlValueType::Blob,
            CqlValue::Boolean(_)   => CqlValueType::Boolean,
            CqlValue::Counter(_)   => CqlValueType::Counter,
            CqlValue::Date(_)      => CqlValueType::Date,
            CqlValue::Decimal(_)   => CqlValueType::Decimal,
            CqlValue::Double(_)    => CqlValueType::Double,
            CqlValue::Duration(_)  => CqlValueType::Duration,
            CqlValue::Float(_)     => CqlValueType::Float,
            CqlValue::Inet(_)      => CqlValueType::Inet,
            CqlValue::Int(_)       => CqlValueType::Int,
            CqlValue::SmallInt(_)  => CqlValueType::SmallInt,
            CqlValue::Text(_)      => CqlValueType::Text,
            CqlValue::Time(_)      => CqlValueType::Time,
            CqlValue::TimeUuid(_)  => CqlValueType::TimeUuid,
            CqlValue::Timestamp(_) => CqlValueType::Timestamp,
            CqlValue::TinyInt(_)   => CqlValueType::TinyInt,
            CqlValue::Uuid(_)      => CqlValueType::Uuid,
            CqlValue::VarChar(_)   => CqlValueType::VarChar,
            CqlValue::VarInt(_)    => CqlValueType::VarInt,
        }
    }
}

impl From<CqlAscii> for CqlValue {
    fn from(value: CqlAscii) -> Self {
        CqlValue::Ascii(value)
    }
}

impl From<i8> for CqlValue {
    fn from(value: i8) -> Self {
        CqlValue::TinyInt(value)
    }
}

impl From<i16> for CqlValue {
    fn from(value: i16) -> Self {
        CqlValue::SmallInt(value)
    }
}

impl From<i32> for CqlValue {
    fn from(value: i32) -> Self {
        CqlValue::Int(value)
    }
}

impl From<i64> for CqlValue {
    fn from(value: i64) -> Self {
        CqlValue::BigInt(value)
    }
}

impl From<f32> for CqlValue {
    fn from(value: f32) -> Self {
        CqlValue::Float(value)
    }
}

impl From<f64> for CqlValue {
    fn from(value: f64) -> Self {
        CqlValue::Double(value)
    }
}

impl From<bool> for CqlValue {
    fn from(value: bool) -> Self {
        CqlValue::Boolean(value)
    }
}

impl From<Vec<u8>> for CqlValue {
    fn from(value: Vec<u8>) -> Self {
        CqlValue::Blob(value)
    }
}

impl From<&[u8]> for CqlValue {
    fn from(value: &[u8]) -> Self {
        CqlValue::Blob(value.to_vec())
    }
}

impl From<String> for CqlValue {
    fn from(value: String) -> Self {
        CqlValue::Text(value)
    }
}

impl From<&str> for CqlValue {
    fn from(value: &str) -> Self {
        CqlValue::Text(value.to_string())
    }
}

impl From<&String> for CqlValue {
    fn from(value: &String) -> Self {
        CqlValue::Text(value.clone())
    }
}

impl From<CqlCounter> for CqlValue {
    fn from(value: CqlCounter) -> Self {
        CqlValue::Counter(value)
    }
}

impl From<CqlDate> for CqlValue {
    fn from(value: CqlDate) -> Self {
        CqlValue::Date(value)
    }
}

impl From<CqlDecimal> for CqlValue {
    fn from(value: CqlDecimal) -> Self {
        CqlValue::Decimal(value)
    }
}

impl From<CqlDuration> for CqlValue {
    fn from(value: CqlDuration) -> Self {
        CqlValue::Duration(value)
    }
}

impl From<CqlInet> for CqlValue {
    fn from(value: CqlInet) -> Self {
        CqlValue::Inet(value)
    }
}

impl From<CqlTime> for CqlValue {
    fn from(value: CqlTime) -> Self {
        CqlValue::Time(value)
    }
}

impl From<CqlTimestamp> for CqlValue {
    fn from(value: CqlTimestamp) -> Self {
        CqlValue::Timestamp(value)
    }
}

impl From<CqlUuid> for CqlValue {
    fn from(value: CqlUuid) -> Self {
        CqlValue::Uuid(value)
    }
}

impl From<CqlVarInt> for CqlValue {
    fn from(value: CqlVarInt) -> Self {
        CqlValue::VarInt(value)
    }
}
