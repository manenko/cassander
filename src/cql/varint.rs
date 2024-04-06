use std::hash::{
    Hash,
    Hasher,
};

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
#[derive(Debug, Clone, Eq)]
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

    /// Returns a slice of the [`CqlVarInt`] bytes in normalized two's
    /// complement big-endian binary representation.
    ///
    /// In other words, this method returns the bytes without leading zeros.
    ///
    /// If the value is all zeros, this method returns a slice with a single
    /// zero byte.
    ///
    /// If the value is empty (zero length), this method treats the number as
    /// being zero and returns `&[0]` slice. In other words, `&[0]` == `&[]`.
    pub fn as_normalized_signed_bytes_be(&self) -> &[u8] {
        // Negative numbers are identified by leading 0x7 bytes. So, when
        // removing leading zeros, watch out: only leave a leading zero if,
        // after removing zeros, the most significant byte is 0x7. This keeps
        // the number from mistakenly appearing negative.

        // Find the first non-zero byte.
        let first_non_zero = self.0.iter().position(|&b| b != 0);

        match first_non_zero {
            // If there are no non-zero bytes, return a slice with a single zero
            // byte. This is also the case when the value is empty.
            None => &[0],
            // If there no leading zeros, return the whole slice.
            Some(0) => &self.0,
            // If the most significant byte is 0x7, return the slice with the
            // leading zero to keep the number posititive.
            Some(i) if self.0[i] == 0x7 => &self.0[i - 1..],
            // Otherwise, return the slice without the leading zeros.
            Some(i) => &self.0[i..],
        }
    }
}

impl PartialEq for CqlVarInt {
    /// Compares two CQL `varint` objects for equality.
    ///
    /// It works for both normalized and non-normalized `varint`s.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cassander::cql::CqlVarInt;
    ///
    /// // Two varints with the same value but different leading zeros.
    /// let a = CqlVarInt::from_signed_bytes_be(vec![0x0, 0x1]);
    /// let b = CqlVarInt::from_signed_bytes_be(vec![0x1]);
    /// assert_eq!(a, b);
    ///
    /// // An empty varint is zero.
    /// let a = CqlVarInt::from_signed_bytes_be(vec![]);
    /// let b = CqlVarInt::from_signed_bytes_be(vec![0x0]);
    /// assert_eq!(a, b);
    ///
    /// // Two varints with different values. The first one is positive and the
    /// // second one is negative.
    /// let a = CqlVarInt::from_signed_bytes_be(vec![0x0, 0x7, 0x5]);
    /// let b = CqlVarInt::from_signed_bytes_be(vec![0x7, 0x5]);
    /// assert_ne!(a, b);
    ///
    /// // Two varints with the same value but different leading zeros.
    /// let a = CqlVarInt::from_signed_bytes_be(vec![0x0, 0x0, 0x7]);
    /// let b = CqlVarInt::from_signed_bytes_be(vec![0x0, 0x7]);
    fn eq(&self, other: &Self) -> bool {
        self.as_normalized_signed_bytes_be()
            == other.as_normalized_signed_bytes_be()
    }
}

impl Hash for CqlVarInt {
    /// Hashes the normalized bytes of the CQL `varint`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_normalized_signed_bytes_be().hash(state);
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
