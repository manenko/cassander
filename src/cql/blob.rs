/// A CQL `blob` type which represents an arbitrary sequence of bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CqlBlob(Vec<u8>);

impl CqlBlob {
    /// Creates a new [`CqlBlob`] from the given bytes.
    pub fn new<B>(bytes: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self(bytes.into())
    }

    /// Converts the [`CqlBlob`] into a vector of bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Returns a slice of the [`CqlBlob`] bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the length of the [`CqlBlob`] in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the [`CqlBlob`] is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<u8>> for CqlBlob {
    /// Converts the given vector of bytes into a [`CqlBlob`].
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl From<&[u8]> for CqlBlob {
    /// Converts the given slice of bytes into a [`CqlBlob`].
    fn from(value: &[u8]) -> Self {
        Self::new(value)
    }
}

impl AsRef<[u8]> for CqlBlob {
    /// Returns the inner bytes as a reference.
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<Vec<u8>> for CqlBlob {
    /// Returns the inner bytes as a reference.
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl From<CqlBlob> for Vec<u8> {
    /// Converts the given [`CqlBlob`] into a vector of bytes.
    fn from(value: CqlBlob) -> Self {
        value.into_bytes()
    }
}

#[rustfmt::skip]
impl IntoIterator for CqlBlob {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    /// Returns an iterator over the bytes of the [`CqlBlob`].
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[rustfmt::skip]
impl<'a> IntoIterator for &'a CqlBlob {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    /// Returns an iterator over the bytes of the [`CqlBlob`].
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
