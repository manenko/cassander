//! Cassandra authentication providers.

// TODO: Implement custom authentication provider via
//       `cass_cluster_set_authenticator_callbacks`.

use std::fmt::Debug;

/// The authentication provider used by the driver.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Authenticator {
    /// The authenticator that does not perform any authentication.
    None,
    /// The authenticator that performs plain text authentication.
    PlainText { username: String, password: String },
}

impl Authenticator {
    /// Creates a new plain text authenticator.
    pub fn plain_text<U, P>(username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Authenticator::PlainText {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl Debug for Authenticator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Authenticator::None => write!(f, "Authenticator::None"),
            Authenticator::PlainText {
                ..
            } => {
                // Do not print credentials.
                write!(f, "Authenticator::PlainText {{ ... }}")
            }
        }
    }
}

impl Default for Authenticator {
    /// Returns the default authenticator.
    fn default() -> Self {
        Authenticator::None
    }
}
