//! `tracing` integration for the Cassandra driver.
//!
//! This module provides a way to integrate the Cassandra driver logging with
//! the [`tracing`](https://docs.rs/tracing/latest/tracing/index.html) crate.
//!
//! # Example
//!
//! ```rust
//! use cassander::{
//!     logging,
//!     Cluster,
//! };
//! use tracing::Level;
//!
//! #[tokio::main]
//! async fn main() {
//!     let max_level = Level::TRACE;
//!
//!     tracing_subscriber::fmt::fmt()
//!         .with_ansi(true)
//!         .with_max_level(max_level)
//!         .init();
//!
//!     logging::set_log_level(Some(max_level));
//!     logging::use_tracing_logger();
//!
//!     let mut cluster = Cluster::default();
//!     cluster.set_contact_points("127.0.0.1").uwrap();
//!     let _session = cluster.connect_keyspace("kyiv").await.unwrap();
//!
//!     println!("Connected to the cluster");
//! }
//! ```

use std::ffi::{
    c_char,
    c_void,
    CStr,
};

use tracing::{
    event,
    Level,
};

use crate::ffi::{
    cass_log_set_callback,
    cass_log_set_level,
};
#[rustfmt::skip]
use crate::ffi::{
    struct_CassLogMessage_,
    CassLogLevel,
    enum_CassLogLevel__CASS_LOG_INFO     as CASS_LOG_INFO,
    enum_CassLogLevel__CASS_LOG_WARN     as CASS_LOG_WARN,
    enum_CassLogLevel__CASS_LOG_ERROR    as CASS_LOG_ERROR,
    enum_CassLogLevel__CASS_LOG_DISABLED as CASS_LOG_DISABLED,
    enum_CassLogLevel__CASS_LOG_CRITICAL as CASS_LOG_CRITICAL,
    enum_CassLogLevel__CASS_LOG_TRACE    as CASS_LOG_TRACE,
    enum_CassLogLevel__CASS_LOG_DEBUG    as CASS_LOG_DEBUG,
};

/// A log message produced by the Cassandra driver.
#[repr(transparent)]
pub struct LogMessage(*const struct_CassLogMessage_);

impl LogMessage {
    /// Creates a new `LogMessage` from the given driver object.
    pub(crate) fn from_driver(message: *const struct_CassLogMessage_) -> Self {
        LogMessage(message)
    }

    /// Returns the log message.
    pub fn message(&self) -> &str {
        unsafe {
            let message = self.inner().message.as_ptr() as *const c_char;
            let message = CStr::from_ptr(message);

            message
                .to_str()
                .unwrap_or("failed to decode Cassandra driver log message")
        }
    }

    /// Returns the log level of the message.
    ///
    /// Returns `None` if the logging should be disabled.
    pub fn level(&self) -> Option<Level> {
        log_level_to_trace_level(self.inner().severity)
    }

    /// Returns the file name which produced the log message.
    pub fn file(&self) -> &str {
        unsafe {
            let file = self.inner().file;
            let file = CStr::from_ptr(file);

            file.to_str().unwrap_or(
                "failed to decode a file name for Cassandra driver log message",
            )
        }
    }

    /// Returns the line number of the log message.
    ///
    /// Returns `None` if the line number is not available.
    pub fn line(&self) -> Option<u32> {
        self.inner().line.try_into().ok()
    }

    /// Returns the function name which produced the log message.
    pub fn function(&self) -> &str {
        unsafe {
            let function = self.inner().function;
            let function = CStr::from_ptr(function);

            function.to_str().unwrap_or(
                "failed to decode a function name for Cassandra driver log \
                 message",
            )
        }
    }

    /// Returns the timestamp of the log message.
    pub fn timestamp(&self) -> u64 {
        self.inner().time_ms
    }

    /// Returns the inner `struct_CassLogMessage_` object.
    pub(crate) fn inner(&self) -> &struct_CassLogMessage_ {
        unsafe { self.0.as_ref() }
            .expect("Cassandra driver log message is null")
    }
}

/// Sets the log level for the Cassandra driver.
///
/// This needs to be done before any call that might log.
///
/// If `level` is `None`, the logging will be disabled.
pub fn set_log_level(level: Option<Level>) {
    let level = level
        .map(trace_level_to_log_level)
        .unwrap_or(CASS_LOG_DISABLED);

    unsafe {
        cass_log_set_level(level);
    }
}

/// Sets the Cassandra driver logger to use the `tracing` crate.
pub fn use_tracing_logger() {
    unsafe {
        cass_log_set_callback(Some(logging_callback), std::ptr::null_mut())
    };
}

/// Converts a Cassandra log level to a `tracing` log level.
///
/// Returns `None` if the logging should be disabled.
#[rustfmt::skip]
fn log_level_to_trace_level(level: CassLogLevel) -> Option<Level> {
    match level {
        CASS_LOG_DISABLED => None,
        CASS_LOG_CRITICAL => Some(Level::ERROR),
        CASS_LOG_ERROR    => Some(Level::ERROR),
        CASS_LOG_WARN     => Some(Level::WARN),
        CASS_LOG_INFO     => Some(Level::INFO),
        CASS_LOG_DEBUG    => Some(Level::DEBUG),
        CASS_LOG_TRACE    => Some(Level::TRACE),
        _                 => Some(Level::ERROR),
    }
}

#[rustfmt::skip]
fn trace_level_to_log_level(level: Level) -> CassLogLevel {
    match level {
        Level::ERROR => CASS_LOG_ERROR,
        Level::WARN  => CASS_LOG_WARN,
        Level::INFO  => CASS_LOG_INFO,
        Level::DEBUG => CASS_LOG_DEBUG,
        Level::TRACE => CASS_LOG_TRACE,
    }
}

#[rustfmt::skip]
unsafe extern "C" fn logging_callback(
    message: *const struct_CassLogMessage_,
    _data: *mut c_void,
) {
    if message.is_null() {
        return;
    }


    let message = LogMessage::from_driver(message);

    macro_rules! log {
        ($level:expr, $message:expr) => {
            event!(
                target: TARGET,
                $level,
                message   = message.message(),
                file      = message.file(),
                line      = message.line(),
                function  = message.function(),
                timestamp = message.timestamp(),
                )
        };
    }

    if let Some(level) = message.level() {
        match level {
            Level::ERROR => log!(Level::ERROR, message),
            Level::WARN  => log!(Level::WARN, message),
            Level::INFO  => log!(Level::INFO, message),
            Level::DEBUG => log!(Level::DEBUG, message),
            Level::TRACE => log!(Level::TRACE, message),
        }
    }
}

const TARGET: &str = "cassander::driver";
