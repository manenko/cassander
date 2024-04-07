#[cfg(feature = "chrono")]
use chrono::{
    DateTime,
    LocalResult::Single,
    TimeZone,
    Utc,
};
#[cfg(feature = "chrono")]
use thiserror::Error;

/// A CQL `timestamp` type.
///
/// Cassandra timestamps are 64-bit signed integers representing the number of
/// milliseconds since the Unix epoch (1970-01-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CqlTimestamp(i64);

impl CqlTimestamp {
    /// Creates a new [`CqlTimestamp`] from the given raw CQL timestamp value.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner raw CQL timestamp value.
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CqlTimestamp {
    /// Converts the given [`i64`] into a [`CqlTimestamp`].
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<CqlTimestamp> for i64 {
    /// Converts the given [`CqlTimestamp`] into an [`i64`].
    fn from(value: CqlTimestamp) -> Self {
        value.as_i64()
    }
}

impl AsRef<i64> for CqlTimestamp {
    /// Returns the inner [`i64`] value as a reference.
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(feature = "chrono")]
impl From<DateTime<Utc>> for CqlTimestamp {
    /// Converts the given [`DateTime<Utc>`] into a [`CqlTimestamp`].
    fn from(value: DateTime<Utc>) -> Self {
        value.timestamp_millis().into()
    }
}

#[cfg(feature = "chrono")]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error(
    "the CqlTimestamp value {} is out of range for the \
     chrono::DateTime<chrono::Utc> type",
    .0.as_i64()
)]
pub struct DateTimeOverlfowError(CqlTimestamp);

#[cfg(feature = "chrono")]
impl TryFrom<CqlTimestamp> for DateTime<Utc> {
    type Error = DateTimeOverlfowError;

    /// Converts the given [`CqlTimestamp`] into a [`DateTime<Utc>`].
    fn try_from(value: CqlTimestamp) -> Result<Self, Self::Error> {
        let timestamp = value.as_i64();

        match Utc.timestamp_millis_opt(timestamp) {
            Single(datetime) => Ok(datetime),
            _ => Err(DateTimeOverlfowError(value)),
        }
    }
}
