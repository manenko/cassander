#[cfg(feature = "chrono")]
use chrono::NaiveTime;
#[cfg(feature = "chrono")]
use thiserror::Error;

/// A CQL `time` type.
///
/// Cassandra's `time` represents the number of nanoseconds since midnight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CqlTime(i64);

impl CqlTime {
    /// Creates a new [`CqlTime`] from the given nanoseconds since midnight.
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the inner nanoseconds since midnight value.
    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CqlTime {
    /// Converts the given [`i64`] into a [`CqlTime`].
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<CqlTime> for i64 {
    /// Converts the given [`CqlTime`] into an [`i64`].
    fn from(value: CqlTime) -> Self {
        value.as_i64()
    }
}

impl AsRef<i64> for CqlTime {
    /// Returns the inner nanoseconds since midnight value as a reference.
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[cfg(feature = "chrono")]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("the time {0} is out of range for CQL time type")]
pub struct CqlTimeOverflowError(NaiveTime);

#[cfg(feature = "chrono")]
impl TryFrom<NaiveTime> for CqlTime {
    type Error = CqlTimeOverflowError;

    /// Converts the given [`NaiveTime`] into a [`CqlTime`].
    fn try_from(value: NaiveTime) -> Result<Self, Self::Error> {
        let nanos = value
            .signed_duration_since(NaiveTime::MIN)
            .num_nanoseconds()
            .expect("NaiveTime::MIN is within range for num_nanoseconds");

        // The CQL time overlfows on leap seconds.
        //
        // The maximum value for a CQL time is 24 hours, which is 86399999999999
        // nanoseconds. However, `NaiveTime` supports leap second, see:
        // https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html#leap-second-handling
        if nanos > 86399999999999 {
            Err(CqlTimeOverflowError(value))
        } else {
            Ok(Self::new(nanos))
        }
    }
}

/// An error indicating that the given [`CqlTime`] is unrepresentable as
/// a [`NaiveTime`] because it is out of range..
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("the CqlTime {} is out of range for \
         the chrono::NaiveTime type",
        .0.as_i64()
)]
pub struct NaiveTimeOverflowError(CqlTime);

#[cfg(feature = "chrono")]
impl TryFrom<CqlTime> for NaiveTime {
    type Error = NaiveTimeOverflowError;

    /// Converts the given [`CqlTime`] into a [`NaiveTime`].
    fn try_from(value: CqlTime) -> Result<Self, Self::Error> {
        let nanos = value.as_i64();
        let secs = (nanos / 1_000_000_000)
            .try_into()
            .map_err(|_| NaiveTimeOverflowError(value))?;
        let nanos = (nanos % 1_000_000_000)
            .try_into()
            .map_err(|_| NaiveTimeOverflowError(value))?;

        NaiveTime::from_num_seconds_from_midnight_opt(secs, nanos)
            .ok_or(NaiveTimeOverflowError(value))
    }
}
