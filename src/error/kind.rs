use std::fmt::{
    Display,
    Formatter,
};

use crate::DriverError;
#[rustfmt::skip]
use crate::ffi::{
    enum_CassError_,
    enum_CassError__CASS_ERROR_LIB_BAD_PARAMS                   as LIB_BAD_PARAMS,
    enum_CassError__CASS_ERROR_LIB_CALLBACK_ALREADY_SET         as LIB_CALLBACK_ALREADY_SET,
    enum_CassError__CASS_ERROR_LIB_EXECUTION_PROFILE_INVALID    as LIB_EXECUTION_PROFILE_INVALID,
    enum_CassError__CASS_ERROR_LIB_HOST_RESOLUTION              as LIB_HOST_RESOLUTION,
    enum_CassError__CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS          as LIB_INDEX_OUT_OF_BOUNDS,
    enum_CassError__CASS_ERROR_LIB_INTERNAL_ERROR               as LIB_INTERNAL_ERROR,
    enum_CassError__CASS_ERROR_LIB_INVALID_CUSTOM_TYPE          as LIB_INVALID_CUSTOM_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_DATA                 as LIB_INVALID_DATA,
    enum_CassError__CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE    as LIB_INVALID_ERROR_RESULT_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_FUTURE_TYPE          as LIB_INVALID_FUTURE_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_ITEM_COUNT           as LIB_INVALID_ITEM_COUNT,
    enum_CassError__CASS_ERROR_LIB_INVALID_STATE                as LIB_INVALID_STATE,
    enum_CassError__CASS_ERROR_LIB_INVALID_STATEMENT_TYPE       as LIB_INVALID_STATEMENT_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_VALUE_TYPE           as LIB_INVALID_VALUE_TYPE,
    enum_CassError__CASS_ERROR_LIB_MESSAGE_ENCODE               as LIB_MESSAGE_ENCODE,
    enum_CassError__CASS_ERROR_LIB_NAME_DOES_NOT_EXIST          as LIB_NAME_DOES_NOT_EXIST,
    enum_CassError__CASS_ERROR_LIB_NOT_ENOUGH_DATA              as LIB_NOT_ENOUGH_DATA,
    enum_CassError__CASS_ERROR_LIB_NOT_IMPLEMENTED              as LIB_NOT_IMPLEMENTED,
    enum_CassError__CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD       as LIB_NO_AVAILABLE_IO_THREAD,
    enum_CassError__CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD            as LIB_NO_CUSTOM_PAYLOAD,
    enum_CassError__CASS_ERROR_LIB_NO_HOSTS_AVAILABLE           as LIB_NO_HOSTS_AVAILABLE,
    enum_CassError__CASS_ERROR_LIB_NO_PAGING_STATE              as LIB_NO_PAGING_STATE,
    enum_CassError__CASS_ERROR_LIB_NO_STREAMS                   as LIB_NO_STREAMS,
    enum_CassError__CASS_ERROR_LIB_NO_TRACING_ID                as LIB_NO_TRACING_ID,
    enum_CassError__CASS_ERROR_LIB_NULL_VALUE                   as LIB_NULL_VALUE,
    enum_CassError__CASS_ERROR_LIB_PARAMETER_UNSET              as LIB_PARAMETER_UNSET,
    enum_CassError__CASS_ERROR_LIB_REQUEST_QUEUE_FULL           as LIB_REQUEST_QUEUE_FULL,
    enum_CassError__CASS_ERROR_LIB_REQUEST_TIMED_OUT            as LIB_REQUEST_TIMED_OUT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_CLOSE              as LIB_UNABLE_TO_CLOSE,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_CONNECT            as LIB_UNABLE_TO_CONNECT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL as LIB_UNABLE_TO_DETERMINE_PROTOCOL,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_INIT               as LIB_UNABLE_TO_INIT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE       as LIB_UNABLE_TO_SET_KEYSPACE,
    enum_CassError__CASS_ERROR_LIB_UNEXPECTED_RESPONSE          as LIB_UNEXPECTED_RESPONSE,
    enum_CassError__CASS_ERROR_LIB_WRITE_ERROR                  as LIB_WRITE_ERROR,
    enum_CassError__CASS_ERROR_SERVER_ALREADY_EXISTS            as SERVER_ALREADY_EXISTS,
    enum_CassError__CASS_ERROR_SERVER_BAD_CREDENTIALS           as SERVER_BAD_CREDENTIALS,
    enum_CassError__CASS_ERROR_SERVER_CONFIG_ERROR              as SERVER_CONFIG_ERROR,
    enum_CassError__CASS_ERROR_SERVER_FUNCTION_FAILURE          as SERVER_FUNCTION_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_INVALID_QUERY             as SERVER_INVALID_QUERY,
    enum_CassError__CASS_ERROR_SERVER_IS_BOOTSTRAPPING          as SERVER_IS_BOOTSTRAPPING,
    enum_CassError__CASS_ERROR_SERVER_OVERLOADED                as SERVER_OVERLOADED,
    enum_CassError__CASS_ERROR_SERVER_PROTOCOL_ERROR            as SERVER_PROTOCOL_ERROR,
    enum_CassError__CASS_ERROR_SERVER_READ_FAILURE              as SERVER_READ_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_READ_TIMEOUT              as SERVER_READ_TIMEOUT,
    enum_CassError__CASS_ERROR_SERVER_SERVER_ERROR              as SERVER_SERVER_ERROR,
    enum_CassError__CASS_ERROR_SERVER_SYNTAX_ERROR              as SERVER_SYNTAX_ERROR,
    enum_CassError__CASS_ERROR_SERVER_TRUNCATE_ERROR            as SERVER_TRUNCATE_ERROR,
    enum_CassError__CASS_ERROR_SERVER_UNAUTHORIZED              as SERVER_UNAUTHORIZED,
    enum_CassError__CASS_ERROR_SERVER_UNAVAILABLE               as SERVER_UNAVAILABLE,
    enum_CassError__CASS_ERROR_SERVER_UNPREPARED                as SERVER_UNPREPARED,
    enum_CassError__CASS_ERROR_SERVER_WRITE_FAILURE             as SERVER_WRITE_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_WRITE_TIMEOUT             as SERVER_WRITE_TIMEOUT,
    enum_CassError__CASS_ERROR_SSL_CLOSED                       as SSL_CLOSED,
    enum_CassError__CASS_ERROR_SSL_IDENTITY_MISMATCH            as SSL_IDENTITY_MISMATCH,
    enum_CassError__CASS_ERROR_SSL_INVALID_CERT                 as SSL_INVALID_CERT,
    enum_CassError__CASS_ERROR_SSL_INVALID_PEER_CERT            as SSL_INVALID_PEER_CERT,
    enum_CassError__CASS_ERROR_SSL_INVALID_PRIVATE_KEY          as SSL_INVALID_PRIVATE_KEY,
    enum_CassError__CASS_ERROR_SSL_NO_PEER_CERT                 as SSL_NO_PEER_CERT,
    enum_CassError__CASS_ERROR_SSL_PROTOCOL_ERROR               as SSL_PROTOCOL_ERROR,
    enum_CassError__CASS_OK                                     as CASS_OK,
};

