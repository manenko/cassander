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
    /// CQL `ascii` type, which represents strings with only ASCII characters.
    Ascii,
    /// CQL `bigint` type, which represents 64-bit signed integer numbers.
    BigInt,
    /// CQL `blob` type, which represents arbitrary binary data.
    Blob,
    /// CQL `boolean` type, which represents boolean values.
    Boolean,
    /// CQL `counter` type, which represents 64-bit signed integer numbers that
    /// are incremented or decremented.
    ///
    /// Note that the value of a counter cannot be set: a counter does not
    /// exist until first incremented/decremented, and that first
    /// increment/decrement is made as if the prior value was 0.
    ///
    /// See [Cassandra documention](https://cassandra.apache.org/doc/stable/cassandra/cql/types.html#counters)
    /// for more information.
    Counter,
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
    Custom,
    /// CQL `date` type, which represents a date without a time component.
    Date,
    /// CQL `decimal` type, which represents arbitrary precision decimal numbers.
    Decimal,
    /// CQL `double` type, which represents 64-bit floating point numbers.
    Double,
    /// CQL `duration` type, which represents a period of time with nanoseconds
    /// precision.
    Duration,
    /// CQL `float` type, which represents 32-bit floating point numbers.
    Float,
    /// CQL `inet` type, which represents an IPv4 or IPv6 address.
    Inet,
    /// CQL `int` type, which represents 32-bit signed integer numbers.
    Int,
    /// CQL `list` type, which represents a (sorted) collection of non-unique
    /// values where elements are ordered by their position in the list.
    List,
    /// CQL `map` type, which represents a (sorted) set of key-value pairs,
    /// where keys are unique and the map is sorted by its keys.
    Map,
    /// CQL `set` type, which represents a (sorted) collection of unique values.
    Set,
    /// CQL `smallint` type, which represents 16-bit signed integer numbers.
    SmallInt,
    /// CQL `text` type, which represents strings with UTF-8 encoding.
    Text,
    /// CQL `time` type, which represents a time of day as the number of
    /// nanoseconds since midnight.
    Time,
    /// CQL `timeuuid` type, which represents a type 1 UUID, which is a
    /// combination of a timestamp and a unique identifier.
    ///
    /// It is generally used as a conflict-free timestamp.
    TimeUuid,
    /// CQL `timestamp` type, which represents a date and time with millisecond
    /// precision.
    Timestamp,
    /// CQL `tinyint` type, which represents 8-bit signed integer numbers.
    TinyInt,
    /// CQL `tuple` type, which represents a fixed-size collection of elements
    /// where each element can have a different type.
    ///
    /// Functionally, tuples can be though as anonymous UDT with anonymous
    /// fields.
    Tuple,
    /// CQL user-defined type aka UDT.
    ///
    /// A UDT has a name (used to declare columns of that type) and is a set of
    /// named and typed fields. Fields name can be any type, including
    /// collections or other UDT.
    ///
    /// See [Cassandra documention](https://cassandra.apache.org/doc/stable/cql/types.html#udts)
    /// for more information.
    Udt,
    /// CQL `uuid` type, which represents a type 1 or type 4 UUID.
    Uuid,
    /// CQL `varchar` type, which represents strings with UTF-8 encoding.
    VarChar,
    /// CQL `varint` type, which represents arbitrary precision integer numbers.
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
