use std::fmt::{
    Display,
    Formatter,
};
use std::net::{
    IpAddr,
    Ipv4Addr,
};
use std::str::FromStr;

use thiserror::Error;

// TODO: Consider using `url::Host` instead of custom implementation.

/// An error that occurs when parsing a host.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("failed to parse host '{0}'")]
pub struct HostParseError(String);

/// A host that can be used to connect to a Cassandra cluster.
///
/// A host can be either a domain name or an IP address.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Host {
    // The host is a domain name.
    Domain(Domain),
    // The host is an IP address.
    IpAddr(IpAddr),
}

impl FromStr for Host {
    type Err = HostParseError;

    /// Parses a host.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(ip_addr) => Ok(Host::IpAddr(ip_addr)),
            Err(_) => parse_domain(s.to_string()).map(Host::Domain),
        }
    }
}

impl From<IpAddr> for Host {
    /// Converts an IP address into a host.
    fn from(ip_addr: IpAddr) -> Self {
        Host::IpAddr(ip_addr)
    }
}

impl Display for Host {
    /// Writes the host to the formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::Domain(domain) => write!(f, "{}", domain),
            Host::IpAddr(ip_addr) => write!(f, "{}", ip_addr),
        }
    }
}

impl Default for Host {
    /// Creates a new host with the default value.
    fn default() -> Self {
        Host::IpAddr(Ipv4Addr::LOCALHOST.into())
    }
}

/// A domain name.
///
/// <div class="warning">
/// The purpose of this implementation is to provide a quick check for common
/// mistakes like invalid characters or empty strings. It does not provide a
/// full validation of domain names.
/// </div>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Domain(String);

impl AsRef<str> for Domain {
    /// Returns a reference to the domain name.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Domain {
    /// Writes the domain name to the formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Domain {
    type Err = HostParseError;

    /// Parses a domain name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_domain(s)
    }
}

impl TryFrom<String> for Domain {
    type Error = HostParseError;

    /// Parses a domain name.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        parse_domain(s)
    }
}

fn parse_domain<S>(s: S) -> Result<Domain, HostParseError>
where
    S: Into<String>,
{
    let s = s.into();
    if s.is_empty() {
        return Err(HostParseError(s));
    }

    if has_invalid_domain_chars(&s) {
        return Err(HostParseError(s));
    }

    Ok(Domain(s))
}

fn has_invalid_domain_chars(s: &str) -> bool {
    s.chars().any(is_invalid_domain_char)
}

fn is_invalid_domain_char(c: char) -> bool {
    // https://docs.rs/url/latest/src/url/host.rs.html#92-111
    matches!(
        c,
        '\0'..='\u{001F}'
            | ' '
            | '#'
            | '%'
            | '/'
            | ':'
            | '<'
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '\u{007F}'
            | '|'
    )
}