/// Cassandra error codes.
///
/// The driver returns three types of errors:
///
/// - `Lib*` errors that originate from the driver itself,
/// - `Server*` errors that originate from the server,
/// - `Ssl*` errors that originate from the SSL layer.
///
/// The `Unknown` variant is used for error codes that are not known to this
/// crate.
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DriverErrorKind {
    /// The incorrect params were passed to a driver's function.
    LibBadParams,
    /// The future callback was already set.
    LibCallbackAlreadySet,
    /// The execution profile is invalid or does not exist.
    LibExecutionProfileInvalid,
    /// Failed to resolve the host.
    LibHostResolution,
    /// The index is out of bounds.
    ///
    /// This is returned by many functions that take an index as a parameter.
    LibIndexOutOfBounds,
    /// The driver encountered an internal error which is most likely a bug.
    LibInternalError,
    /// The value has invalid custom data type.
    LibInvalidCustomType,
    /// An error occurred while parsing or validating data coming from
    /// Cassandra.
    LibInvalidData,
    /// The requested data cannot be extracted from the error result because
    /// its type does not support it.
    LibInvalidErrorResultType,
    /// The requested data cannot be extracted from the future because its type
    /// does not support it.
    LibInvalidFutureType,
    /// The collection has an invalid item count.
    ///
    /// For example, a Cassandra map with an odd number of items.
    LibInvalidItemCount,
    /// The driver failed tp perform an operation because of an invalid state.
    ///
    /// This is most likely a bug in the driver.
    LibInvalidState,
    /// The statement has an invalid type.
    ///
    /// Currently, the driver does not use this error code.
    LibInvalidStatementType,
    /// Cannot perform the operation because of an invalid value type.
    ///
    /// For example, this happens when a trying to get a `tinyint` as a `text`.
    LibInvalidValueType,
    /// The operation is unsupported by the current version of the protocol or
    /// the encoded request had no data to write.
    LibMessageEncode,
    /// There is no such name in the data.
    LibNameDoesNotExist,
    /// The driver did not receive enough data to perform the operation.
    LibNotEnoughData,
    /// The operation is not implemented.
    LibNotImplemented,
    /// No IO threads are available.
    LibNoAvailableIoThread,
    /// Failed to get the custom payload from the driver's future.
    LibNoCustomPayload,
    /// The driver's session cannot connect to any hosts.
    LibNoHostsAvailable,
    /// The response from the server does not contain a paging state token.
    LibNoPagingState,
    /// The driver's session has no networking streams available.
    LibNoStreams,
    /// The response from the server does not contain a tracing ID.
    LibNoTracingId,
    /// Tried to extract data from a Cassandra value that is `NULL`.
    LibNullValue,
    /// The statement's parameter is not set, i.e. it is unbound.
    LibParameterUnset,
    /// The request queue has reached its capacity.
    LibRequestQueueFull,
    /// The request timed out.
    LibRequestTimedOut,
    /// Cannot close the session because it is already closed or is closing.
    LibUnableToClose,
    /// The driver was unable to connect to the server for unknown reason.
    ///
    /// This is most likely a bug in the driver.
    LibUnableToConnect,
    /// The driver was unable to determine the protocol version to use.
    LibUnableToDetermineProtocol,
    /// The driver was unable to initialize one of its subsystems.
    LibUnableToInit,
    /// The driver was unable to set the keyspace.
    LibUnableToSetKeyspace,
    /// The driver received an unexpected response from the server.
    LibUnexpectedResponse,
    /// The driver encountered an error while writing data to the server.
    LibWriteError,
    /// The query attempted to create a keyspace or a table that was already
    /// existing.
    ServerAlreadyExists,
    /// Authentication was required and failed.
    ///
    /// The possible reason for failing depends on the authenticator in use,
    /// which may or may not include more detail in the accompanying error
    /// message.
    ServerBadCredentials,
    /// The query is invalid because of a configuration issue.
    ServerConfigError,
    /// A (user defined) function failed during execution.
    ServerFunctionFailure,
    /// The query is syntactically correct but invalid.
    ServerInvalidQuery,
    /// The request was a read request but the coordinator node is
    /// bootstrapping.
    ServerIsBootstrapping,
    /// The request cannot be processed because the coordinator node is
    /// overloaded.
    ServerOverloaded,
    /// A client message triggered a protocol violation.
    ServerProtocolError,
    /// A non-timeout error during a read request.
    ServerReadFailure,
    /// Timeout error during a read request.
    ServerReadTimeout,
    /// Something unexpected happened.
    ///
    /// This indicates a server-side bug.
    ServerServerError,
    /// The submitted query has a syntax error.
    ServerSyntaxError,
    /// A `TRUNCATE` operation triggered an error.
    ServerTruncateError,
    /// The logged user doesn't have the right to perform the query.
    ServerUnauthorized,
    /// The server is not available.
    ServerUnavailable,
    /// Can happen while a prepared statement tries to be executed if the
    /// provided prepared statement ID is not known by this host.
    ServerUnprepared,
    /// A non-timeout error during a write request.
    ServerWriteFailure,
    /// Timeout error during a write request.
    ServerWriteTimeout,
    /// The SSL connection was closed.
    SslClosed,
    /// The certificate does not match the host or IP address.
    SslIdentityMismatch,
    /// The certificate is invalid.
    SslInvalidCert,
    /// The peer certificate is invalid.
    SslInvalidPeerCert,
    /// The private key is invalid.
    SslInvalidPrivateKey,
    /// No peer certificate was provided.
    SslNoPeerCert,
    /// An SSL protocol error occurred.
    SslProtocolError,
    /// The driver returned an error which code is unknown to this crate.
    Other(u32),
}

