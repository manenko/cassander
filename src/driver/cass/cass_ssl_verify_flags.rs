use std::ops::{
    BitAnd,
    BitOr,
};

use crate::driver::ffi::enum_CassSslVerifyFlags_;

/// SSL verification flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CassSslVerifyFlags(u32);

impl CassSslVerifyFlags {
    /// No verification is performed.
    pub const NONE: CassSslVerifyFlags = CassSslVerifyFlags(0x0);
    /// Peer certificate is present and valid.
    ///
    /// This is the default verification.
    pub const PEER_CERT: CassSslVerifyFlags = CassSslVerifyFlags(0x1);
    /// IP address matches the certificate's common name or one of its subject
    /// alternative names, the cetificate is present and valid.
    pub const PEER_IDENTITY: CassSslVerifyFlags = CassSslVerifyFlags(0x2);
    /// Hostname matches the certificate’s common name or one of its subject
    /// alternative names, the certificate is present and valid, and the
    /// hostname resolution is enabled.
    pub const PEER_IDENTITY_DNS: CassSslVerifyFlags = CassSslVerifyFlags(0x4);
}

impl BitAnd for CassSslVerifyFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        CassSslVerifyFlags(self.0 & rhs.0)
    }
}

impl BitOr for CassSslVerifyFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        CassSslVerifyFlags(self.0 | rhs.0)
    }
}

impl Default for CassSslVerifyFlags {
    /// The default value is `PEER_CERT`.
    fn default() -> Self {
        CassSslVerifyFlags::PEER_CERT
    }
}

impl From<CassSslVerifyFlags> for enum_CassSslVerifyFlags_ {
    fn from(flags: CassSslVerifyFlags) -> Self {
        flags.0
    }
}
