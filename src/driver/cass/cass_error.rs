use std::fmt::{
    Display,
    Formatter,
};

use crate::driver::ffi::{
    enum_CassError_,
    enum_CassError__CASS_ERROR_LIB_BAD_PARAMS as CASS_ERROR_LIB_BAD_PARAMS,
    enum_CassError__CASS_ERROR_LIB_CALLBACK_ALREADY_SET as CASS_ERROR_LIB_CALLBACK_ALREADY_SET,
    enum_CassError__CASS_ERROR_LIB_EXECUTION_PROFILE_INVALID as CASS_ERROR_LIB_EXECUTION_PROFILE_INVALID,
    enum_CassError__CASS_ERROR_LIB_HOST_RESOLUTION as CASS_ERROR_LIB_HOST_RESOLUTION,
    enum_CassError__CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS as CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS,
    enum_CassError__CASS_ERROR_LIB_INTERNAL_ERROR as CASS_ERROR_LIB_INTERNAL_ERROR,
    enum_CassError__CASS_ERROR_LIB_INVALID_CUSTOM_TYPE as CASS_ERROR_LIB_INVALID_CUSTOM_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_DATA as CASS_ERROR_LIB_INVALID_DATA,
    enum_CassError__CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE as CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_FUTURE_TYPE as CASS_ERROR_LIB_INVALID_FUTURE_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_ITEM_COUNT as CASS_ERROR_LIB_INVALID_ITEM_COUNT,
    enum_CassError__CASS_ERROR_LIB_INVALID_STATE as CASS_ERROR_LIB_INVALID_STATE,
    enum_CassError__CASS_ERROR_LIB_INVALID_STATEMENT_TYPE as CASS_ERROR_LIB_INVALID_STATEMENT_TYPE,
    enum_CassError__CASS_ERROR_LIB_INVALID_VALUE_TYPE as CASS_ERROR_LIB_INVALID_VALUE_TYPE,
    enum_CassError__CASS_ERROR_LIB_MESSAGE_ENCODE as CASS_ERROR_LIB_MESSAGE_ENCODE,
    enum_CassError__CASS_ERROR_LIB_NAME_DOES_NOT_EXIST as CASS_ERROR_LIB_NAME_DOES_NOT_EXIST,
    enum_CassError__CASS_ERROR_LIB_NOT_ENOUGH_DATA as CASS_ERROR_LIB_NOT_ENOUGH_DATA,
    enum_CassError__CASS_ERROR_LIB_NOT_IMPLEMENTED as CASS_ERROR_LIB_NOT_IMPLEMENTED,
    enum_CassError__CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD as CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD,
    enum_CassError__CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD as CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD,
    enum_CassError__CASS_ERROR_LIB_NO_HOSTS_AVAILABLE as CASS_ERROR_LIB_NO_HOSTS_AVAILABLE,
    enum_CassError__CASS_ERROR_LIB_NO_PAGING_STATE as CASS_ERROR_LIB_NO_PAGING_STATE,
    enum_CassError__CASS_ERROR_LIB_NO_STREAMS as CASS_ERROR_LIB_NO_STREAMS,
    enum_CassError__CASS_ERROR_LIB_NO_TRACING_ID as CASS_ERROR_LIB_NO_TRACING_ID,
    enum_CassError__CASS_ERROR_LIB_NULL_VALUE as CASS_ERROR_LIB_NULL_VALUE,
    enum_CassError__CASS_ERROR_LIB_PARAMETER_UNSET as CASS_ERROR_LIB_PARAMETER_UNSET,
    enum_CassError__CASS_ERROR_LIB_REQUEST_QUEUE_FULL as CASS_ERROR_LIB_REQUEST_QUEUE_FULL,
    enum_CassError__CASS_ERROR_LIB_REQUEST_TIMED_OUT as CASS_ERROR_LIB_REQUEST_TIMED_OUT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_CLOSE as CASS_ERROR_LIB_UNABLE_TO_CLOSE,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_CONNECT as CASS_ERROR_LIB_UNABLE_TO_CONNECT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL as CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_INIT as CASS_ERROR_LIB_UNABLE_TO_INIT,
    enum_CassError__CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE as CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE,
    enum_CassError__CASS_ERROR_LIB_UNEXPECTED_RESPONSE as CASS_ERROR_LIB_UNEXPECTED_RESPONSE,
    enum_CassError__CASS_ERROR_LIB_WRITE_ERROR as CASS_ERROR_LIB_WRITE_ERROR,
    enum_CassError__CASS_ERROR_SERVER_ALREADY_EXISTS as CASS_ERROR_SERVER_ALREADY_EXISTS,
    enum_CassError__CASS_ERROR_SERVER_BAD_CREDENTIALS as CASS_ERROR_SERVER_BAD_CREDENTIALS,
    enum_CassError__CASS_ERROR_SERVER_CONFIG_ERROR as CASS_ERROR_SERVER_CONFIG_ERROR,
    enum_CassError__CASS_ERROR_SERVER_FUNCTION_FAILURE as CASS_ERROR_SERVER_FUNCTION_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_INVALID_QUERY as CASS_ERROR_SERVER_INVALID_QUERY,
    enum_CassError__CASS_ERROR_SERVER_IS_BOOTSTRAPPING as CASS_ERROR_SERVER_IS_BOOTSTRAPPING,
    enum_CassError__CASS_ERROR_SERVER_OVERLOADED as CASS_ERROR_SERVER_OVERLOADED,
    enum_CassError__CASS_ERROR_SERVER_PROTOCOL_ERROR as CASS_ERROR_SERVER_PROTOCOL_ERROR,
    enum_CassError__CASS_ERROR_SERVER_READ_FAILURE as CASS_ERROR_SERVER_READ_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_READ_TIMEOUT as CASS_ERROR_SERVER_READ_TIMEOUT,
    enum_CassError__CASS_ERROR_SERVER_SERVER_ERROR as CASS_ERROR_SERVER_SERVER_ERROR,
    enum_CassError__CASS_ERROR_SERVER_SYNTAX_ERROR as CASS_ERROR_SERVER_SYNTAX_ERROR,
    enum_CassError__CASS_ERROR_SERVER_TRUNCATE_ERROR as CASS_ERROR_SERVER_TRUNCATE_ERROR,
    enum_CassError__CASS_ERROR_SERVER_UNAUTHORIZED as CASS_ERROR_SERVER_UNAUTHORIZED,
    enum_CassError__CASS_ERROR_SERVER_UNAVAILABLE as CASS_ERROR_SERVER_UNAVAILABLE,
    enum_CassError__CASS_ERROR_SERVER_UNPREPARED as CASS_ERROR_SERVER_UNPREPARED,
    enum_CassError__CASS_ERROR_SERVER_WRITE_FAILURE as CASS_ERROR_SERVER_WRITE_FAILURE,
    enum_CassError__CASS_ERROR_SERVER_WRITE_TIMEOUT as CASS_ERROR_SERVER_WRITE_TIMEOUT,
    enum_CassError__CASS_ERROR_SSL_CLOSED as CASS_ERROR_SSL_CLOSED,
    enum_CassError__CASS_ERROR_SSL_IDENTITY_MISMATCH as CASS_ERROR_SSL_IDENTITY_MISMATCH,
    enum_CassError__CASS_ERROR_SSL_INVALID_CERT as CASS_ERROR_SSL_INVALID_CERT,
    enum_CassError__CASS_ERROR_SSL_INVALID_PEER_CERT as CASS_ERROR_SSL_INVALID_PEER_CERT,
    enum_CassError__CASS_ERROR_SSL_INVALID_PRIVATE_KEY as CASS_ERROR_SSL_INVALID_PRIVATE_KEY,
    enum_CassError__CASS_ERROR_SSL_NO_PEER_CERT as CASS_ERROR_SSL_NO_PEER_CERT,
    enum_CassError__CASS_ERROR_SSL_PROTOCOL_ERROR as CASS_ERROR_SSL_PROTOCOL_ERROR,
    enum_CassError__CASS_OK as CASS_OK,
};