impl DriverErrorKind {
    /// Creates a new `DriverErrorKind` from a driver raw error code.
    ///
    /// Returns `None` if the error code is `CASS_OK`.
    #[rustfmt::skip]
    pub(crate) fn from_driver(code: enum_CassError_) -> Option<Self> {
        use DriverErrorKind::*;

        match code {
            CASS_OK                          => None,
            LIB_BAD_PARAMS                   => Some(LibBadParams),
            LIB_CALLBACK_ALREADY_SET         => Some(LibCallbackAlreadySet),
            LIB_EXECUTION_PROFILE_INVALID    => Some(LibExecutionProfileInvalid),
            LIB_HOST_RESOLUTION              => Some(LibHostResolution),
            LIB_INDEX_OUT_OF_BOUNDS          => Some(LibIndexOutOfBounds),
            LIB_INTERNAL_ERROR               => Some(LibInternalError),
            LIB_INVALID_CUSTOM_TYPE          => Some(LibInvalidCustomType),
            LIB_INVALID_DATA                 => Some(LibInvalidData),
            LIB_INVALID_ERROR_RESULT_TYPE    => Some(LibInvalidErrorResultType),
            LIB_INVALID_FUTURE_TYPE          => Some(LibInvalidFutureType),
            LIB_INVALID_ITEM_COUNT           => Some(LibInvalidItemCount),
            LIB_INVALID_STATE                => Some(LibInvalidState),
            LIB_INVALID_STATEMENT_TYPE       => Some(LibInvalidStatementType),
            LIB_INVALID_VALUE_TYPE           => Some(LibInvalidValueType),
            LIB_MESSAGE_ENCODE               => Some(LibMessageEncode),
            LIB_NAME_DOES_NOT_EXIST          => Some(LibNameDoesNotExist),
            LIB_NOT_ENOUGH_DATA              => Some(LibNotEnoughData),
            LIB_NOT_IMPLEMENTED              => Some(LibNotImplemented),
            LIB_NO_AVAILABLE_IO_THREAD       => Some(LibNoAvailableIoThread),
            LIB_NO_CUSTOM_PAYLOAD            => Some(LibNoCustomPayload),
            LIB_NO_HOSTS_AVAILABLE           => Some(LibNoHostsAvailable),
            LIB_NO_PAGING_STATE              => Some(LibNoPagingState),
            LIB_NO_STREAMS                   => Some(LibNoStreams),
            LIB_NO_TRACING_ID                => Some(LibNoTracingId),
            LIB_NULL_VALUE                   => Some(LibNullValue),
            LIB_PARAMETER_UNSET              => Some(LibParameterUnset),
            LIB_REQUEST_QUEUE_FULL           => Some(LibRequestQueueFull),
            LIB_REQUEST_TIMED_OUT            => Some(LibRequestTimedOut),
            LIB_UNABLE_TO_CLOSE              => Some(LibUnableToClose),
            LIB_UNABLE_TO_CONNECT            => Some(LibUnableToConnect),
            LIB_UNABLE_TO_DETERMINE_PROTOCOL => Some(LibUnableToDetermineProtocol),
            LIB_UNABLE_TO_INIT               => Some(LibUnableToInit),
            LIB_UNABLE_TO_SET_KEYSPACE       => Some(LibUnableToSetKeyspace),
            LIB_UNEXPECTED_RESPONSE          => Some(LibUnexpectedResponse),
            LIB_WRITE_ERROR                  => Some(LibWriteError),
            SERVER_ALREADY_EXISTS            => Some(ServerAlreadyExists),
            SERVER_BAD_CREDENTIALS           => Some(ServerBadCredentials),
            SERVER_CONFIG_ERROR              => Some(ServerConfigError),
            SERVER_FUNCTION_FAILURE          => Some(ServerFunctionFailure),
            SERVER_INVALID_QUERY             => Some(ServerInvalidQuery),
            SERVER_IS_BOOTSTRAPPING          => Some(ServerIsBootstrapping),
            SERVER_OVERLOADED                => Some(ServerOverloaded),
            SERVER_PROTOCOL_ERROR            => Some(ServerProtocolError),
            SERVER_READ_FAILURE              => Some(ServerReadFailure),
            SERVER_READ_TIMEOUT              => Some(ServerReadTimeout),
            SERVER_SERVER_ERROR              => Some(ServerServerError),
            SERVER_SYNTAX_ERROR              => Some(ServerSyntaxError),
            SERVER_TRUNCATE_ERROR            => Some(ServerTruncateError),
            SERVER_UNAUTHORIZED              => Some(ServerUnauthorized),
            SERVER_UNAVAILABLE               => Some(ServerUnavailable),
            SERVER_UNPREPARED                => Some(ServerUnprepared),
            SERVER_WRITE_FAILURE             => Some(ServerWriteFailure),
            SERVER_WRITE_TIMEOUT             => Some(ServerWriteTimeout),
            SSL_CLOSED                       => Some(SslClosed),
            SSL_IDENTITY_MISMATCH            => Some(SslIdentityMismatch),
            SSL_INVALID_CERT                 => Some(SslInvalidCert),
            SSL_INVALID_PEER_CERT            => Some(SslInvalidPeerCert),
            SSL_INVALID_PRIVATE_KEY          => Some(SslInvalidPrivateKey),
            SSL_NO_PEER_CERT                 => Some(SslNoPeerCert),
            SSL_PROTOCOL_ERROR               => Some(SslProtocolError),
            unknown                          => Some(Other(unknown)),
        }
    }
}

