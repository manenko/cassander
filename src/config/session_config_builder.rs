use std::time::Duration;

use crate::{
    Consistency,
    ContactPoint,
    ProtocolVersion,
    SessionConfig,
    Ssl,
    TimestampGen,
};

/// A builder for the Cassandra session configuration.
pub struct SessionConfigBuilder {
    config: SessionConfig,
}

impl SessionConfigBuilder {
    /// Creates a new Cassandra session configuration builder.
    pub fn new() -> Self {
        Self {
            config: SessionConfig::default(),
        }
    }

    /// Adds a contact point to the session configuration.
    ///
    /// If no contact points are added, the driver will connect to localhost by
    /// default.
    pub fn contact_point(mut self, contact_point: ContactPoint) -> Self {
        self.config.contact_points.push(contact_point);

        self
    }

    /// Adds multiple contact points to the session configuration.
    ///
    /// If no contact points are added, the driver will connect to localhost by
    /// default.
    pub fn contact_points<I>(mut self, contact_points: I) -> Self
    where
        I: IntoIterator<Item = ContactPoint>,
    {
        self.config.contact_points.extend(contact_points);

        self
    }

    /// Sets the port to use for the control connection.
    ///
    /// If not set, the default port 9042 is used.
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = Some(port);

        self
    }

    /// Sets the SSL configuration.
    ///
    /// If not set, SSL is disabled.
    pub fn ssl(mut self, ssl: Ssl) -> Self {
        self.config.ssl = Some(ssl);

        self
    }

    /// Sets the Apache Cassandra protocol version.
    ///
    /// This will automatically downgrade to the lowest supported protocol
    /// version.
    ///
    /// The default value is [`ProtocolVersion::V4`].
    pub fn protocol_version(mut self, version: ProtocolVersion) -> Self {
        self.config.protocol_version = Some(version);

        self
    }

    /// Sets the default consistency level of a statement.
    ///
    /// The default value is [`Consistency::LocalOne`].
    pub fn consistency(mut self, consistency: Consistency) -> Self {
        self.config.consistency = Some(consistency);

        self
    }

    /// Sets the default serial consistency level of a statement.
    ///
    /// The default value is [`Consistency::Any`].
    pub fn serial_consistency(mut self, consistency: Consistency) -> Self {
        self.config.serial_consistency = Some(consistency);

        self
    }

    /// Sets the number of I/O threads that will handle query requests.
    ///
    /// The default value is 1.
    pub fn io_threads_count(mut self, count: usize) -> Self {
        self.config.io_threads_count = Some(count);

        self
    }

    /// Sets the size of the fixed size queue that stores pending requests.
    ///
    /// The default value is 8192.
    pub fn io_queue_size(mut self, size: usize) -> Self {
        self.config.io_queue_size = Some(size);

        self
    }

    /// Sets the size of the fixed size queue that stores events.
    ///
    /// The default value is 8192.
    pub fn event_queue_size(mut self, size: usize) -> Self {
        self.config.event_queue_size = Some(size);

        self
    }

    /// Sets the number of connections made to each server in each I/O thread.
    ///
    /// The default value is 1.
    pub fn core_connections_per_host(mut self, count: usize) -> Self {
        self.config.core_connections_per_host = Some(count);

        self
    }

    /// Sets the maximum number of connections made to each server in each I/O
    /// thread.
    ///
    /// The default value is 2.
    pub fn max_connections_per_host(mut self, count: usize) -> Self {
        self.config.max_connections_per_host = Some(count);

        self
    }

    /// Sets the wait time before attempting to reconnect.
    ///
    /// The default value is 2000 milliseconds.
    pub fn reconnect_wait_time(mut self, duration: Duration) -> Self {
        self.config.reconnect_wait_time = Some(duration);

        self
    }

    /// Sets the maximum number of connections that will be created
    /// concurrently.
    ///
    /// Connections are created when the current connections are unable to keep
    /// up with request throughput.
    ///
    /// The default value is 1.
    pub fn max_concurrent_creation(mut self, count: usize) -> Self {
        self.config.max_concurrent_creation = Some(count);

        self
    }

    /// Sets the timeout for establishing a connection to a Cassandra node.
    ///
    /// The default value is 5000 milliseconds.
    pub fn connect_timeout(mut self, duration: Duration) -> Self {
        self.config.connect_timeout = Some(duration);

        self
    }

    /// Sets the timeout for waiting for a response from a Cassandra node.
    ///
    /// The default value is 12000 milliseconds.
    pub fn request_timeout(mut self, duration: Duration) -> Self {
        self.config.request_timeout = Some(duration);

        self
    }

    /// Sets the timeout for waiting for DNS name resolution.
    ///
    /// The default value is 2000 milliseconds.
    pub fn resolve_timeout(mut self, duration: Duration) -> Self {
        self.config.resolve_timeout = Some(duration);

        self
    }

    /// Sets the maximum time to wait for a schema agreement after a
    /// schema-altering query (e.g. creating, altering, dropping a
    /// table/keyspace/view/index etc).
    ///
    /// The default value is 10000 milliseconds.
    pub fn max_schema_wait_time(mut self, duration: Duration) -> Self {
        self.config.max_schema_wait_time = Some(duration);

        self
    }

    /// Sets the maximum time to wait for tracing data to become available.
    ///
    /// The default value is 15 milliseconds.
    pub fn tracing_max_wait_time(mut self, duration: Duration) -> Self {
        self.config.tracing_max_wait_time = Some(duration);

        self
    }

    /// Sets the amount of time to wait between attempts to check to see if
    /// tracing is available.
    ///
    /// The default value is 3 milliseconds.
    pub fn tracing_retry_wait_time(mut self, duration: Duration) -> Self {
        self.config.tracing_retry_wait_time = Some(duration);

        self
    }

    /// Sets the consistency level to use for checking to see if tracing data is
    /// available.
    ///
    /// The default value is [`Consistency::LocalOne`].
    pub fn tracing_consistency(mut self, consistency: Consistency) -> Self {
        self.config.tracing_consistency = Some(consistency);

        self
    }

    /// Sets the default keyspace to use for all queries.
    ///
    /// There is no keyspace set by default and queries must specify the
    /// keyspace explicitly (`keyspace_name.table_name`).
    pub fn keyspace<S>(mut self, keyspace: S) -> Self
    where
        S: Into<String>,
    {
        self.config.keyspace = Some(keyspace.into());

        self
    }

    /// Sets the default page size for all queries.
    ///
    /// Setting the page size to zero disables paging.
    ///
    /// The paging is disabled by default. It is recommended to set the page
    /// size to a reasonable value. The page size could be set for each query
    /// individually.
    pub fn page_size(mut self, size: usize) -> Self {
        self.config.page_size = Some(size);

        self
    }

    /// Sets the timestamp generator used to assign timestamps to all queries
    /// unless overridden by setting it on a query or batch individually.
    ///
    /// The default value is the monotonically increasing, client-side timestamp
    /// generator.
    pub fn timestamp_gen(mut self, timestamp_gen: TimestampGen) -> Self {
        self.config.timestamp_gen = Some(timestamp_gen);

        self
    }

    /// Enables or disables bypassing Nagle's algorithm.
    ///
    /// When set to `true`, the segments are always sent as soon as possible,
    /// even if there is only a small amount of data.
    ///
    /// When not set to `false`, data is buffered until there is a sufficient
    /// amount to send out, thereby avoiding the frequent sending of small
    /// packets, which results in poor utilization of the network.
    ///
    /// Nagle's algorithm is disabled by default.
    pub fn tcp_nodelay(mut self, enable: bool) -> Self {
        self.config.tcp_nodelay = Some(enable);

        self
    }

    /// Sets the keepalive period for the TCP connection.
    ///
    /// Setting a value greater than [`Duration::ZERO`] enables the keepalive
    /// with the given period. Setting a value of [`Duration::ZERO`] disables
    /// the keepalive.
    ///
    /// The default value is [`Duration::ZERO`].
    pub fn tcp_keepalive(mut self, duration: Duration) -> Self {
        self.config.tcp_keepalive = Some(duration);

        self
    }

    /// Sets the amount of time between heartbeat messages to keep the
    /// connection alive.
    ///
    /// This is useful for preventing intermediate network devices from dropping
    /// connections.
    ///
    /// The default value is 30 seconds.
    pub fn connection_heartbeat_interval(mut self, duration: Duration) -> Self {
        self.config.connection_heartbeat_interval = Some(duration);

        self
    }

    /// Sets the amount of time a connection is allowed to be without
    /// a successful heartbeat response before being terminated and scheduled
    /// for reconnection.
    ///
    /// The default value is 60 seconds.
    pub fn connection_idle_timeout(mut self, duration: Duration) -> Self {
        self.config.connection_idle_timeout = Some(duration);

        self
    }

    /// Sets the amount of time between monitor reporting event messages.
    ///
    /// Setting this value to [`Duration::ZERO`] disables the reporting of
    /// event messages.
    ///
    /// The default value is 300 seconds.
    pub fn monitor_reporting_interval(mut self, duration: Duration) -> Self {
        self.config.monitor_reporting_interval = Some(duration);

        self
    }

    /// Sets the amount of time after which metric histograms should be
    /// refreshed.
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
    pub fn metrics_histogram_refresh_interval(
        mut self,
        duration: Duration,
    ) -> Self {
        self.config.metrics_histogram_refresh_interval = Some(duration);

        self
    }

    /// Enables/disables retrieving and updating schema metadata.
    ///
    /// If disabled, allows the driver to skip over retrieving and updating
    /// schema metadata and [`Session::get_schema_meta`] will always return an
    /// empty object. This can be useful for reducing the startup overhead of
    /// short-lived sessions.
    ///
    /// The schema metadata is enabled by default.
    pub fn schema_metadata(mut self, enable: bool) -> Self {
        self.config.schema_metadata = Some(enable);

        self
    }

    /// Enables/disables the randomization of the contact points list.
    ///
    /// <div class="warning">
    /// This setting should only be disabled for debugging or testing purposes.
    /// </div>
    ///
    /// This is enabled by default.
    pub fn randomized_contact_points(mut self, enable: bool) -> Self {
        self.config.randomized_contact_points = Some(enable);

        self
    }

    /// Sets the maximum number of "pending write" objects that will be saved
    /// for re-use for marshalling new requests.
    ///
    /// These objects may hold on to a significant amount of memory and
    /// reducing the number of these objects may reduce memory usage of the
    /// application.
    ///
    /// The cost of reducing the value of this setting is potentially slower
    /// marshalling of requests prior to sending.
    ///
    /// The default value is [`u32::MAX`].
    pub fn max_reusable_write_objects(mut self, count: usize) -> Self {
        self.config.max_reusable_write_objects = Some(count);

        self
    }

    /// Enables/disables preparation of statements on all available hosts.
    ///
    /// This is enabled by default.
    pub fn prepare_on_all_hosts(mut self, enable: bool) -> Self {
        self.config.prepare_on_all_hosts = Some(enable);

        self
    }

    /// Enables/disables pre-preparing cached prepared statements when existing
    /// hosts become available again or when new hosts are added to the cluster.
    ///
    /// This can help mitigate request latency when executing prepared
    /// statements by avoiding an extra round trip in cases where the statement
    /// is unprepared on a freshly started server. The main tradeoff is extra
    /// background network traffic is required to prepare the statements on
    /// hosts as they become available.
    ///
    /// This is enabled by default.
    pub fn prepare_on_up_or_add_host(mut self, enable: bool) -> Self {
        self.config.prepare_on_up_or_add_host = Some(enable);

        self
    }

    /// Enables/disables the `NO_COMPACT` startup option.
    ///
    /// This can help facilitate uninterrupted cluster upgrades where tables
    /// using `COMPACT_STORAGE` will operate in "compatibility mode" for
    /// `BATCH`, `DELETE`, `SELECT`, and `UPDATE` CQL operations.
    ///
    /// The option is disabled by default.
    pub fn no_compact(mut self, enable: bool) -> Self {
        self.config.no_compact = Some(enable);

        self
    }

    /// Sets the application name.
    ///
    /// This is optional; however it provides the server with the application
    /// name that can aid in debugging issues with larger clusters where there
    /// are a lot of client (or application) connections.
    pub fn application_name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.config.application_name = Some(name.into());

        self
    }

    /// Sets the application version.
    ///
    /// This is optional; however it provides the server with the application
    /// version that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections that may have
    /// different versions in use.
    pub fn application_version<S>(mut self, version: S) -> Self
    where
        S: Into<String>,
    {
        self.config.application_version = Some(version.into());

        self
    }

    /// Sets the client identifier.
    ///
    /// This is optional; however it provides the server with the client
    /// identifier that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections.
    ///
    /// Default value is a random UUID v4 string.
    pub fn client_id<S>(mut self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.config.client_id = Some(id.into());

        self
    }

    /// Builds the session configuration consuming the builder.
    pub fn build(mut self) -> SessionConfig {
        if self.config.contact_points.is_empty() {
            // Connect to localhost by default.
            self.config.contact_points.push(ContactPoint::default());
        }

        self.config
    }
}

impl Default for SessionConfigBuilder {
    /// Creates a new Cassandra session configuration builder.
    fn default() -> Self {
        Self::new()
    }
}