/// Cassandra error codes.
///
/// The driver returns three types of errors:
///
/// - `Lib*` errors that originate from the driver itself,
/// - `Server*` errors that originate from the server,
/// - `Ssl*` errors that originate from the SSL layer.
///
/// The `Ok` variant is used for successful operations.
///
/// The `Unknown` variant is used for error codes that are not known to this
/// crate.
#[must_use]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CassError {
    /// The operation finished successfully.
    Ok,
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

impl CassError {
    /// Returns `true` if the error is not `Ok`.
    pub fn is_error(self) -> bool {
        self != CassError::Ok
    }

    /// Returns `true` if the error is `Ok`.
    pub fn is_ok(self) -> bool {
        self == CassError::Ok
    }

    /// Returns `Ok(())` if this is [`CassError::Ok`] or `Err(self)` otherwise.
    pub fn as_result(self) -> Result<(), CassError> {
        if self.is_error() {
            Err(self)
        } else {
            Ok(())
        }
    }
}

impl From<enum_CassError_> for CassError {
    #[rustfmt::skip]
    fn from(value: enum_CassError_) -> Self {
        match value {
            CASS_OK                                     => CassError::Ok,
            CASS_ERROR_LIB_BAD_PARAMS                   => CassError::LibBadParams,
            CASS_ERROR_LIB_CALLBACK_ALREADY_SET         => CassError::LibCallbackAlreadySet,
            CASS_ERROR_LIB_EXECUTION_PROFILE_INVALID    => CassError::LibExecutionProfileInvalid,
            CASS_ERROR_LIB_HOST_RESOLUTION              => CassError::LibHostResolution,
            CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS          => CassError::LibIndexOutOfBounds,
            CASS_ERROR_LIB_INTERNAL_ERROR               => CassError::LibInternalError,
            CASS_ERROR_LIB_INVALID_CUSTOM_TYPE          => CassError::LibInvalidCustomType,
            CASS_ERROR_LIB_INVALID_DATA                 => CassError::LibInvalidData,
            CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE    => CassError::LibInvalidErrorResultType,
            CASS_ERROR_LIB_INVALID_FUTURE_TYPE          => CassError::LibInvalidFutureType,
            CASS_ERROR_LIB_INVALID_ITEM_COUNT           => CassError::LibInvalidItemCount,
            CASS_ERROR_LIB_INVALID_STATE                => CassError::LibInvalidState,
            CASS_ERROR_LIB_INVALID_STATEMENT_TYPE       => CassError::LibInvalidStatementType,
            CASS_ERROR_LIB_INVALID_VALUE_TYPE           => CassError::LibInvalidValueType,
            CASS_ERROR_LIB_MESSAGE_ENCODE               => CassError::LibMessageEncode,
            CASS_ERROR_LIB_NAME_DOES_NOT_EXIST          => CassError::LibNameDoesNotExist,
            CASS_ERROR_LIB_NOT_ENOUGH_DATA              => CassError::LibNotEnoughData,
            CASS_ERROR_LIB_NOT_IMPLEMENTED              => CassError::LibNotImplemented,
            CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD       => CassError::LibNoAvailableIoThread,
            CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD            => CassError::LibNoCustomPayload,
            CASS_ERROR_LIB_NO_HOSTS_AVAILABLE           => CassError::LibNoHostsAvailable,
            CASS_ERROR_LIB_NO_PAGING_STATE              => CassError::LibNoPagingState,
            CASS_ERROR_LIB_NO_STREAMS                   => CassError::LibNoStreams,
            CASS_ERROR_LIB_NO_TRACING_ID                => CassError::LibNoTracingId,
            CASS_ERROR_LIB_NULL_VALUE                   => CassError::LibNullValue,
            CASS_ERROR_LIB_PARAMETER_UNSET              => CassError::LibParameterUnset,
            CASS_ERROR_LIB_REQUEST_QUEUE_FULL           => CassError::LibRequestQueueFull,
            CASS_ERROR_LIB_REQUEST_TIMED_OUT            => CassError::LibRequestTimedOut,
            CASS_ERROR_LIB_UNABLE_TO_CLOSE              => CassError::LibUnableToClose,
            CASS_ERROR_LIB_UNABLE_TO_CONNECT            => CassError::LibUnableToConnect,
            CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL => CassError::LibUnableToDetermineProtocol,
            CASS_ERROR_LIB_UNABLE_TO_INIT               => CassError::LibUnableToInit,
            CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE       => CassError::LibUnableToSetKeyspace,
            CASS_ERROR_LIB_UNEXPECTED_RESPONSE          => CassError::LibUnexpectedResponse,
            CASS_ERROR_LIB_WRITE_ERROR                  => CassError::LibWriteError,
            CASS_ERROR_SERVER_ALREADY_EXISTS            => CassError::ServerAlreadyExists,
            CASS_ERROR_SERVER_BAD_CREDENTIALS           => CassError::ServerBadCredentials,
            CASS_ERROR_SERVER_CONFIG_ERROR              => CassError::ServerConfigError,
            CASS_ERROR_SERVER_FUNCTION_FAILURE          => CassError::ServerFunctionFailure,
            CASS_ERROR_SERVER_INVALID_QUERY             => CassError::ServerInvalidQuery,
            CASS_ERROR_SERVER_IS_BOOTSTRAPPING          => CassError::ServerIsBootstrapping,
            CASS_ERROR_SERVER_OVERLOADED                => CassError::ServerOverloaded,
            CASS_ERROR_SERVER_PROTOCOL_ERROR            => CassError::ServerProtocolError,
            CASS_ERROR_SERVER_READ_FAILURE              => CassError::ServerReadFailure,
            CASS_ERROR_SERVER_READ_TIMEOUT              => CassError::ServerReadTimeout,
            CASS_ERROR_SERVER_SERVER_ERROR              => CassError::ServerServerError,
            CASS_ERROR_SERVER_SYNTAX_ERROR              => CassError::ServerSyntaxError,
            CASS_ERROR_SERVER_TRUNCATE_ERROR            => CassError::ServerTruncateError,
            CASS_ERROR_SERVER_UNAUTHORIZED              => CassError::ServerUnauthorized,
            CASS_ERROR_SERVER_UNAVAILABLE               => CassError::ServerUnavailable,
            CASS_ERROR_SERVER_UNPREPARED                => CassError::ServerUnprepared,
            CASS_ERROR_SERVER_WRITE_FAILURE             => CassError::ServerWriteFailure,
            CASS_ERROR_SERVER_WRITE_TIMEOUT             => CassError::ServerWriteTimeout,
            CASS_ERROR_SSL_CLOSED                       => CassError::SslClosed,
            CASS_ERROR_SSL_IDENTITY_MISMATCH            => CassError::SslIdentityMismatch,
            CASS_ERROR_SSL_INVALID_CERT                 => CassError::SslInvalidCert,
            CASS_ERROR_SSL_INVALID_PEER_CERT            => CassError::SslInvalidPeerCert,
            CASS_ERROR_SSL_INVALID_PRIVATE_KEY          => CassError::SslInvalidPrivateKey,
            CASS_ERROR_SSL_NO_PEER_CERT                 => CassError::SslNoPeerCert,
            CASS_ERROR_SSL_PROTOCOL_ERROR               => CassError::SslProtocolError,
            unknown                                     => CassError::Other(unknown),
        }
    }
}

