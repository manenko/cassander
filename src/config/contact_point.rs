use std::fmt::{
    Display,
    Formatter,
};
use std::str::FromStr;

use thiserror::Error;

use crate::Host;

/// An error that occurs when parsing a contact point.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("failed to parse contact point '{0}'")]
pub struct ContactPointParseError(String);

/// A contact point aka known node that can be used to connect to a Cassandra
/// cluster.
///
/// When the `serde` feature is enabled, this type can be serialized and
/// deserialized using [serde](https://docs.rs/serde/latest/serde/) crate. The
/// contact point is serialized into a string and deserialized from a string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(into = "String", try_from = "String")
)]
pub struct ContactPoint {
    /// The host of the Cassandra node.
    ///
    /// The host can be either a domain name or an IP address (v4 or v6).
    pub host: Host,
    /// The optional port of the Cassandra node.
    pub port: Option<u16>,
}

impl ContactPoint {
    /// Creates a new contact point with the given host and port.
    pub fn new(host: Host, port: u16) -> Self {
        Self {
            host,
            port: Some(port),
        }
    }

    /// Creates a new contact point with the given host and no port.
    pub fn with_host(host: Host) -> Self {
        Self {
            host,
            port: None,
        }
    }
}

impl From<Host> for ContactPoint {
    /// Converts a host into a contact point with no port.
    fn from(host: Host) -> Self {
        Self::with_host(host)
    }
}

impl Display for ContactPoint {
    /// Writes the contact point to the formatter.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.port {
            Some(port) => write!(f, "{}:{}", self.host, port),
            None => write!(f, "{}", self.host),
        }
    }
}

impl FromStr for ContactPoint {
    type Err = ContactPointParseError;

    /// Parses a contact point.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_contact_point(s)
    }
}

impl TryFrom<String> for ContactPoint {
    type Error = ContactPointParseError;

    /// Converts a string into a contact point.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        parse_contact_point(s)
    }
}

impl From<ContactPoint> for String {
    /// Converts a contact point into a string.
    fn from(contact_point: ContactPoint) -> Self {
        contact_point.to_string()
    }
}

impl Default for ContactPoint {
    /// Creates a new contact point with the default host and port.
    fn default() -> Self {
        Self {
            host: Host::default(),
            port: None,
        }
    }
}

fn parse_contact_point<S>(s: S) -> Result<ContactPoint, ContactPointParseError>
where
    S: Into<String>,
{
    let s = s.into();

    s.rsplit_once(':')
        .map(|(host, port)| {
            let host = host
                .parse()
                .map_err(|_| ContactPointParseError(s.clone()))?;
            let port = port
                .parse()
                .map_err(|_| ContactPointParseError(s.clone()))?;
            Ok(ContactPoint::new(host, port))
        })
        .unwrap_or_else(|| {
            let host =
                s.parse().map_err(|_| ContactPointParseError(s.clone()))?;
            Ok(ContactPoint::with_host(host))
        })
}
