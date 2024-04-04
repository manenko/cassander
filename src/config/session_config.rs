use std::fmt::Debug;
use std::num::{
    NonZeroU64,
    TryFromIntError,
};
use std::time::Duration;

use thiserror::Error;

use crate::authenticator::Authenticator;
use crate::cql::CqlUuid;
use crate::{
    Cluster,
    Consistency,
    ContactPoint,
    DriverError,
    DriverErrorKind,
    Host,
    LatencyAwareRoutingPolicy,
    LoadBalancingPolicy,
    ProtocolVersion,
    RetryPolicy,
    Session,
    SessionConfigBuilder,
    SessionCreationError,
    SpeculativeExecutionPolicy,
    Ssl,
    StandardLatencyAwareRoutingPolicySettings,
    TimestampGen,
    TokenAwareRoutingPolicy,
};

// TODO: Think about better names for the fields including the prefixes for
//       the fields that belong to the same group. Also, think about "time" vs
//       "interval" vs "timeout" vs "wait_time" vs "period" vs "duration".
// TODO: Use `KeyspaceName` instead of `String` for `SessionConfig::keyspace` to
//       avoid empty or blank strings. Or use a more generic type like `Name`,
//       `NonBlankString`.

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
    /// Default value is a random UUID v4.
    pub client_id: Option<CqlUuid>,

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

    /// The load balancing policy to use when selecting a node to send a
    /// request to.
    ///
    /// This configures which nodes the driver talks to, and in which order
    /// they are tried.
    ///
    /// If not set, the default load balancing policy is
    /// [`LoadBalancingPolicy::DatacenterAware`]. It is strongly recommended
    /// to use the default load balancing policy.
    pub load_balancing_policy: Option<LoadBalancingPolicy>,
}

impl SessionConfig {
    /// Creates a new Cassandra session configuration builder.
    pub fn builder() -> SessionConfigBuilder {
        SessionConfigBuilder::default()
    }

    /// Connects to the Cassandra cluster using the session configuration.
    pub async fn connect(self) -> Result<Session, SessionCreationError> {
        Session::connect(self).await
    }
}

impl Debug for SessionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionConfig")
            .field("contact_points", &self.contact_points)
            .field("port", &self.port)
            .field("ssl", &self.ssl.as_ref().map(|_| "Ssl { ... }"))
            .field("protocol_version", &self.protocol_version)
            .field("consistency", &self.consistency)
            .field("serial_consistency", &self.serial_consistency)
            .field("io_threads_count", &self.io_threads_count)
            .field("io_queue_size", &self.io_queue_size)
            .field("event_queue_size", &self.event_queue_size)
            .field("core_connections_per_host", &self.core_connections_per_host)
            .field("max_connections_per_host", &self.max_connections_per_host)
            .field("reconnect_wait_time", &self.reconnect_wait_time)
            .field("max_concurrent_creation", &self.max_concurrent_creation)
            .field("connect_timeout", &self.connect_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("resolve_timeout", &self.resolve_timeout)
            .field("max_schema_wait_time", &self.max_schema_wait_time)
            .field("tracing_max_wait_time", &self.tracing_max_wait_time)
            .field("tracing_retry_wait_time", &self.tracing_retry_wait_time)
            .field("tracing_consistency", &self.tracing_consistency)
            .field("keyspace", &self.keyspace)
            .field("page_size", &self.page_size)
            .field(
                "timestamp_gen",
                &self.timestamp_gen.as_ref().map(|_| "TimestampGen { ... }"),
            )
            .field("tcp_nodelay", &self.tcp_nodelay)
            .field("tcp_keepalive", &self.tcp_keepalive)
            .field(
                "connection_heartbeat_interval",
                &self.connection_heartbeat_interval,
            )
            .field("connection_idle_timeout", &self.connection_idle_timeout)
            .field(
                "monitor_reporting_interval",
                &self.monitor_reporting_interval,
            )
            .field(
                "metrics_histogram_refresh_interval",
                &self.metrics_histogram_refresh_interval,
            )
            .field("schema_metadata", &self.schema_metadata)
            .field("randomized_contact_points", &self.randomized_contact_points)
            .field(
                "max_reusable_write_objects",
                &self.max_reusable_write_objects,
            )
            .field("prepare_on_all_hosts", &self.prepare_on_all_hosts)
            .field("prepare_on_up_or_add_host", &self.prepare_on_up_or_add_host)
            .field("no_compact", &self.no_compact)
            .field("application_name", &self.application_name)
            .field("application_version", &self.application_version)
            .field("client_id", &self.client_id)
            .field("authenticator", &self.authenticator)
            .field("whitelisted_hosts", &self.whitelisted_hosts)
            .field("blacklisted_hosts", &self.blacklisted_hosts)
            .field("whitelisted_datacenters", &self.whitelisted_datacenters)
            .field("blacklisted_datacenters", &self.blacklisted_datacenters)
            .field("retry_policy", &self.retry_policy)
            .field(
                "speculative_execution_policy",
                &self.speculative_execution_policy,
            )
            .field(
                "token_aware_routing_policy",
                &self.token_aware_routing_policy,
            )
            .field(
                "latency_aware_routing_policy",
                &self.latency_aware_routing_policy,
            )
            .field("load_balancing_policy", &self.load_balancing_policy)
            .finish()
    }
}