impl From<CassError> for enum_CassError_ {
    #[rustfmt::skip]
    fn from(value: CassError) -> Self {
        match value {
            CassError::Ok                           => CASS_OK,
            CassError::LibBadParams                 => CASS_ERROR_LIB_BAD_PARAMS,
            CassError::LibCallbackAlreadySet        => CASS_ERROR_LIB_CALLBACK_ALREADY_SET,
            CassError::LibExecutionProfileInvalid   => CASS_ERROR_LIB_EXECUTION_PROFILE_INVALID,
            CassError::LibHostResolution            => CASS_ERROR_LIB_HOST_RESOLUTION,
            CassError::LibIndexOutOfBounds          => CASS_ERROR_LIB_INDEX_OUT_OF_BOUNDS,
            CassError::LibInternalError             => CASS_ERROR_LIB_INTERNAL_ERROR,
            CassError::LibInvalidCustomType         => CASS_ERROR_LIB_INVALID_CUSTOM_TYPE,
            CassError::LibInvalidData               => CASS_ERROR_LIB_INVALID_DATA,
            CassError::LibInvalidErrorResultType    => CASS_ERROR_LIB_INVALID_ERROR_RESULT_TYPE,
            CassError::LibInvalidFutureType         => CASS_ERROR_LIB_INVALID_FUTURE_TYPE,
            CassError::LibInvalidItemCount          => CASS_ERROR_LIB_INVALID_ITEM_COUNT,
            CassError::LibInvalidState              => CASS_ERROR_LIB_INVALID_STATE,
            CassError::LibInvalidStatementType      => CASS_ERROR_LIB_INVALID_STATEMENT_TYPE,
            CassError::LibInvalidValueType          => CASS_ERROR_LIB_INVALID_VALUE_TYPE,
            CassError::LibMessageEncode             => CASS_ERROR_LIB_MESSAGE_ENCODE,
            CassError::LibNameDoesNotExist          => CASS_ERROR_LIB_NAME_DOES_NOT_EXIST,
            CassError::LibNotEnoughData             => CASS_ERROR_LIB_NOT_ENOUGH_DATA,
            CassError::LibNotImplemented            => CASS_ERROR_LIB_NOT_IMPLEMENTED,
            CassError::LibNoAvailableIoThread       => CASS_ERROR_LIB_NO_AVAILABLE_IO_THREAD,
            CassError::LibNoCustomPayload           => CASS_ERROR_LIB_NO_CUSTOM_PAYLOAD,
            CassError::LibNoHostsAvailable          => CASS_ERROR_LIB_NO_HOSTS_AVAILABLE,
            CassError::LibNoPagingState             => CASS_ERROR_LIB_NO_PAGING_STATE,
            CassError::LibNoStreams                 => CASS_ERROR_LIB_NO_STREAMS,
            CassError::LibNoTracingId               => CASS_ERROR_LIB_NO_TRACING_ID,
            CassError::LibNullValue                 => CASS_ERROR_LIB_NULL_VALUE,
            CassError::LibParameterUnset            => CASS_ERROR_LIB_PARAMETER_UNSET,
            CassError::LibRequestQueueFull          => CASS_ERROR_LIB_REQUEST_QUEUE_FULL,
            CassError::LibRequestTimedOut           => CASS_ERROR_LIB_REQUEST_TIMED_OUT,
            CassError::LibUnableToClose             => CASS_ERROR_LIB_UNABLE_TO_CLOSE,
            CassError::LibUnableToConnect           => CASS_ERROR_LIB_UNABLE_TO_CONNECT,
            CassError::LibUnableToDetermineProtocol => CASS_ERROR_LIB_UNABLE_TO_DETERMINE_PROTOCOL,
            CassError::LibUnableToInit              => CASS_ERROR_LIB_UNABLE_TO_INIT,
            CassError::LibUnableToSetKeyspace       => CASS_ERROR_LIB_UNABLE_TO_SET_KEYSPACE,
            CassError::LibUnexpectedResponse        => CASS_ERROR_LIB_UNEXPECTED_RESPONSE,
            CassError::LibWriteError                => CASS_ERROR_LIB_WRITE_ERROR,
            CassError::ServerAlreadyExists          => CASS_ERROR_SERVER_ALREADY_EXISTS,
            CassError::ServerBadCredentials         => CASS_ERROR_SERVER_BAD_CREDENTIALS,
            CassError::ServerConfigError            => CASS_ERROR_SERVER_CONFIG_ERROR,
            CassError::ServerFunctionFailure        => CASS_ERROR_SERVER_FUNCTION_FAILURE,
            CassError::ServerInvalidQuery           => CASS_ERROR_SERVER_INVALID_QUERY,
            CassError::ServerIsBootstrapping        => CASS_ERROR_SERVER_IS_BOOTSTRAPPING,
            CassError::ServerOverloaded             => CASS_ERROR_SERVER_OVERLOADED,
            CassError::ServerProtocolError          => CASS_ERROR_SERVER_PROTOCOL_ERROR,
            CassError::ServerReadFailure            => CASS_ERROR_SERVER_READ_FAILURE,
            CassError::ServerReadTimeout            => CASS_ERROR_SERVER_READ_TIMEOUT,
            CassError::ServerServerError            => CASS_ERROR_SERVER_SERVER_ERROR,
            CassError::ServerSyntaxError            => CASS_ERROR_SERVER_SYNTAX_ERROR,
            CassError::ServerTruncateError          => CASS_ERROR_SERVER_TRUNCATE_ERROR,
            CassError::ServerUnauthorized           => CASS_ERROR_SERVER_UNAUTHORIZED,
            CassError::ServerUnavailable            => CASS_ERROR_SERVER_UNAVAILABLE,
            CassError::ServerUnprepared             => CASS_ERROR_SERVER_UNPREPARED,
            CassError::ServerWriteFailure           => CASS_ERROR_SERVER_WRITE_FAILURE,
            CassError::ServerWriteTimeout           => CASS_ERROR_SERVER_WRITE_TIMEOUT,
            CassError::SslClosed                    => CASS_ERROR_SSL_CLOSED,
            CassError::SslIdentityMismatch          => CASS_ERROR_SSL_IDENTITY_MISMATCH,
            CassError::SslInvalidCert               => CASS_ERROR_SSL_INVALID_CERT,
            CassError::SslInvalidPeerCert           => CASS_ERROR_SSL_INVALID_PEER_CERT,
            CassError::SslInvalidPrivateKey         => CASS_ERROR_SSL_INVALID_PRIVATE_KEY,
            CassError::SslNoPeerCert                => CASS_ERROR_SSL_NO_PEER_CERT,
            CassError::SslProtocolError             => CASS_ERROR_SSL_PROTOCOL_ERROR,
            CassError::Other(unknown)               => unknown,
        }
    }
}