/// Converts a driver error code to a `Result`.
pub(crate) fn to_result<T>(code: enum_CassError_) -> Result<T, DriverError>
where
    T: Default,
{
    to_result_with_message(code, code.to_string())
}

/// Converts a driver error code to a `Result` with the given error message.
pub(crate) fn to_result_with_message<T, M>(
    code: enum_CassError_,
    message: M,
) -> Result<T, DriverError>
where
    T: Default,
    M: Into<String>,
{
    DriverErrorKind::from_driver(code)
        .map(|kind| Err(DriverError::with_message(kind, message)))
        .unwrap_or_else(|| Ok(T::default()))
}

impl Display for DriverErrorKind {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use DriverErrorKind::*;

        let unknown_code;
        let message = match self {
            LibBadParams                 => "bad parameters",
            LibCallbackAlreadySet        => "callback already set",
            LibExecutionProfileInvalid   => "invalid execution profile specified",
            LibHostResolution            => "unable to resolve host",
            LibIndexOutOfBounds          => "index out of bounds",
            LibInternalError             => "internal error",
            LibInvalidCustomType         => "invalid custom type",
            LibInvalidData               => "invalid data",
            LibInvalidErrorResultType    => "invalid error result type",
            LibInvalidFutureType         => "invalid future type",
            LibInvalidItemCount          => "invalid item count",
            LibInvalidState              => "invalid state",
            LibInvalidStatementType      => "invalid statement type",
            LibInvalidValueType          => "invalid value type",
            LibMessageEncode             => "unable to encode message",
            LibNameDoesNotExist          => "no value or column for name",
            LibNotEnoughData             => "not enough data",
            LibNotImplemented            => "not implemented",
            LibNoAvailableIoThread       => "no available IO threads",
            LibNoCustomPayload           => "no custom payload",
            LibNoHostsAvailable          => "no hosts available",
            LibNoPagingState             => "no paging state",
            LibNoStreams                 => "no streams available",
            LibNoTracingId               => "no tracing ID",
            LibNullValue                 => "NULL value specified",
            LibParameterUnset            => "parameter unse",
            LibRequestQueueFull          => "the request queue is full",
            LibRequestTimedOut           => "request timed out",
            LibUnableToClose             => "unable to close",
            LibUnableToConnect           => "unable to connect",
            LibUnableToDetermineProtocol => "unable to find supported protocol version",
            LibUnableToInit              => "unable to initialize",
            LibUnableToSetKeyspace       => "unable to set keyspace",
            LibUnexpectedResponse        => "unexpected response from server",
            LibWriteError                => "write error",
            ServerAlreadyExists          => "already exists",
            ServerBadCredentials         => "bad credentials",
            ServerConfigError            => "configuration error",
            ServerFunctionFailure        => "function failure",
            ServerInvalidQuery           => "invalid query",
            ServerIsBootstrapping        => "is bootstrapping",
            ServerOverloaded             => "overloaded",
            ServerProtocolError          => "protocol error",
            ServerReadFailure            => "read failure",
            ServerReadTimeout            => "read timeout",
            ServerServerError            => "server error",
            ServerSyntaxError            => "syntax error",
            ServerTruncateError          => "truncate error",
            ServerUnauthorized           => "unauthorized",
            ServerUnavailable            => "unavailable",
            ServerUnprepared             => "unprepared",
            ServerWriteFailure           => "write failure",
            ServerWriteTimeout           => "write timeout",
            SslClosed                    => "connection closed",
            SslIdentityMismatch          => "certificate does not match host or IP address",
            SslInvalidCert               => "unable to load certificate",
            SslInvalidPeerCert           => "invalid peer certificate",
            SslInvalidPrivateKey         => "unable to load private key",
            SslNoPeerCert                => "no peer certificate",
            SslProtocolError             => "protocol error",
            Other(unknown)               => {unknown_code = format!("unknown CassError {unknown}"); &unknown_code},
        };

        write!(f, "{}", message)
    }
}