/// An error that can occurr while configuring a session.
#[derive(Debug, Error)]
#[error("error configuring session configuration key {}: {}", .key, .details)]
pub struct SessionConfigError {
    /// The session config key that caused the error.
    pub key:     String,
    /// The details of the error.
    pub details: DriverError,
}

impl SessionConfigError {
    fn new<S>(key: S, details: DriverError) -> Self
    where
        S: Into<String>,
    {
        Self {
            key: key.into(),
            details,
        }
    }

    fn with_error<S, E>(key: S, details: E) -> Self
    where
        S: Into<String>,
        E: ToString,
    {
        Self::new(
            key,
            DriverError::with_message(
                DriverErrorKind::LibBadParams,
                details.to_string(),
            ),
        )
    }
}

impl TryFrom<SessionConfig> for Cluster {
    type Error = SessionConfigError;

    /// Converts a session configuration into a cluster configuration.
    fn try_from(value: SessionConfig) -> Result<Self, Self::Error> {
        let mut cluster = Cluster::new();

        macro_rules! apply_config {
            // Match one or more `config => setter` pairs, separated by commas
            ($($config:ident $setter:ident),+ $(,)?) => {
                $(
                    $setter(&mut cluster, value.$config)?;
                 )+
            };
        }

        apply_config!(
            application_name                   apply_appication_name,
            application_version                apply_appication_version,
            authenticator                      apply_authenticator,
            blacklisted_datacenters            apply_blacklisted_datacenters,
            blacklisted_hosts                  apply_blacklisted_hosts,
            client_id                          apply_client_id,
            connect_timeout                    apply_connect_timeout,
            connection_heartbeat_interval      apply_connection_heartbeat_interval,
            connection_idle_timeout            apply_connection_idle_timeout,
            consistency                        apply_consistency,
            contact_points                     apply_contact_points,
            core_connections_per_host          apply_core_connections_per_host,
            event_queue_size                   apply_event_queue_size,
            io_queue_size                      apply_io_queue_size,
            io_threads_count                   apply_io_threads_count,
            latency_aware_routing_policy       apply_latency_aware_routing_policy,
            load_balancing_policy              apply_load_balancing_policy,
            max_concurrent_creation            apply_max_concurrent_creation,
            max_connections_per_host           apply_max_connections_per_host,
            max_reusable_write_objects         apply_max_reusable_write_objects,
            max_schema_wait_time               apply_max_schema_wait_time,
            metrics_histogram_refresh_interval apply_metrics_histogram_refresh_interval,
            monitor_reporting_interval         apply_monitor_reporting_interval,
            no_compact                         apply_no_compact,
            port                               apply_port,
            prepare_on_all_hosts               apply_prepare_on_all_hosts,
            prepare_on_up_or_add_host          apply_prepare_on_up_or_add_host,
            protocol_version                   apply_protocol_version,
            randomized_contact_points          apply_randomized_contact_points,
            reconnect_wait_time                apply_reconnect_wait_time,
            request_timeout                    apply_request_timeout,
            resolve_timeout                    apply_resolve_timeout,
            retry_policy                       apply_retry_policy,
            schema_metadata                    apply_schema_metadata,
            serial_consistency                 apply_serial_consistency,
            speculative_execution_policy       apply_speculative_execution_policy,
            ssl                                apply_ssl,
            tcp_keepalive                      apply_tcp_keepalive,
            tcp_nodelay                        apply_tcp_nodelay,
            timestamp_gen                      apply_timestamp_gen,
            token_aware_routing_policy         apply_token_aware_routing_policy,
            tracing_consistency                apply_tracing_consistency,
            tracing_max_wait_time              apply_tracing_max_wait_time,
            tracing_retry_wait_time            apply_tracing_retry_wait_time,
            whitelisted_datacenters            apply_whitelisted_datacenters,
            whitelisted_hosts                  apply_whitelisted_hosts,
        );

        // The `keyspace` and `page_size` fields are not in the `Cluster`
        // configuration. They are set on the session object instead upon
        // creation.

        Ok(cluster)
    }
}

