use std::ops::{
    BitAnd,
    BitOr,
};

use crate::ffi::enum_CassSslVerifyFlags_;

/// SSL verification flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SslVerifyFlags(u32);

impl SslVerifyFlags {
    /// No verification is performed.
    pub const NONE: SslVerifyFlags = SslVerifyFlags(0x0);
    /// Peer certificate is present and valid.
    ///
    /// This is the default verification.
    pub const PEER_CERT: SslVerifyFlags = SslVerifyFlags(0x1);
    /// IP address matches the certificate's common name or one of its subject
    /// alternative names, the cetificate is present and valid.
    pub const PEER_IDENTITY: SslVerifyFlags = SslVerifyFlags(0x2);
    /// Hostname matches the certificate’s common name or one of its subject
    /// alternative names, the certificate is present and valid, and the
    /// hostname resolution is enabled.
    pub const PEER_IDENTITY_DNS: SslVerifyFlags = SslVerifyFlags(0x4);

    /// Returns the inner value of the flags.
    pub(crate) fn inner(self) -> u32 {
        self.0
    }

    /// Converts the flags to the C++ driver's enum type.
    pub(crate) fn to_driver(self) -> enum_CassSslVerifyFlags_ {
        self.0
    }
}

impl BitAnd for SslVerifyFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        SslVerifyFlags(self.0 & rhs.0)
    }
}

impl BitOr for SslVerifyFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        SslVerifyFlags(self.0 | rhs.0)
    }
}

impl Default for SslVerifyFlags {
    /// The default value is `PEER_CERT`.
    fn default() -> Self {
        SslVerifyFlags::PEER_CERT
    }
}
