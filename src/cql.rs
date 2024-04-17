//! CQL types and utilities.
//!
//! # CQL Data Types
//!
//! CQL is a typed language which supports a few
//! [data types](https://cassandra.apache.org/doc/stable/cassandra/cql/types.html).
//!
//! The [`ValueType`] enum represents a type of a CQL value, [`ValueKind`]
//! represents its kind (native, custom, collection, tuple, user-defined). The
//! [`DataType`] provides runtime information about the CQL data type. It
//! also can be used to create CQL data types.
//!
//! ```text
//! +---------+           +-----------+
//! |ValueType|           | DataType  |
//! +---------+           +-----------+
//! |    Ascii|           |     Native|--->NativeDataType
//! |   BigInt|           |     Custom|--->CustomDataType
//! |     Blob|           |    Indexed|--->IndexedDataType
//! |  Boolean|           |        Map|--->MapDataType
//! |  Counter|           |      Tuple|--->TupleDataType
//! |     Date|           |UserDefined|--->UserDefinedDataType
//! |  Decimal|           +-----------+
//! |   Double|                 |
//! | Duration|                 |           +-----------------+
//! |    Float|                 v       +-->|native(type)     |
//! |     Inet|            +---------+  +-->|custom()         |
//! |      Int|<-----+     |builder()|--+-->|indexed(set/list)|
//! | SmallInt|      |     +---------+  +-->|map()            |
//! |     Text|      |                  +-->|tuple()          |
//! |     Time|      |                  +-->|udt()            |
//! | TimeUuid|      |                      +-----------------+
//! |Timestamp|      |
//! |  TinyInt|      |
//! |     Uuid|      |
//! |  VarChar|      |
//! |   VarInt|      |
//! |         |      |
//! |         |      |  +-----------+
//! |   Custom|<--+  |  | ValueKind |
//! |         |   |  |  +-----------+
//! |         |   |  +--|Native     |
//! |     List|   +-----|Custom     |
//! |      Map|<--------|Collection |
//! |      Set|   +-----|Tuple      |
//! |         |   |  +--|UserDefined|
//! |         |   |  |  +-----------+
//! |    Tuple|<--+  |
//! |         |      |
//! |         |      |
//! |      Udt|<-----+
//! +---------+
//! ```

mod ascii;
mod collection;
mod counter;
mod data_type;
mod date;
mod decimal;
mod duration;
mod inet;
mod time;
mod timestamp;
mod tuple;
mod uuid;
mod uuid_gen;
mod value;
mod value_type;
mod varint;

pub use ascii::*;
pub use collection::*;
pub use counter::*;
pub use data_type::*;
pub use date::*;
pub use decimal::*;
pub use duration::*;
pub use inet::*;
pub use time::*;
pub use timestamp::*;
pub use tuple::*;
pub use uuid::*;
pub use uuid_gen::*;
pub use value::*;
pub use value_type::*;
pub use varint::*;
