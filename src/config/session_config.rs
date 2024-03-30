use std::time::Duration;

use crate::authenticator::Authenticator;
use crate::{
    Consistency,
    ContactPoint,
    Host,
    LatencyAwareRoutingPolicy,
    ProtocolVersion,
    RetryPolicy,
    SessionConfigBuilder,
    SpeculativeExecutionPolicy,
    Ssl,
    TimestampGen,
    TokenAwareRoutingPolicy,
};

// TODO: Implement the `Debug` trait for `SessionConfig`.
// TODO: Think about better names for the fields including the prefixes for
//       the fields that belong to the same group. Also, think about "time" vs
//       "interval" vs "timeout" vs "wait_time" vs "period" vs "duration".

/// Cassandra session configuration.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfig {
    /// The list of contact points to use to connect to the Cassandra cluster.
    pub contact_points: Vec<ContactPoint>,

    /// The port to use for the control connection.
    ///
    /// If not set, the default port 9042 is used.
    pub port: Option<u16>,

    /// The SSL configuration.
    ///
    /// If not set, SSL is disabled.
    ///
    /// This field is not available for serialization and deserialization.
    ///
    /// While it is possible to serialize and deserialize the SSL configuration
    /// and then construct an instance of the [`Ssl`] type, it is unwise to do
    /// so because it is all about security. So, deserialize the configuration
    /// and then set the SSL configuration manually.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub ssl: Option<Ssl>,

    /// The Apache Cassandra protocol version.
    ///
    /// This will automatically downgrade to the lowest supported protocol
    /// version.
    ///
    /// If not set, the default value is [`ProtocolVersion::V4`].
    pub protocol_version: Option<ProtocolVersion>,

    /// The default consistency level of a statement.
    ///
    /// If not set, the default value is [`Consistency::LocalOne`].
    pub consistency: Option<Consistency>,

    /// The default serial consistency level of a statement.
    ///
    /// If not set, the default value is [`Consistency::Any`].
    pub serial_consistency: Option<Consistency>,

    /// The number of I/O threads that will handle query requests.
    ///
    /// If not set, the default value is 1.
    pub io_threads_count: Option<usize>,

    /// The size of the fixed size queue that stores pending requests.
    ///
    /// If not set, the default value is 8192.
    pub io_queue_size: Option<usize>,

    /// The size of the fixed size queue that stores events.
    //
    /// If not set, the default value is 8192.
    pub event_queue_size: Option<usize>,

    /// The number of connections made to each server in each I/O thread.
    ///
    /// If not set, the default value is 1.
    pub core_connections_per_host: Option<usize>,

    /// The maximum number of connections made to each server in each I/O
    /// thread.
    ///
    /// If not set, the default value is 2.
    pub max_connections_per_host: Option<usize>,

    /// The wait time before attempting to reconnect.
    ///
    /// If not set, the default value is 2000 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub reconnect_wait_time: Option<Duration>,

    /// The maximum number of connections that will be created concurrently.
    ///
    /// Connections are created when the current connections are unable to keep
    /// up with request throughput.
    ///
    /// If not set, the default value is 1.
    pub max_concurrent_creation: Option<usize>,

    /// The timeout for establishing a connection to a Cassandra node.
    ///
    /// If not set, the default value is 5000 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub connect_timeout: Option<Duration>,

    /// The timeout for waiting for a response from a Cassandra node.
    ///
    /// If not set, the default value is 12000 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub request_timeout: Option<Duration>,

    /// The timeout for waiting for DNS name resolution.
    ///
    /// If not set, the default value is 2000 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub resolve_timeout: Option<Duration>,

    /// The maximum time to wait for schema agreement after a schema-altering
    /// query (e.g. creating, altering, dropping a table/keyspace/view/index
    /// etc).
    ///
    /// If not set, the default value is 10000 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub max_schema_wait_time: Option<Duration>,

    /// The maximum time to wait for tracing data to become available.
    ///
    /// If not set, the default value is 15 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub tracing_max_wait_time: Option<Duration>,

    /// The amount of time to wait between attempts to check to see if tracing
    /// is available.
    ///
    /// If not set, the default value is 3 milliseconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub tracing_retry_wait_time: Option<Duration>,

    /// The consistency level to use for checking to see if tracing data is
    /// available.
    ///
    /// If not set, the default value is [`Consistency::One`].
    pub tracing_consistency: Option<Consistency>,

    /// The default keyspace to use for all queries.
    ///
    /// If not set, the default keyspace is not used. The queries must specify
    /// the keyspace explicitly (`keyspace_name.table_name`).
    pub keyspace: Option<String>,

    /// The page size for all queries unless overridden by setting it on a
    /// query or batch individually.
    ///
    /// Setting a value of 0 disables paging.
    ///
    /// The paging is disabled by default.
    pub page_size: Option<usize>,

    /// The timestamp generator used to assign timestamps to all queries unless
    /// overridden by setting it on a query or batch individually.
    ///
    /// If not set, the monotonically increasing, client-side timestamp
    /// generator is used.
    ///
    /// This field is not available for serialization and deserialization.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub timestamp_gen: Option<TimestampGen>,

    /// Enables or disables bypassing Nagle's algorithm.
    ///
    /// When set to `Some(true)`, the segments are always sent as soon as
    /// possible, even if there is only a small amount of data.
    ///
    /// When not set to `Some(false)`, data is buffered until there is a
    /// sufficient amount to send out, thereby avoiding the frequent sending of
    /// small packets, which results in poor utilization of the network.
    ///
    /// Nagle's algorithm is disabled by default.
    pub tcp_nodelay: Option<bool>,

    /// The keepalive period for the TCP connection.
    ///
    /// Setting a value greater than [`Duration::ZERO`] enables the keepalive
    /// with the given period. Setting a value of [`Duration::ZERO`] disables
    /// the keepalive.
    ///
    /// If not set, the keepalive is disabled.
    pub tcp_keepalive: Option<Duration>,

    /// The amount of time between heartbeat messages to keep the connection
    /// alive.
    ///
    /// This is useful for preventing intermediate network devices from
    /// dropping connections.
    ///
    /// If not set, the default value is 30 seconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub connection_heartbeat_interval: Option<Duration>,

    /// The amount of time a connection is allowed to be without a successful
    /// heartbeat response before being terminated and scheduled for
    /// reconnection.
    ///
    /// If not set, the default value is 60 seconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub connection_idle_timeout: Option<Duration>,

    /// The amount of time between monitor reporting event messages.
    ///
    /// Setting this value to [`Duration::ZERO`] disables the reporting of
    /// event messages.
    ///
    /// If not set, the default value is 300 seconds.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub monitor_reporting_interval: Option<Duration>,

    /// The amount of time after which metric histograms should be refreshed.
    ///
    /// Histograms are reset to zero upon refresh, effectively dropping any
    /// history to that point. Refresh occurs when a snapshot is requested so
    /// this value should be thought of as a minimum time to refresh.
    ///
    /// Setting this value to [`Duration::ZERO`] disables the refresh of metric
    /// histograms.
    ///
    /// If refresh is not enabled the driver will continue to accumulate
    /// histogram data over the life of a session; this is the default
    /// behaviour.
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serialization::opt_duration_as_string")
    )]
    pub metrics_histogram_refresh_interval: Option<Duration>,

    /// Enables/disables retrieving and updating schema metadata.
    ///
    /// If disabled, allows the driver to skip over retrieving and updating
    /// schema metadata and [`Session::get_schema_meta`] will always return an
    /// empty object. This can be useful for reducing the startup overhead of
    /// short-lived sessions.
    ///
    /// The schema metadata is enabled by default.
    pub schema_metadata: Option<bool>,

    /// Enables/disables the randomization of the contact points list.
    ///
    /// <div class="warning">
    /// This setting should only be disabled for debugging or testing purposes.
    /// </div>
    ///
    /// This is enabled by default.
    pub randomized_contact_points: Option<bool>,

    /// The maximum number of "pending write" objects that will be saved for
    /// re-use for marshalling new requests.
    ///
    /// These objects may hold on to a significant amount of memory and
    /// reducing the number of these objects may reduce memory usage of the
    /// application.
    ///
    /// The cost of reducing the value of this setting is potentially slower
    /// marshalling of requests prior to sending.
    ///
    /// If not set, the default value is [`u32::MAX`].
    pub max_reusable_write_objects: Option<usize>,

    /// Enables/disables preparation of statements on all available hosts.
    ///
    /// This is enabled by default.
    pub prepare_on_all_hosts: Option<bool>,

    /// Enables/disables pre-preparing cached prepared statements when existing
    /// hosts become available again or when new hosts are added to the
    /// cluster.
    ///
    /// This can help mitigate request latency when executing prepared
    /// statements by avoiding an extra round trip in cases where the statement
    /// is unprepared on a freshly started server. The main tradeoff is extra
    /// background network traffic is required to prepare the statements on
    /// hosts as they become available.
    ///
    /// This is enabled by default.
    pub prepare_on_up_or_add_host: Option<bool>,

    /// Enables/disables the `NO_COMPACT` startup option.
    ///
    /// This can help facilitate uninterrupted cluster upgrades where tables
    /// using `COMPACT_STORAGE` will operate in "compatibility mode" for
    /// `BATCH`, `DELETE`, `SELECT`, and `UPDATE` CQL operations.
    ///
    /// The option is disabled by default.
    pub no_compact: Option<bool>,

    /// The application name.
    ///
    /// This is optional; however it provides the server with the application
    /// name that can aid in debugging issues with larger clusters where there
    /// are a lot of client (or application) connections.
    pub application_name: Option<String>,

    /// The application version.
    ///
    /// This is optional; however it provides the server with the application
    /// version that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections that may have
    /// different versions in use.
    pub application_version: Option<String>,

    /// The client identifier.
    ///
    /// This is optional; however it provides the server with the client
    /// identifier that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections.
    ///
    /// Default value is a random UUID v4 string.
    pub client_id: Option<String>,

    /// The authenticator used to authenticate to the Cassandra cluster.
    ///
    /// If not set, the default authenticator is [`Authenticator::None`].
    pub authenticator: Option<Authenticator>,

    /// The list of hosts that are allowed to be connected to.
    ///
    /// Any host not in the list will be ignored and a connection will not be
    /// established.
    pub whitelisted_hosts: Vec<Host>,

    /// The list of hosts that are not allowed to be connected to.
    ///
    /// Any host in the list will be ignored and a connection will not be
    /// established.
    pub blacklisted_hosts: Vec<Host>,

    /// The list of datacenters which hosts are allowed to be connected to.
    ///
    /// Any host whose datacenter is not in the list will be ignored and a
    /// connection will not be established.
    pub whitelisted_datacenters: Vec<String>,

    /// The list of datacenters which hosts are not allowed to be connected to.
    ///
    /// Any host whose datacenter is in the list will be ignored and a
    /// connection will not be established.
    pub blacklisted_datacenters: Vec<String>,

    /// The retry policy used for all requests unless overridden by setting
    /// a retry policy on a statement or a batch.
    ///
    /// If not set, the default retry policy is [`RetryPolicy::Default`].
    pub retry_policy: Option<RetryPolicy>,

    /// Configures the speculative executions for queries.
    ///
    /// This policy that decides if the driver will send speculative queries to
    /// the next nodes when the current node takes too long to respond.
    ///
    /// If not set, the default speculative execution policy is
    /// [`SpeculativeExecutionPolicy::None`] which disables speculative
    /// executions.
    pub speculative_execution_policy: Option<SpeculativeExecutionPolicy>,

    /// The token-aware routing policy.
    ///
    /// If not set, the default token-aware routing policy is
    /// [`TokenAwareRoutingPolicy::Standard`] which shuffles the replicas
    /// before routing the request.
    pub token_aware_routing_policy: Option<TokenAwareRoutingPolicy>,

    /// The latency-aware routing policy.
    ///
    /// If not set, the default latency-aware routing policy is
    /// [`LatencyAwareRoutingPolicy::None`] which disables latency-aware
    /// routing.
    pub latency_aware_routing_policy: Option<LatencyAwareRoutingPolicy>,
}

impl SessionConfig {
    /// Creates a new Cassandra session configuration builder.
    pub fn builder() -> SessionConfigBuilder {
        SessionConfigBuilder::default()
    }
}
