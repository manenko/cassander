#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "bigdecimal")]
use thiserror::Error;

use crate::cql::CqlVarInt;

/// A CQL decimal.
///
/// Cassandra encodes decimal values as a pair of a magnitude ([`CqlVarInt`])
/// and an exponent.
///
/// This type is not much useful on its own if you need to perform operations
/// over the decimal values. However, when `bigdecimal` feature is enabled, the
/// crate implements conversions between the `CqlDecimal` and the `BigDecimal`
/// type from the [`bigdecimal`](https://docs.rs/bigdecimal/latest/bigdecimal/)
/// crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CqlDecimal {
    magnitude: CqlVarInt,
    exponent:  i32,
}

impl CqlDecimal {
    /// Creates a new [`CqlDecimal`] from the given magnitude representing
    /// [`CqlVarInt`] and exponent.
    pub fn from_signed_bytes_be_and_exponent<B>(
        magnitude: B,
        exponent: i32,
    ) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self {
            magnitude: CqlVarInt::from_signed_bytes_be(magnitude),
            exponent,
        }
    }

    /// Returns the magnitude of the [`CqlDecimal`] as a reference to the
    /// [`CqlVarInt`].
    pub fn magnitude(&self) -> &CqlVarInt {
        &self.magnitude
    }

    /// Returns the exponent of the [`CqlDecimal`].
    pub fn exponent(&self) -> i32 {
        self.exponent
    }

    /// Converts the [`CqlDecimal`] into a tuple of the magnitude as a vector of
    /// bytes in two's complement big-endian binary representation and the
    /// exponent.
    pub fn into_signed_bytes_be_and_exponent(self) -> (Vec<u8>, i32) {
        (self.magnitude.into_signed_bytes_be(), self.exponent)
    }

    /// Returns a tuple of the magnitude as a reference to the underlying two's
    /// complement binary in big-endian order and the exponent.
    pub fn as_signed_bytes_be_and_exponent(&self) -> (&[u8], i32) {
        (self.magnitude.as_signed_bytes_be(), self.exponent)
    }
}

#[cfg(feature = "bigdecimal")]
impl From<CqlDecimal> for BigDecimal {
    /// Converts the given [`CqlDecimal`] into a [`BigDecimal`].
    fn from(value: CqlDecimal) -> Self {
        use bigdecimal::num_bigint::BigInt;

        let magnitude = BigInt::from_signed_bytes_be(
            value.magnitude().as_signed_bytes_be(),
        );

        let exponent = value.exponent() as i64;

        Self::new(magnitude, exponent)
    }
}

/// An error indicating that the given `BigDecimal` is unrepresentable as
/// [`CqlDecimal`] because its decimal exponent overflows.
///
/// The Cassandra decimal type has a 32-bit signed integer exponent, while the
/// `BigDecimal` type from the `bigdecimal` crate has a 64-bit signed integer
/// exponent.
#[cfg(feature = "bigdecimal")]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "the given bidecimal::BigDecimal is unrepresentable as CqlDecimal,
        because its decimal exponent overflows: {0}"
)]
pub struct DecimalExponentOverflowError(BigDecimal);

#[cfg(feature = "bigdecimal")]
impl TryFrom<BigDecimal> for CqlDecimal {
    type Error = DecimalExponentOverflowError;

    /// Converts the given `BigDecimal` into a [`CqlDecimal`].
    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        let (magnitude, exponent) = value.as_bigint_and_exponent();

        let magnitude =
            CqlVarInt::from_signed_bytes_be(magnitude.to_signed_bytes_be());

        let exponent = exponent
            .try_into()
            .map_err(|_| DecimalExponentOverflowError(value))?;

        Ok(Self {
            magnitude,
            exponent,
        })
    }
}