fn apply_appication_name(
    cluster: &mut Cluster,
    name: Option<String>,
) -> Result<(), SessionConfigError> {
    let option = "application_name";

    name.map(|n| cluster.set_application_name(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_appication_version(
    cluster: &mut Cluster,
    version: Option<String>,
) -> Result<(), SessionConfigError> {
    let option = "application_version";

    version
        .map(|v| cluster.set_application_version(v))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_authenticator(
    cluster: &mut Cluster,
    authenticator: Option<Authenticator>,
) -> Result<(), SessionConfigError> {
    if matches!(authenticator, None | Some(Authenticator::None)) {
        return Ok(());
    }

    let option = "authenticator";

    // Plain text authenticator is the only authenticator we support at the
    // moment.
    if let Some(Authenticator::PlainText {
        username,
        password,
    }) = authenticator
    {
        cluster
            .set_credentials(username, password)
            .map_err(|e| SessionConfigError::new(option, e))?;

        return Ok(());
    }

    // The authenticator is not supported, return an error.
    Err(SessionConfigError::new(
        option,
        DriverError::with_message(
            DriverErrorKind::LibBadParams,
            format!("unsupported authenticator: {:?}", authenticator),
        ),
    ))
}

fn apply_blacklisted_datacenters(
    cluster: &mut Cluster,
    datacenters: Vec<String>,
) -> Result<(), SessionConfigError> {
    let option = "blacklisted_datacenters";

    into_comma_separated_string(datacenters)
        .map(|d| cluster.set_blacklist_dc_filtering(d))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_blacklisted_hosts(
    cluster: &mut Cluster,
    hosts: Vec<Host>,
) -> Result<(), SessionConfigError> {
    let option = "blacklisted_hosts";

    into_comma_separated_string(hosts)
        .map(|h| cluster.set_blacklist_filtering(h))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_client_id(
    cluster: &mut Cluster,
    client_id: Option<CqlUuid>,
) -> Result<(), SessionConfigError> {
    let option = "client_id";

    client_id
        .map(|id| cluster.set_client_id(id))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_consistency(
    cluster: &mut Cluster,
    consistency: Option<Consistency>,
) -> Result<(), SessionConfigError> {
    let option = "consistency";

    consistency
        .map(|c| cluster.set_consistency(c))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_connect_timeout(
    cluster: &mut Cluster,
    timeout: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "connect_timeout";

    timeout
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_connect_timeout(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_connection_heartbeat_interval(
    cluster: &mut Cluster,
    interval: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "connection_heartbeat_interval";

    interval
        .map(|i| cluster.set_connection_heartbeat_interval(i.as_secs()))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_connection_idle_timeout(
    cluster: &mut Cluster,
    timeout: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "connection_idle_timeout";

    timeout
        .map(|t| cluster.set_connection_idle_timeout(t.as_secs()))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_contact_points(
    cluster: &mut Cluster,
    contact_points: Vec<ContactPoint>,
) -> Result<(), SessionConfigError> {
    if contact_points.is_empty() {
        return Ok(());
    }

    let option = "contact_points";

    let hosts = contact_points
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
        .join(",");

    cluster
        .set_contact_points(hosts)
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_core_connections_per_host(
    cluster: &mut Cluster,
    connections: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "core_connections_per_host";

    connections
        .map(|c| cluster.set_core_connections_per_host(c))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_event_queue_size(
    cluster: &mut Cluster,
    size: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "event_queue_size";

    size.map(|n| cluster.set_queue_size_event(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_io_queue_size(
    cluster: &mut Cluster,
    size: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "io_queue_size";

    size.map(|n| cluster.set_queue_size_io(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_io_threads_count(
    cluster: &mut Cluster,
    threads: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "io_threads_count";

    threads
        .map(|n| cluster.set_num_threads_io(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_latency_aware_routing_policy(
    cluster: &mut Cluster,
    policy: Option<LatencyAwareRoutingPolicy>,
) -> Result<(), SessionConfigError> {
    match policy {
        Some(LatencyAwareRoutingPolicy::Standard {
            settings,
        }) => {
            enable_latency_aware_routing(cluster)?;

            if let Some(settings) = settings {
                apply_latency_aware_routing_policy_settings(cluster, settings)?;
            }
        }
        Some(LatencyAwareRoutingPolicy::None) => {
            disable_latency_aware_routing(cluster)?;
        }
        None => {}
    };

    Ok(())
}

fn apply_load_balancing_policy(
    cluster: &mut Cluster,
    policy: Option<LoadBalancingPolicy>,
) -> Result<(), SessionConfigError> {
    let option = "load_balancing_policy";

    match policy {
        Some(LoadBalancingPolicy::DatacenterAware {
            local_dc,
        }) => {
            // The empty string is used to indicate that the local datacenter
            // should be automatically determined.
            //
            // However, there is a bug in the driver that causes the driver to
            // return an error when the local datacenter is not set.
            //
            // See https://datastax-oss.atlassian.net/browse/CPP-998
            // TODO: What should we do about this? Any workarounds should go
            //       to the `Cluster` implementation. For now, let's replicate
            //       the behavior of the driver.
            let local_dc = local_dc.unwrap_or_default();
            cluster
                .set_load_balance_dc_aware(local_dc)
                .map_err(|e| SessionConfigError::new(option, e))?;
        }
        Some(LoadBalancingPolicy::RoundRobin) => {
            cluster
                .set_load_balance_round_robin()
                .map_err(|e| SessionConfigError::new(option, e))?;
        }
        None => {}
    };

    Ok(())
}

fn apply_max_connections_per_host(
    cluster: &mut Cluster,
    connections: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "max_connections_per_host";

    connections
        .map(|n| cluster.set_max_connections_per_host(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_max_concurrent_creation(
    cluster: &mut Cluster,
    max_concurrent_creation: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "max_concurrent_creation";

    max_concurrent_creation
        .map(|n| cluster.set_max_concurrent_creation(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_max_reusable_write_objects(
    cluster: &mut Cluster,
    max_reusable_write_objects: Option<usize>,
) -> Result<(), SessionConfigError> {
    let option = "max_reusable_write_objects";

    max_reusable_write_objects
        .map(|n| cluster.set_max_reusable_write_objects(n))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_max_schema_wait_time(
    cluster: &mut Cluster,
    wait_time: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "max_schema_wait_time";

    wait_time
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_max_schema_wait_time(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_metrics_histogram_refresh_interval(
    cluster: &mut Cluster,
    interval: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "metrics_histogram_refresh_interval";

    interval
        .map(|i| into_non_zero_milliseconds(i, option))
        .transpose()?
        .map(|i| cluster.set_histogram_refresh_interval(i))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_monitor_reporting_interval(
    cluster: &mut Cluster,
    interval: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "monitor_reporting_interval";

    interval
        .map(|i| cluster.set_monitor_reporting_interval(i.as_secs()))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_no_compact(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "no_compact";

    enabled
        .map(|enabled| cluster.set_no_compact(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_port(
    cluster: &mut Cluster,
    port: Option<u16>,
) -> Result<(), SessionConfigError> {
    let option = "port";

    port.map(|p| cluster.set_port(p))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_prepare_on_all_hosts(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "prepare_on_all_hosts";

    enabled
        .map(|enabled| cluster.set_prepare_on_all_hosts(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_prepare_on_up_or_add_host(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "prepare_on_up_or_add_host";

    enabled
        .map(|enabled| cluster.set_prepare_on_up_or_add_host(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_protocol_version(
    cluster: &mut Cluster,
    version: Option<ProtocolVersion>,
) -> Result<(), SessionConfigError> {
    let option = "protocol_version";

    version
        .map(|v| cluster.set_protocol_version(v))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_randomized_contact_points(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "randomized_contact_points";

    enabled
        .map(|enabled| cluster.set_use_randomized_contact_points(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_reconnect_wait_time(
    cluster: &mut Cluster,
    wait_time: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "reconnect_wait_time";

    wait_time
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_reconnect_wait_time(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_request_timeout(
    cluster: &mut Cluster,
    timeout: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "request_timeout";

    timeout
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_request_timeout(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_resolve_timeout(
    cluster: &mut Cluster,
    timeout: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "resolve_timeout";

    timeout
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_resolve_timeout(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_retry_policy(
    cluster: &mut Cluster,
    policy: Option<RetryPolicy>,
) -> Result<(), SessionConfigError> {
    let option = "retry_policy";

    policy
        .map(|p| cluster.set_retry_policy(&p.into()))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_schema_metadata(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "schema_metadata";

    enabled
        .map(|enabled| cluster.set_use_schema(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_serial_consistency(
    cluster: &mut Cluster,
    consistency: Option<Consistency>,
) -> Result<(), SessionConfigError> {
    let option = "serial_consistency";

    consistency
        .map(|c| cluster.set_serial_consistency(c))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_speculative_execution_policy(
    cluster: &mut Cluster,
    policy: Option<SpeculativeExecutionPolicy>,
) -> Result<(), SessionConfigError> {
    let option = "speculative_execution_policy";

    match policy {
        Some(SpeculativeExecutionPolicy::Constant {
            delay,
            max_executions,
        }) => {
            let delay = into_milliseconds(delay, option)?;
            cluster
                .set_constant_speculative_execution_policy(
                    delay,
                    max_executions.get(),
                )
                .map_err(|e| SessionConfigError::new(option, e))?;
        }
        Some(SpeculativeExecutionPolicy::None) => {
            cluster
                .set_no_speculative_execution_policy()
                .map_err(|e| SessionConfigError::new(option, e))?;
        }
        None => {}
    };

    Ok(())
}

fn apply_ssl(
    cluster: &mut Cluster,
    ssl: Option<Ssl>,
) -> Result<(), SessionConfigError> {
    let option = "ssl";

    if let Some(ssl) = ssl {
        cluster
            .set_ssl(&ssl)
            .map_err(|e| SessionConfigError::new(option, e))?;
    }

    Ok(())
}

fn apply_tcp_keepalive(
    cluster: &mut Cluster,
    keepalive: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "tcp_keepalive";

    keepalive
        .map(|k| into_milliseconds(k, option))
        .transpose()?
        // Disable the keepalive if the duration is zero.
        .map(|k| if k == 0 { None } else { Some(k) })
        .map(|k| cluster.set_tcp_keepalive(k))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_tcp_nodelay(
    cluster: &mut Cluster,
    enabled: Option<bool>,
) -> Result<(), SessionConfigError> {
    let option = "tcp_nodelay";

    enabled
        .map(|enabled| cluster.set_tcp_nodelay(enabled))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_timestamp_gen(
    cluster: &mut Cluster,
    timestamp_gen: Option<TimestampGen>,
) -> Result<(), SessionConfigError> {
    let option = "timestamp_gen";

    timestamp_gen
        .map(|t| cluster.set_timestamp_gen(&t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_token_aware_routing_policy(
    cluster: &mut Cluster,
    policy: Option<TokenAwareRoutingPolicy>,
) -> Result<(), SessionConfigError> {
    let option = "token_aware_routing_policy";

    match policy {
        Some(TokenAwareRoutingPolicy::Standard {
            shuffle_replicas,
        }) => {
            set_token_aware_routing(cluster, true)?;

            if let Some(enabled) = shuffle_replicas {
                cluster
                    .set_token_aware_routing_shuffle_replicas(enabled)
                    .map_err(|e| SessionConfigError::new(option, e))?;
            }
        }
        // Disable token-aware routing.
        Some(TokenAwareRoutingPolicy::None) => {
            set_token_aware_routing(cluster, false)?;
        }
        None => {}
    };

    Ok(())
}

fn apply_tracing_consistency(
    cluster: &mut Cluster,
    consistency: Option<Consistency>,
) -> Result<(), SessionConfigError> {
    let option = "tracing_consistency";

    consistency
        .map(|c| cluster.set_tracing_consistency(c))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_tracing_max_wait_time(
    cluster: &mut Cluster,
    wait_time: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "tracing_max_wait_time";

    wait_time
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_tracing_max_wait_time(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_tracing_retry_wait_time(
    cluster: &mut Cluster,
    wait_time: Option<Duration>,
) -> Result<(), SessionConfigError> {
    let option = "tracing_retry_wait_time";

    wait_time
        .map(|t| into_milliseconds(t, option))
        .transpose()?
        .map(|t| cluster.set_tracing_retry_wait_time(t))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_whitelisted_datacenters(
    cluster: &mut Cluster,
    datacenters: Vec<String>,
) -> Result<(), SessionConfigError> {
    let option = "whitelisted_datacenters";

    into_comma_separated_string(datacenters)
        .map(|d| cluster.set_whitelist_dc_filtering(d))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_whitelisted_hosts(
    cluster: &mut Cluster,
    hosts: Vec<Host>,
) -> Result<(), SessionConfigError> {
    let option = "whitelisted_hosts";

    into_comma_separated_string(hosts)
        .map(|h| cluster.set_whitelist_filtering(h))
        .transpose()
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn set_token_aware_routing(
    cluster: &mut Cluster,
    enabled: bool,
) -> Result<(), SessionConfigError> {
    let option = "token_aware_routing_policy";

    cluster
        .set_token_aware_routing(enabled)
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn disable_latency_aware_routing(
    cluster: &mut Cluster,
) -> Result<(), SessionConfigError> {
    set_latency_aware_routing(cluster, false)
}

fn enable_latency_aware_routing(
    cluster: &mut Cluster,
) -> Result<(), SessionConfigError> {
    set_latency_aware_routing(cluster, true)
}

fn set_latency_aware_routing(
    cluster: &mut Cluster,
    enabled: bool,
) -> Result<(), SessionConfigError> {
    let option = "latency_aware_routing_policy";

    cluster
        .set_latency_aware_routing(enabled)
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn apply_latency_aware_routing_policy_settings(
    cluster: &mut Cluster,
    settings: StandardLatencyAwareRoutingPolicySettings,
) -> Result<(), SessionConfigError> {
    let scale = into_milliseconds(
        settings.scale,
        "latency_aware_routing_policy.scale",
    )?;

    let retry_period = into_milliseconds(
        settings.retry_period,
        "latency_aware_routing_policy.retry_period",
    )?;

    let update_rate = into_milliseconds(
        settings.update_rate,
        "latency_aware_routing_policy.update_rate",
    )?;

    let option = "latency_aware_routing_policy";
    cluster
        .set_latency_aware_routing_settings(
            settings.exclusion_threshold,
            scale,
            retry_period,
            update_rate,
            settings.min_measures,
        )
        .map_err(|e| SessionConfigError::new(option, e))?;

    Ok(())
}

fn into_milliseconds<S>(
    duration: Duration,
    option: S,
) -> Result<u64, SessionConfigError>
where
    S: Into<String>,
{
    duration
        .as_millis()
        .try_into()
        .map_err(|e: TryFromIntError| {
            SessionConfigError::new(
                option,
                DriverError::with_message(
                    DriverErrorKind::LibBadParams,
                    e.to_string(),
                ),
            )
        })
}

fn into_non_zero_milliseconds<S>(
    duration: Duration,
    option: S,
) -> Result<NonZeroU64, SessionConfigError>
where
    S: Into<String>,
{
    let option = option.into();
    let milliseconds = into_milliseconds(duration, &option)?;

    NonZeroU64::new(milliseconds).ok_or_else(|| {
        SessionConfigError::with_error(
            option,
            "duration must be greater than zero",
        )
    })
}

fn into_comma_separated_string<T>(values: Vec<T>) -> Option<String>
where
    T: Into<String>,
{
    if values.is_empty() {
        None
    } else {
        Some(
            values
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>()
                .join(","),
        )
    }
}
