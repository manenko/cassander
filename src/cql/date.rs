#[cfg(feature = "chrono")]
use chrono::Duration;
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
use thiserror::Error;

/// A CQL `date` type which represents a date without a time zone.
///
/// Cassandra encodes dates as a 32-bit integers representing a number of days
/// with the epoch at the center of the range (2^31). The epoch is 1970-01-01.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CqlDate(u32);

impl CqlDate {
    /// Creates a new [`CqlDate`] from the given raw CQL date value.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the inner raw CQL date value.
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for CqlDate {
    /// Converts the given [`u32`] into a [`CqlDate`].
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<CqlDate> for u32 {
    /// Converts the given [`CqlDate`] into an [`u32`].
    fn from(value: CqlDate) -> Self {
        value.as_u32()
    }
}

/// The CQL epoch is in the middle of the range of the 32-bit integer, which is
/// 2 ^ 31.
#[cfg(feature = "chrono")]
const CQL_DATE_EPOCH: i64 = 1 << 31; // 2147483648

#[cfg(feature = "chrono")]
impl From<NaiveDate> for CqlDate {
    /// Converts the given [`NaiveDate`] into a [`CqlDate`].
    fn from(value: NaiveDate) -> Self {
        let unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .expect("Unix Epoch is a valid date");

        // let max_days = NaiveDate::MAX.signed_duration_since(unix_epoch)
        //                              .num_days();
        // let min_days = NaiveDate::MIN.signed_duration_since(unix_epoch)
        //                              .num_days();
        //
        // - MAX days since Unix Epoch: +95026236
        // - MIN days since Unix Epoch: -96465292
        //
        // We add CQL_DATE_EPOCH to the days from the Unix Epoch to get the CQL
        // date value. Which means the range of the CQL date is:
        //
        // - MAX CQL date: 2242509884 days
        // - MIN CQL date: 2051018356 days
        //
        // This is within the range of the 32-bit integer and hence the value
        // overflow is impossible.

        let days = value.signed_duration_since(unix_epoch).num_days();
        let days = (days + CQL_DATE_EPOCH) as u32;

        Self::new(days)
    }
}

/// Error type for when a CQL date value is out of range for a [`NaiveDate`].
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("CQL date value {} is out of range for chrono::NaiveDate", .0.as_u32())]
pub struct CqlDateOverflowError(CqlDate);

#[cfg(feature = "chrono")]
impl TryFrom<CqlDate> for NaiveDate {
    type Error = CqlDateOverflowError;

    /// Converts the given [`CqlDate`] into a [`NaiveDate`].
    fn try_from(value: CqlDate) -> Result<Self, Self::Error> {
        let days = value.as_u32() as i64 - CQL_DATE_EPOCH;

        // If the value was 0, then the days is -2147483648.
        // If the value was 4294967295, then the days is 2147483647.
        // Both of these values are in the range of the Duration.
        let duration = Duration::try_days(days)
            .expect("CQL days always fit the chrono::Duration");

        NaiveDate::from_ymd_opt(1970, 1, 1)
            .expect("Unix Epoch is a valid date")
            .checked_add_signed(duration)
            .ok_or(CqlDateOverflowError(value))
    }
}
