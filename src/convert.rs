//! Traits for conversions between types.

/// A trait for types that can be converted from another type.
///
/// This is not a general-purpose conversion trait. It is intended to be used
/// in cases where a value of a type can be absent and the type has a special
/// value to represent that absence. This is the case for many types in the
/// DataStax C++ driver for Apache Cassandra.
pub trait MaybeFrom<T>: Sized {
    /// Converts a value of type `T` into `Self`.
    ///
    /// Returns `None` if the `value` represents a special meaning of an absent
    /// value.
    fn maybe_from(value: T) -> Option<Self>;
}

/// A trait for types that can be converted into another type.
///
/// This is not a general-purpose conversion trait. It is intended to be used
/// in cases where a value of a type can be absent and the type has a special
/// value to represent that absence. This is the case for many types in the
/// DataStax C++ driver for Apache Cassandra.
pub trait MaybeInto<T>: Sized {
    /// Converts a value of type `Self` into `T`.
    ///
    /// Returns `None` if the `self` represents a special meaning of an absent
    /// value.
    fn maybe_into(self) -> Option<T>;
}

impl<A, B> MaybeInto<B> for A
where
    B: MaybeFrom<A>,
{
    /// Converts a value of type `A` which implements [`MaybeFrom<B>`] into a
    /// value of type `B`.
    fn maybe_into(self) -> Option<B> {
        B::maybe_from(self)
    }
}
