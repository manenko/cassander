use std::fmt::{
    Display,
    Formatter,
};
use std::net::{
    AddrParseError,
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
};
use std::str::FromStr;

use crate::ffi::{
    cass_inet_init_v4,
    cass_inet_init_v6,
    struct_CassInet_,
};

/// An Internet Protocol (IP) address for either IPv4 or IPv6.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CqlInet(struct_CassInet_);

impl CqlInet {
    /// Creates a new `Inet` from the given driver object.
    pub(crate) fn from_driver(inet: struct_CassInet_) -> Self {
        Self(inet)
    }

    /// Returns the inner driver object.
    pub(crate) fn inner(&self) -> &struct_CassInet_ {
        &self.0
    }
}

impl From<Ipv4Addr> for CqlInet {
    /// Converts the given [`Ipv4Addr`] to a CQL inet object.
    fn from(value: Ipv4Addr) -> Self {
        let address = unsafe { cass_inet_init_v4(value.octets().as_ptr()) };

        Self::from_driver(address)
    }
}

impl From<Ipv6Addr> for CqlInet {
    /// Converts the given [`Ipv6Addr`] to a CQL inet object.
    fn from(value: Ipv6Addr) -> Self {
        let address = unsafe { cass_inet_init_v6(value.octets().as_ptr()) };

        Self::from_driver(address)
    }
}

impl From<IpAddr> for CqlInet {
    /// Converts the given [`IpAddr`] to a CQL inet object.
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(address) => address.into(),
            IpAddr::V6(address) => address.into(),
        }
    }
}

impl From<&CqlInet> for IpAddr {
    /// Converts the given CQL inet object to an [`IpAddr`].
    fn from(value: &CqlInet) -> Self {
        let inet = value.inner();

        match inet.address_length {
            4 => {
                let mut octets = [0; 4];
                octets.copy_from_slice(&inet.address[..4]);

                IpAddr::V4(Ipv4Addr::from(octets))
            }
            16 => IpAddr::from(inet.address),
            _ => unreachable!("invalid CQL inet address: {:?}", inet),
        }
    }
}

impl From<CqlInet> for IpAddr {
    /// Converts the given CQL inet object to an [`IpAddr`].
    fn from(value: CqlInet) -> Self {
        IpAddr::from(&value)
    }
}

impl PartialEq for CqlInet {
    /// Compares two CQL inet objects for equality.
    fn eq(&self, other: &Self) -> bool {
        let inet1 = self.inner();
        let inet2 = other.inner();

        inet1.address_length == inet2.address_length
            && inet1
                .address
                .iter()
                .zip(inet2.address.iter())
                .all(|(a, b)| a == b)
    }
}

impl Eq for CqlInet {}

impl Display for CqlInet {
    /// Formats the CQL inet object as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let address = IpAddr::from(self);

        write!(f, "{}", address)
    }
}

impl FromStr for CqlInet {
    type Err = AddrParseError;

    /// Parses the given string as a CQL inet object.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = s.parse::<IpAddr>()?;

        Ok(address.into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_ipv4() {
        let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
        let cql_inet = CqlInet::from(ipv4);
        assert_eq!(IpAddr::from(cql_inet), IpAddr::V4(ipv4));
    }

    #[test]
    fn test_from_ipv6() {
        let ipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let cql_inet = CqlInet::from(ipv6);
        assert_eq!(IpAddr::from(cql_inet), IpAddr::V6(ipv6));
    }

    #[test]
    fn test_from_ipaddr_v4() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let cql_inet = CqlInet::from(ip);
        assert_eq!(IpAddr::from(cql_inet), ip);
    }

    #[test]
    fn test_from_ipaddr_v6() {
        let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let cql_inet = CqlInet::from(ip);
        assert_eq!(IpAddr::from(cql_inet), ip);
    }

    #[test]
    fn test_equality() {
        let cql_inet1 = CqlInet::from(Ipv4Addr::new(127, 0, 0, 1));
        let cql_inet2 = CqlInet::from(Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(cql_inet1, cql_inet2);
    }

    #[test]
    fn test_inequality() {
        let cql_inet1 = CqlInet::from(Ipv4Addr::new(127, 0, 0, 1));
        let cql_inet2 = CqlInet::from(Ipv4Addr::new(192, 168, 0, 1));
        assert_ne!(cql_inet1, cql_inet2);
    }

    #[test]
    fn test_from_str_v4() {
        let cql_inet = CqlInet::from_str("127.0.0.1").unwrap();
        assert_eq!(
            IpAddr::from(cql_inet),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn test_from_str_v6() {
        let cql_inet = CqlInet::from_str("::1").unwrap();
        assert_eq!(
            IpAddr::from(cql_inet),
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(CqlInet::from_str("invalid").is_err());
    }
}
