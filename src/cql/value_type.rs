#[rustfmt::skip]
use crate::ffi::{
    enum_CassValueType_,
    enum_CassValueType__CASS_VALUE_TYPE_ASCII     as ASCII,
    enum_CassValueType__CASS_VALUE_TYPE_BIGINT    as BIGINT,
    enum_CassValueType__CASS_VALUE_TYPE_BLOB      as BLOB,
    enum_CassValueType__CASS_VALUE_TYPE_BOOLEAN   as BOOLEAN,
    enum_CassValueType__CASS_VALUE_TYPE_COUNTER   as COUNTER,
    enum_CassValueType__CASS_VALUE_TYPE_CUSTOM    as CUSTOM,
    enum_CassValueType__CASS_VALUE_TYPE_DATE      as DATE,
    enum_CassValueType__CASS_VALUE_TYPE_DECIMAL   as DECIMAL,
    enum_CassValueType__CASS_VALUE_TYPE_DOUBLE    as DOUBLE,
    enum_CassValueType__CASS_VALUE_TYPE_DURATION  as DURATION,
    enum_CassValueType__CASS_VALUE_TYPE_FLOAT     as FLOAT,
    enum_CassValueType__CASS_VALUE_TYPE_INET      as INET,
    enum_CassValueType__CASS_VALUE_TYPE_INT       as INT,
    enum_CassValueType__CASS_VALUE_TYPE_LIST      as LIST,
    enum_CassValueType__CASS_VALUE_TYPE_MAP       as MAP,
    enum_CassValueType__CASS_VALUE_TYPE_SET       as SET,
    enum_CassValueType__CASS_VALUE_TYPE_SMALL_INT as SMALL_INT,
    enum_CassValueType__CASS_VALUE_TYPE_TEXT      as TEXT,
    enum_CassValueType__CASS_VALUE_TYPE_TIME      as TIME,
    enum_CassValueType__CASS_VALUE_TYPE_TIMESTAMP as TIMESTAMP,
    enum_CassValueType__CASS_VALUE_TYPE_TIMEUUID  as TIMEUUID,
    enum_CassValueType__CASS_VALUE_TYPE_TINY_INT  as TINY_INT,
    enum_CassValueType__CASS_VALUE_TYPE_TUPLE     as TUPLE,
    enum_CassValueType__CASS_VALUE_TYPE_UDT       as UDT,
    enum_CassValueType__CASS_VALUE_TYPE_UUID      as UUID,
    enum_CassValueType__CASS_VALUE_TYPE_VARCHAR   as VARCHAR,
    enum_CassValueType__CASS_VALUE_TYPE_VARINT    as VARINT,
};

/// A type of a CQL value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Ascii,
    BigInt,
    Blob,
    Boolean,
    Counter,
    Custom,
    Date,
    Decimal,
    Double,
    Duration,
    Float,
    Inet,
    Int,
    List,
    Map,
    Set,
    SmallInt,
    Text,
    Time,
    TimeUuid,
    Timestamp,
    TinyInt,
    Tuple,
    Udt,
    Uuid,
    VarChar,
    VarInt,
}

impl ValueType {
    /// Returns the kind of the value type (native, collection, UDT, etc.).
    #[rustfmt::skip]
    pub fn kind(self) -> ValueKind {
        use ValueType::*;

        match self {
            |  Ascii
            |  BigInt
            |  Blob
            |  Boolean
            |  Counter
            |  Date
            |  Decimal
            |  Double
            |  Duration
            |  Float
            |  Inet
            |  Int
            |  SmallInt
            |  Text
            |  Time
            |  TimeUuid
            |  Timestamp
            |  TinyInt
            |  Uuid
            |  VarChar
            |  VarInt
            => ValueKind::Native,

            |  Custom
            => ValueKind::Custom,

            |  List
            |  Map
            |  Set
            => ValueKind::Collection,

            |  Tuple
            => ValueKind::Tuple,

            |  Udt
            => ValueKind::UserDefined,
        }
    }
}

impl From<enum_CassValueType_> for ValueType {
    #[rustfmt::skip]
    fn from(value_type: enum_CassValueType_) -> Self {
        use ValueType::*;

        match value_type {
            ASCII       => Ascii,
            BIGINT      => BigInt,
            BLOB        => Blob,
            BOOLEAN     => Boolean,
            COUNTER     => Counter,
            CUSTOM      => Custom,
            DATE        => Date,
            DECIMAL     => Decimal,
            DOUBLE      => Double,
            DURATION    => Duration,
            FLOAT       => Float,
            INET        => Inet,
            INT         => Int,
            LIST        => List,
            MAP         => Map,
            SET         => Set,
            SMALL_INT   => SmallInt,
            TEXT        => Text,
            TIME        => Time,
            TIMESTAMP   => Timestamp,
            TIMEUUID    => TimeUuid,
            TINY_INT    => TinyInt,
            TUPLE       => Tuple,
            UDT         => Udt,
            UUID        => Uuid,
            VARCHAR     => VarChar,
            VARINT      => VarInt,
            u           => unreachable!("unknown CassValueType: {}", u)
        }
    }
}

impl From<ValueType> for enum_CassValueType_ {
    #[rustfmt::skip]
    fn from(value_type: ValueType) -> Self {
        use ValueType::*;

        match value_type {
            Ascii            => ASCII,
            BigInt           => BIGINT,
            Blob             => BLOB,
            Boolean          => BOOLEAN,
            Counter          => COUNTER,
            Custom           => CUSTOM,
            Date             => DATE,
            Decimal          => DECIMAL,
            Double           => DOUBLE,
            Duration         => DURATION,
            Float            => FLOAT,
            Inet             => INET,
            Int              => INT,
            List             => LIST,
            Map              => MAP,
            Set              => SET,
            SmallInt         => SMALL_INT,
            Text             => TEXT,
            Time             => TIME,
            TimeUuid         => TIMEUUID,
            Timestamp        => TIMESTAMP,
            TinyInt          => TINY_INT,
            Tuple            => TUPLE,
            Udt              => UDT,
            Uuid             => UUID,
            VarChar          => VARCHAR,
            VarInt           => VARINT,
        }
    }
}

/// A kind of a CQL value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    /// One of the CQL native types.
    Native,
    /// A custom type.
    Custom,
    /// A CQL collection type.
    Collection,
    /// A CQL tuple.
    Tuple,
    /// A CQL user-defined type.
    UserDefined,
}
