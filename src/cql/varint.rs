#[cfg(feature = "num-bigint")]
use num_bigint::BigInt;

/// A CQL `varint` type.
///
/// Cassandra represents `varint` as two's-complement binary in big-endian
/// order. The data is not normalized, so the leading bytes may be zero.
///
/// This type is not much useful on its own if you need to perform operations
/// over the `varint` values. However, when `num-bigint` feature is enabled, the
/// crate implements conversions between the `CqlVarInt` and the `BigInt` type
/// from the
/// [`num-bigint`](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html)
/// crate.
#[derive(Debug, Clone)]
pub struct CqlVarInt(Vec<u8>);

impl CqlVarInt {
    /// Creates a new [`CqlVarInt`] from the given bytes in two's complement
    /// big-endian binary representation.
    pub fn from_signed_bytes_be<B>(bytes: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self(bytes.into())
    }

    /// Converts the [`CqlVarInt`] into a vector of bytes in two's complement
    /// big-endian binary representation.
    pub fn into_signed_bytes_be(self) -> Vec<u8> {
        self.0
    }

    /// Returns a slice of the [`CqlVarInt`] bytes in two's complement
    /// big-endian binary representation.
    pub fn as_signed_bytes_be(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "num-bigint")]
impl From<BigInt> for CqlVarInt {
    /// Converts the given [`BigInt`] into a [`CqlVarInt`].
    fn from(value: BigInt) -> Self {
        let bytes = value.to_signed_bytes_be();

        Self::from_signed_bytes_be(bytes)
    }
}

#[cfg(feature = "num-bigint")]
impl From<CqlVarInt> for BigInt {
    /// Converts the given [`CqlVarInt`] into a [`BigInt`].
    fn from(value: CqlVarInt) -> Self {
        BigInt::from_signed_bytes_be(value.as_signed_bytes_be())
    }
}
