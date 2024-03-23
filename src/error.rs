mod details;
mod kind;
mod write_type;

pub use details::*;
pub use kind::*;
use thiserror::Error;
pub use write_type::*;

#[derive(Debug, Error)]
#[error("{0}", .message)]
pub struct DriverError {
    /// The category of the error.
    pub kind:    DriverErrorKind,
    /// The error message.
    pub message: String,
    /// The error details available for server errors only.
    pub details: Option<DriverErrorDetails>,
}

impl DriverError {
    /// Creates a new driver error.
    pub fn new<T>(
        kind: DriverErrorKind,
        message: T,
        details: Option<DriverErrorDetails>,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            kind,
            message: message.into(),
            details,
        }
    }

    /// Creates a new driver error with the given category, message, and no
    /// details.
    pub(crate) fn with_message<T>(kind: DriverErrorKind, message: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(kind, message, None)
    }

    /// Creates a new driver error with the given category, standard message,
    /// and no details.
    pub(crate) fn with_kind(kind: DriverErrorKind) -> Self {
        Self::with_message(kind, kind.to_string())
    }
}
