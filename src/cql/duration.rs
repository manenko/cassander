/// A CQL `duration` type.
///
/// Cassandra `duration` type stores stores months, days, and seconds separately
/// due to the fact that the number of days in a month varies, and a day can
/// have 23 or 25 hours if a daylight saving is involved.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CqlDuration {
    /// The number of months.
    months:      i32,
    /// The number of days.
    days:        i32,
    /// The number of nanoseconds.
    nanoseconds: i64,
}

// NB: The C++ driver does not check months, days, and nanoseconds but the Java
// driver does. It expects all values being negative or positive at the same:
// https://github.com/datastax/java-driver-pre-donation/blob/4.x/core/src/main/java/com/datastax/oss/driver/api/core/data/CqlDuration.java#L89
//
// We follow the C++ driver behavior and do not check the values.

impl CqlDuration {
    /// Creates a new [`CqlDuration`] from the given months, days, and
    /// nanoseconds.
    pub fn new(months: i32, days: i32, nanoseconds: i64) -> Self {
        Self {
            months,
            days,
            nanoseconds,
        }
    }

    /// Returns the number of months.
    pub fn months(&self) -> i32 {
        self.months
    }

    /// Returns the number of days.
    pub fn days(&self) -> i32 {
        self.days
    }

    /// Returns the number of nanoseconds.
    pub fn nanoseconds(&self) -> i64 {
        self.nanoseconds
    }
}

// TODO: Into<String> for CqlDuration
// TODO: FromStr for CqlDuration
// TODO: Display for CqlDuration