impl Display for CassError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unknown_code;
        let message = match self {
            CassError::Ok                           => "success",
            CassError::LibBadParams                 => "bad parameters",
            CassError::LibCallbackAlreadySet        => "callback already set",
            CassError::LibExecutionProfileInvalid   => "invalid execution profile specified",
            CassError::LibHostResolution            => "unable to resolve host",
            CassError::LibIndexOutOfBounds          => "index out of bounds",
            CassError::LibInternalError             => "internal error",
            CassError::LibInvalidCustomType         => "invalid custom type",
            CassError::LibInvalidData               => "invalid data",
            CassError::LibInvalidErrorResultType    => "invalid error result type",
            CassError::LibInvalidFutureType         => "invalid future type",
            CassError::LibInvalidItemCount          => "invalid item count",
            CassError::LibInvalidState              => "invalid state",
            CassError::LibInvalidStatementType      => "invalid statement type",
            CassError::LibInvalidValueType          => "invalid value type",
            CassError::LibMessageEncode             => "unable to encode message",
            CassError::LibNameDoesNotExist          => "no value or column for name",
            CassError::LibNotEnoughData             => "not enough data",
            CassError::LibNotImplemented            => "not implemented",
            CassError::LibNoAvailableIoThread       => "no available IO threads",
            CassError::LibNoCustomPayload           => "no custom payload",
            CassError::LibNoHostsAvailable          => "no hosts available",
            CassError::LibNoPagingState             => "no paging state",
            CassError::LibNoStreams                 => "no streams available",
            CassError::LibNoTracingId               => "no tracing ID",
            CassError::LibNullValue                 => "NULL value specified",
            CassError::LibParameterUnset            => "parameter unse",
            CassError::LibRequestQueueFull          => "the request queue is full",
            CassError::LibRequestTimedOut           => "request timed out",
            CassError::LibUnableToClose             => "unable to close",
            CassError::LibUnableToConnect           => "unable to connect",
            CassError::LibUnableToDetermineProtocol => "unable to find supported protocol version",
            CassError::LibUnableToInit              => "unable to initialize",
            CassError::LibUnableToSetKeyspace       => "unable to set keyspace",
            CassError::LibUnexpectedResponse        => "unexpected response from server",
            CassError::LibWriteError                => "write error",
            CassError::ServerAlreadyExists          => "already exists",
            CassError::ServerBadCredentials         => "bad credentials",
            CassError::ServerConfigError            => "configuration error",
            CassError::ServerFunctionFailure        => "function failure",
            CassError::ServerInvalidQuery           => "invalid query",
            CassError::ServerIsBootstrapping        => "is bootstrapping",
            CassError::ServerOverloaded             => "overloaded",
            CassError::ServerProtocolError          => "protocol error",
            CassError::ServerReadFailure            => "read failure",
            CassError::ServerReadTimeout            => "read timeout",
            CassError::ServerServerError            => "server error",
            CassError::ServerSyntaxError            => "syntax error",
            CassError::ServerTruncateError          => "truncate error",
            CassError::ServerUnauthorized           => "unauthorized",
            CassError::ServerUnavailable            => "unavailable",
            CassError::ServerUnprepared             => "unprepared",
            CassError::ServerWriteFailure           => "write failure",
            CassError::ServerWriteTimeout           => "write timeout",
            CassError::SslClosed                    => "connection closed",
            CassError::SslIdentityMismatch          => "certificate does not match host or IP address",
            CassError::SslInvalidCert               => "unable to load certificate",
            CassError::SslInvalidPeerCert           => "invalid peer certificate",
            CassError::SslInvalidPrivateKey         => "unable to load private key",
            CassError::SslNoPeerCert                => "no peer certificate",
            CassError::SslProtocolError             => "protocol error",
            CassError::Other(unknown)               => {unknown_code = format!("unknown CassError {unknown}"); &unknown_code},
        };

        write!(f, "{}", message)
    }
}
