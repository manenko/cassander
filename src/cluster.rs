use std::ffi::c_char;
use std::num::NonZeroI64;

use crate::ffi::{
    cass_cluster_free,
    cass_cluster_new,
    cass_cluster_set_application_name_n,
    cass_cluster_set_application_version_n,
    cass_cluster_set_blacklist_dc_filtering_n,
    cass_cluster_set_blacklist_filtering_n,
    cass_cluster_set_client_id,
    cass_cluster_set_connect_timeout,
    cass_cluster_set_connection_heartbeat_interval,
    cass_cluster_set_connection_idle_timeout,
    cass_cluster_set_consistency,
    cass_cluster_set_constant_speculative_execution_policy,
    cass_cluster_set_contact_points_n,
    cass_cluster_set_core_connections_per_host,
    cass_cluster_set_credentials_n,
    cass_cluster_set_histogram_refresh_interval,
    cass_cluster_set_latency_aware_routing,
    cass_cluster_set_latency_aware_routing_settings,
    cass_cluster_set_load_balance_dc_aware_n,
    cass_cluster_set_load_balance_round_robin,
    cass_cluster_set_max_concurrent_creation,
    cass_cluster_set_max_connections_per_host,
    cass_cluster_set_max_reusable_write_objects,
    cass_cluster_set_max_schema_wait_time,
    cass_cluster_set_monitor_reporting_interval,
    cass_cluster_set_no_compact,
    cass_cluster_set_no_speculative_execution_policy,
    cass_cluster_set_num_threads_io,
    cass_cluster_set_port,
    cass_cluster_set_prepare_on_all_hosts,
    cass_cluster_set_prepare_on_up_or_add_host,
    cass_cluster_set_protocol_version,
    cass_cluster_set_queue_size_event,
    cass_cluster_set_queue_size_io,
    cass_cluster_set_reconnect_wait_time,
    cass_cluster_set_request_timeout,
    cass_cluster_set_resolve_timeout,
    cass_cluster_set_retry_policy,
    cass_cluster_set_serial_consistency,
    cass_cluster_set_ssl,
    cass_cluster_set_tcp_keepalive,
    cass_cluster_set_tcp_nodelay,
    cass_cluster_set_timestamp_gen,
    cass_cluster_set_token_aware_routing,
    cass_cluster_set_token_aware_routing_shuffle_replicas,
    cass_cluster_set_tracing_consistency,
    cass_cluster_set_tracing_max_wait_time,
    cass_cluster_set_tracing_retry_wait_time,
    cass_cluster_set_use_randomized_contact_points,
    cass_cluster_set_use_schema,
    cass_cluster_set_whitelist_dc_filtering_n,
    cass_cluster_set_whitelist_filtering_n,
    struct_CassCluster_,
};
use crate::{
    Consistency,
    DriverError,
    ProtocolVersion,
    Ssl,
    TimestampGen,
};

// TODO: `cass_cluster_set_authenticator_callbacks`
// TODO: `cass_cluster_set_timestamp_gen`
// TODO: `cass_cluster_set_execution_profile`
// TODO: `cass_cluster_set_host_listener_callback`
// TODO: `cass_cluster_set_cloud_secure_connection_bundle_n`
// TODO: `cass_cluster_set_cloud_secure_connection_bundle_no_ssl_lib_init_n`

/// A cluster object describes the configuration of the Cassandra cluster and is
/// used to construct a session instance.
///
/// Unlike other DataStax drivers the cluster object does not maintain the
/// control connection.
#[repr(transparent)]
pub struct Cluster(*mut struct_CassCluster_);

impl Cluster {
    /// Creates a new cluster object.
    pub fn new() -> Self {
        unsafe { Cluster(cass_cluster_new()) }
    }

    /// Returns the raw pointer to the cluster object.
    pub(crate) fn inner(&self) -> *mut struct_CassCluster_ {
        self.0
    }

    /// Sets/Appends contact points.
    ///
    /// This MUST be set.
    ///
    /// The first call sets the contact points and any subsequent calls appends
    /// additional contact points. Passing an empty string will clear the
    /// contact points. White space is striped from the contact points.
    pub fn set_contact_points<T>(
        &mut self,
        contact_points: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let contact_points = contact_points.as_ref();
        let len = contact_points.len();
        let ptr = contact_points.as_ptr() as *const c_char;
        unsafe { cass_cluster_set_contact_points_n(self.inner(), ptr, len) }
            .into()
    }

    /// Sets the port.
    ///
    /// The default value is 9042.
    pub fn set_port(&mut self, port: u16) -> Result<(), DriverError> {
        let port = port as i32;
        unsafe { cass_cluster_set_port(self.inner(), port) }.into()
    }

    /// Sets the SSL context and enables SSL.
    pub fn set_ssl(&mut self, ssl: &Ssl) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_ssl(self.inner(), ssl.inner()) };

        CassError::Ok
    }

    /// Sets the Apache Cassandra protocol version.
    ///
    /// This will automatically downgrade to the lowest supported protocol
    /// version.
    ///
    /// The default value is [`CassProtocolVersion::V4`].
    pub fn set_protocol_version(
        &mut self,
        version: ProtocolVersion,
    ) -> Result<(), DriverError> {
        let version = cass_try_into!(u32::from(version));
        unsafe { cass_cluster_set_protocol_version(self.inner(), version) }
            .into()
    }

    /// Sets the default consistency level of a statement.
    ///
    /// The default value is [`CassConsistency::LocalOne`].
    pub fn set_consistency(
        &mut self,
        consistency: CassConsistency,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_consistency(self.inner(), consistency.into())
        }
        .into()
    }

    /// Sets the default serial consistency level of a statement.
    ///
    /// The default value is [`CassConsistency::Any`].
    pub fn set_serial_consistency(
        &mut self,
        consistency: Consistency,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_serial_consistency(
                self.inner(),
                consistency.to_driver(),
            )
        }
        .into()
    }

    /// Sets the number of I/O threads.
    ///
    /// This is the number of threads that will handle query requests.
    ///
    /// The default value is 1.
    pub fn set_num_threads_io(
        &mut self,
        num_threads: usize,
    ) -> Result<(), DriverError> {
        let num_threads = cass_try_into!(num_threads);
        unsafe { cass_cluster_set_num_threads_io(self.inner(), num_threads) }
            .into()
    }

    /// Sets the size of the fixed size queue that stores pending requests.
    ///
    /// The default value is 8192.
    pub fn set_queue_size_io(
        &mut self,
        queue_size: usize,
    ) -> Result<(), DriverError> {
        let queue_size = cass_try_into!(queue_size);
        unsafe { cass_cluster_set_queue_size_io(self.inner(), queue_size) }
            .into()
    }

    /// Sets the size of the fixed size queue that stores events.
    ///
    /// The default value is 8192.
    pub fn set_queue_size_event(
        &mut self,
        queue_size: usize,
    ) -> Result<(), DriverError> {
        let queue_size = cass_try_into!(queue_size);
        unsafe { cass_cluster_set_queue_size_event(self.inner(), queue_size) }
            .into()
    }

    /// Sets the number of connections made to each server in each IO thread.
    ///
    /// The default value is 1.
    pub fn set_core_connections_per_host(
        &mut self,
        num_connections: usize,
    ) -> Result<(), DriverError> {
        let num_connections = cass_try_into!(num_connections);
        unsafe {
            cass_cluster_set_core_connections_per_host(
                self.inner(),
                num_connections,
            )
        }
        .into()
    }

    /// Sets the maximum number of connections made to each server in each IO
    /// thread.
    ///
    /// The default value is 2.
    pub fn set_max_connections_per_host(
        &mut self,
        num_connections: usize,
    ) -> Result<(), DriverError> {
        let num_connections = cass_try_into!(num_connections);
        unsafe {
            cass_cluster_set_max_connections_per_host(
                self.inner(),
                num_connections,
            )
        }
        .into()
    }

    /// Sets the wait time in milliseconds before attempting to reconnect.
    ///
    /// The default value is 2000ms.
    pub fn set_reconnect_wait_time(
        &mut self,
        wait_time: i64,
    ) -> Result<(), DriverError> {
        let wait_time = cass_try_into!(wait_time);
        unsafe {
            cass_cluster_set_reconnect_wait_time(self.inner(), wait_time)
        };

        CassError::Ok
    }

    /// Sets the maximum number of connections that will be created
    /// concurrently.
    ///
    /// Connections are created when the current connections are unable to keep
    /// up with request throughput.
    ///
    /// The default value is 1.
    pub fn set_max_concurrent_creation(
        &mut self,
        num_connections: usize,
    ) -> Result<(), DriverError> {
        let num_connections = cass_try_into!(num_connections);
        unsafe {
            cass_cluster_set_max_concurrent_creation(
                self.inner(),
                num_connections,
            )
            .into()
        }
    }

    /// Sets the timeout in milliseconds for connecting to a node.
    ///
    /// The default value is 5000ms.
    pub fn set_connect_timeout(
        &mut self,
        timeout: i64,
    ) -> Result<(), DriverError> {
        let timeout = cass_try_into!(timeout);
        unsafe { cass_cluster_set_connect_timeout(self.inner(), timeout) };

        CassError::Ok
    }

    /// Sets the timeout in milliseconds for waiting for a response from a node.
    ///
    /// The default value is 12000ms.
    pub fn set_request_timeout(
        &mut self,
        timeout: i64,
    ) -> Result<(), DriverError> {
        let timeout = cass_try_into!(timeout);
        unsafe { cass_cluster_set_request_timeout(self.inner(), timeout) };

        CassError::Ok
    }

    /// Sets the timeout in milliseconds for waiting for DNS name resolution.
    pub fn set_resolve_timeout(
        &mut self,
        timeout: i64,
    ) -> Result<(), DriverError> {
        let timeout = cass_try_into!(timeout);
        unsafe { cass_cluster_set_resolve_timeout(self.inner(), timeout) };

        CassError::Ok
    }

    /// Sets the maximum time in milliseconds to wait for schema agreement after
    /// a schema change is made (e.g. creating, altering, dropping a
    /// table/keyspace/view/index etc).
    ///
    /// The default value is 10000ms.
    pub fn set_max_schema_wait_time(
        &mut self,
        wait_time: i64,
    ) -> Result<(), DriverError> {
        if let Ok(wait_time) = wait_time.try_into() {
            unsafe {
                cass_cluster_set_max_schema_wait_time(self.inner(), wait_time)
            };

            CassError::Ok
        } else {
            CassError::LibBadParams
        }
    }

    /// Sets the maximum time in milliseconds to wait for tracing data to become
    /// available.
    ///
    /// The default value is 15ms.
    pub fn set_tracing_max_wait_time(
        &mut self,
        wait_time: i64,
    ) -> Result<(), DriverError> {
        if let Ok(wait_time) = wait_time.try_into() {
            unsafe {
                cass_cluster_set_tracing_max_wait_time(self.inner(), wait_time)
            };

            CassError::Ok
        } else {
            CassError::LibBadParams
        }
    }

    /// Sets the amount of time to wait between attempts to check to see if
    /// tracing is available.
    ///
    /// The default value is 3ms.
    pub fn set_tracing_retry_wait_time(
        &mut self,
        wait_time: i64,
    ) -> Result<(), DriverError> {
        if let Ok(wait_time) = wait_time.try_into() {
            unsafe {
                cass_cluster_set_tracing_retry_wait_time(
                    self.inner(),
                    wait_time,
                )
            };

            CassError::Ok
        } else {
            CassError::LibBadParams
        }
    }

    /// Sets the consistency level to use for checking to see if tracing data is
    /// available.
    ///
    /// The default value is [`CassConsistency::One`].
    pub fn set_tracing_consistency(
        &mut self,
        consistency: Consistency,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_tracing_consistency(
                self.inner(),
                consistency.into(),
            )
        };

        CassError::Ok
    }

    /// Sets credentials for plain text authentication.
    pub fn set_credentials<A, B>(
        &mut self,
        username: A,
        password: B,
    ) -> Result<(), DriverError>
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let username = username.as_ref();
        let username_len = username.len();
        let username_ptr = username.as_ptr() as *const c_char;

        let password = password.as_ref();
        let password_len = password.len();
        let password_ptr = password.as_ptr() as *const c_char;

        unsafe {
            cass_cluster_set_credentials_n(
                self.inner(),
                username_ptr,
                username_len,
                password_ptr,
                password_len,
            )
        };

        CassError::Ok
    }

    /// Configures the cluster to use round-robin load balancing.
    ///
    /// The driver discovers all nodes in a cluster and cycles through them per
    /// request. All are considered 'local'.
    pub fn set_load_balance_round_robin(&mut self) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_load_balance_round_robin(self.inner()) };

        CassError::Ok
    }

    /// Configures the cluster to use DC-aware load balancing.
    ///
    /// For each query, all live nodes in a primary 'local' datacenter are tried
    /// first, followed by any node from other datacenters.
    ///
    /// This is the default and does not need to be called unless switching from
    /// another policy or changing settings. Without further configuration, a
    /// default `local_dc` is chosen from the first connected contact point, and
    /// no remote hosts are considered in query plans. If relying on this
    /// mechanism, be sure to use only contact points from the local datacenter.
    ///
    /// <div class="warning">
    /// The remote datacenter settings for DC-aware load balacing are not
    /// suitable for most scenarios that require a datacenter failover. There is
    /// also unhandled gap between the number of nodes failing and the full
    /// cluster failing. Thus the remote datacenter settings are deprecated:
    ///
    /// - `used_hosts_per_remote_dc`
    /// - `allow_remote_dcs_for_local_cl`
    /// </div>
    pub fn set_load_balance_dc_aware<T>(
        &mut self,
        local_dc: T,
        used_hosts_per_remote_dc: usize,
        allow_remote_dcs_for_local_cl: bool,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let local_dc = local_dc.as_ref();
        let local_dc_len = local_dc.len();
        let local_dc_ptr = local_dc.as_ptr() as *const c_char;

        if let Ok(used_hosts_per_remote_dc) =
            used_hosts_per_remote_dc.try_into()
        {
            unsafe {
                cass_cluster_set_load_balance_dc_aware_n(
                    self.inner(),
                    local_dc_ptr,
                    local_dc_len,
                    used_hosts_per_remote_dc,
                    allow_remote_dcs_for_local_cl.into(),
                )
            }
            .into()
        } else {
            CassError::LibBadParams
        }
    }

    /// Configures the cluster to use token-aware request routing.
    ///
    /// <div class="warning">
    /// Token-aware routing depends on keyspace metadata. For this reason
    /// enabling token-aware routing will also enable retrieving and updating
    /// keyspace schema metadata.
    /// </div>
    ///
    /// This routing policy composes the base routing policy, routing requests
    /// first to replicas on nodes considered 'local' by the base load balancing
    /// policy.
    ///
    /// The default value is `true`.
    pub fn set_token_aware_routing(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_token_aware_routing(self.inner(), enabled.into())
        };

        CassError::Ok
    }

    /// Configures token-aware routing to randomly shuffle replicas.
    ///
    /// This can reduce the effectiveness of server-side caching, but it can
    /// better distribute load over replicas for a given partition key.
    ///
    /// Token-aware routing must be enabled for the setting to have any effect.
    ///
    /// The default value is `true`.
    pub fn set_token_aware_routing_shuffle_replicas(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_token_aware_routing_shuffle_replicas(
                self.inner(),
                enabled.into(),
            )
        };

        CassError::Ok
    }

    /// Configures the cluster to use latency-aware routing.
    ///
    /// This routing policy is a top-level routing policy. It uses the base
    /// routing policy to determine locality (dc-aware) and/or placement
    /// (token-aware) before considering the latency.
    ///
    /// The default value is `false`.
    pub fn set_latency_aware_routing(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_latency_aware_routing(self.inner(), enabled.into())
        };

        CassError::Ok
    }

    /// Configures the cluster settings for latency-aware request routing.
    ///
    /// - `exclusion_threshold` controls how much worse the latency must be
    /// compared to the average latency of the best performing node before it
    /// penalized.
    /// - `scale` controls the weight given to older latencies when calculating
    /// the average latency of a node. A bigger scale will give more weight to
    /// older latency measurements. Measured in milliseconds.
    /// - `retry_period` is the amount of time a node is penalized by the policy
    /// before being given a second chance when the current average latency
    /// exceeds the calculated threshold, which is `exclusion_threshold *
    /// best_average_latency`. Measured in milliseconds.
    /// - `update_rate` is the rate at which the best average latency is
    /// recomputed. Measured in milliseconds.
    /// - `min_measured` is the minimum number of measurements per-host required
    /// to be considered by the policy.
    ///
    /// The default settings are:
    /// - `exclusion_threshold`: 2.0
    /// - `scale`: 100ms
    /// - `retry_period`: 10000ms
    /// - `update_rate`: 100ms
    /// - `min_measured`: 50
    pub fn set_latency_aware_routing_settings(
        &mut self,
        exclusion_threshold: f64,
        scale: i64,
        retry_period: i64,
        update_rate: i64,
        min_measured: usize,
    ) -> Result<(), DriverError> {
        let scale = cass_try_into!(scale);
        let retry_period = cass_try_into!(retry_period);
        let update_rate = cass_try_into!(update_rate);
        let min_measured = cass_try_into!(min_measured);

        unsafe {
            cass_cluster_set_latency_aware_routing_settings(
                self.inner(),
                exclusion_threshold,
                scale,
                retry_period,
                update_rate,
                min_measured,
            )
        };

        CassError::Ok
    }

    /// Sets/Appends whitelist hosts.
    ///
    /// The first call sets the whitelist hosts and any subsequent calls appends
    /// additional hosts. Passing an empty string will clear and disable the
    /// whitelist. White space is striped from the hosts.
    ///
    /// This policy filters requests to all other policies, only allowing
    /// requests to the hosts contained in the whitelist. Any host not in the
    /// whitelist will be ignored and a connection will not be established. This
    /// policy is useful for ensuring that the driver will only connect to a
    /// predefined set of hosts.
    pub fn set_whitelist_filtering<T>(
        &mut self,
        hosts: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let hosts = hosts.as_ref();
        let len = hosts.len();
        let ptr = hosts.as_ptr() as *const c_char;
        unsafe {
            cass_cluster_set_whitelist_filtering_n(self.inner(), ptr, len)
        };

        CassError::Ok
    }

    /// Sets/Appends blacklist hosts.
    ///
    /// The first call sets the blacklist hosts and any subsequent calls appends
    /// additional hosts. Passing an empty string will clear and disable the
    /// blacklist. White space is striped from the hosts.
    ///
    /// This policy filters requests to all other policies, only allowing
    /// requests to the hosts not contained in the blacklist. Any host in the
    /// blacklist will be ignored and a connection will not be established. This
    /// policy is useful for ensuring that the driver will not connect to a
    /// predefined set of hosts.
    pub fn set_blacklist_filtering<T>(
        &mut self,
        hosts: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let hosts = hosts.as_ref();
        let len = hosts.len();
        let ptr = hosts.as_ptr() as *const c_char;
        unsafe {
            cass_cluster_set_blacklist_filtering_n(self.inner(), ptr, len)
        };

        CassError::Ok
    }

    /// Same as
    /// [`set_whitelist_filtering`](CassCluster::set_whitelist_filtering) but
    /// whitelists all hosts of a datacenter.
    pub fn set_whitelist_dc_filtering<T>(
        &mut self,
        datacenters: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let dc = datacenters.as_ref();
        let len = dc.len();
        let ptr = dc.as_ptr() as *const c_char;
        unsafe {
            cass_cluster_set_whitelist_dc_filtering_n(self.inner(), ptr, len)
        };

        CassError::Ok
    }

    /// Same as
    /// [`set_blacklist_filtering`](CassCluster::set_blacklist_filtering) but
    /// blacklists all hosts of a datacenter.
    pub fn set_blacklist_dc_filtering<T>(
        &mut self,
        datacenters: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let dc = datacenters.as_ref();
        let len = dc.len();
        let ptr = dc.as_ptr() as *const c_char;
        unsafe {
            cass_cluster_set_blacklist_dc_filtering_n(self.inner(), ptr, len)
        };

        CassError::Ok
    }

    /// Enables/Disables Nagle's algorithm on connections.
    ///
    /// The default value is `true` (disables Nagle's algorithm).
    pub fn set_tcp_nodelay(&mut self, enable: bool) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_tcp_nodelay(self.inner(), enable.into()) };

        CassError::Ok
    }

    /// Enables/Disables TCP keep-alive on connections.
    ///
    /// When `delay` is `None` the TCP keep-alive is disabled. When `delay` is
    /// `Some` the TCP keep-alive is enabled and the delay is set to the
    /// specified value in seconds.
    ///
    /// The default value is `None` (disable TCP keep-alive).
    pub fn set_tcp_keepalive(
        &mut self,
        delay: Option<i64>,
    ) -> Result<(), DriverError> {
        let enable = delay.is_some();
        let delay = cass_try_into!(delay.unwrap_or(0));
        unsafe {
            cass_cluster_set_tcp_keepalive(self.inner(), enable.into(), delay)
        };

        CassError::Ok
    }

    /// Sets the amount of time in seconds between heartbeat messages and
    /// controls the amount of time the connection must be idle before sending
    /// heartbeat messages.
    ///
    /// This is useful for preventing intermediate network devices from dropping
    /// connections.
    ///
    /// The default value is 30 seconds.
    pub fn set_connection_heartbeat_interval(
        &mut self,
        interval: i64,
    ) -> Result<(), DriverError> {
        let interval = cass_try_into!(interval);
        unsafe {
            cass_cluster_set_connection_heartbeat_interval(
                self.inner(),
                interval,
            )
        };

        CassError::Ok
    }

    /// Sets the amount of time in seconds a connection is allowed to be without
    /// a successful heartbeat response before being terminated and scheduled
    /// for reconnection.
    ///
    /// The default value is 60 seconds.
    pub fn set_connection_idle_timeout(
        &mut self,
        timeout: i64,
    ) -> Result<(), DriverError> {
        let timeout = cass_try_into!(timeout);
        unsafe {
            cass_cluster_set_connection_idle_timeout(self.inner(), timeout)
        };

        CassError::Ok
    }

    /// Sets the retry policy used for all requests unless overridden by setting
    /// a retry policy on a statement or a batch.
    ///
    /// The default policy is [`CassRetryPolicy::default`]. This policy will
    /// retry on a read timeout if there was enough replicas, but no data
    /// present, on a write timeout if a logged batch request failed to write
    /// the batch log, and on a unavailable error it retries using a new host.
    /// In all other cases the default policy will return an error.
    pub fn set_retry_policy(
        &mut self,
        policy: &CassRetryPolicy,
    ) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_retry_policy(self.inner(), policy.inner()) };

        CassError::Ok
    }

    /// Enables/Disables retrieving and updating schema metadata.
    ///
    /// If disabled this is allows the driver to skip over retrieving and
    /// updating schema metadata and [`CassSession::get_schema_meta`] will
    /// always return an empty object. This can be useful for reducing the
    /// startup overhead of short-lived sessions.
    ///
    /// The default value is `true` (enabled).
    pub fn set_use_schema(&mut self, enabled: bool) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_use_schema(self.inner(), enabled.into()) };

        CassError::Ok
    }

    /// Enables/Disables the randomization of the contact points list.
    ///
    /// <div class="warning">
    /// This setting should only be disabled for debugging or testing purposes.
    /// </div>
    ///
    /// The default value is `true` (enabled).
    pub fn set_use_randomized_contact_points(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_use_randomized_contact_points(
                self.inner(),
                enabled.into(),
            )
        }
        .into()
    }

    /// Enables constant speculative executions with the supplied settings.
    ///
    /// The `delay` between each speculative execution is measured in
    /// milliseconds. A zero delay means it should immediately send
    /// `max_speculative_executions` requests along with the original request.
    pub fn set_constant_speculative_execution_policy(
        &mut self,
        delay: i64,
        max_speculative_executions: usize,
    ) -> Result<(), DriverError> {
        let max_speculative_executions =
            cass_try_into!(max_speculative_executions);
        unsafe {
            cass_cluster_set_constant_speculative_execution_policy(
                self.inner(),
                delay,
                max_speculative_executions,
            )
        }
        .into()
    }

    /// Disables speculative executions.
    ///
    /// This is the default behavior.
    pub fn set_no_speculative_execution_policy(
        &mut self,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_no_speculative_execution_policy(self.inner())
        }
        .into()
    }

    /// Sets the maximum number of "pending write" objects that will be saved
    /// for re-use for marshalling new requests.
    ///
    /// These objects may hold on to a significant amount of memory and reducing
    /// the number of these objects may reduce memory usage of the application.
    ///
    /// The cost of reducing the value of this setting is potentially slower
    /// marshalling of requests prior to sending.
    ///
    /// The default value is [`u32::MAX`].
    pub fn set_max_reusable_write_objects(
        &mut self,
        num: usize,
    ) -> Result<(), DriverError> {
        let num = cass_try_into!(num);
        unsafe {
            cass_cluster_set_max_reusable_write_objects(self.inner(), num)
        }
        .into()
    }

    /// Enables/Disables preparation of statements on all available hosts.
    ///
    /// The default value is `true`.
    pub fn set_prepare_on_all_hosts(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_prepare_on_all_hosts(self.inner(), enabled.into())
        }
        .into()
    }

    /// Enables/Disables pre-preparing cached prepared statements when existing
    /// hosts become available again or when new hosts are added to the cluster.
    ///
    /// This can help mitigate request latency when executing prepared
    /// statements by avoiding an extra round trip in cases where the statement
    /// is unprepared on a freshly started server. The main tradeoff is extra
    /// background network traffic is required to prepare the statements on
    /// hosts as they become available.
    ///
    /// The default value is `true`.
    pub fn set_prepare_on_up_or_add_host(
        &mut self,
        enabled: bool,
    ) -> Result<(), DriverError> {
        unsafe {
            cass_cluster_set_prepare_on_up_or_add_host(
                self.inner(),
                enabled.into(),
            )
        }
        .into()
    }

    /// Enables/Disables the `NO_COMPACT` startup option.
    ///
    /// This can help facilitate uninterrupted cluster upgrades where tables
    /// using `COMPACT_STORAGE` will operate in "compatibility mode" for
    /// `BATCH`, `DELETE`, `SELECT`, and `UPDATE` CQL operations.
    ///
    /// The default value is `false`.
    pub fn set_no_compact(&mut self, enabled: bool) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_no_compact(self.inner(), enabled.into()) }
            .into()
    }

    /// Sets the application name.
    ///
    /// This is optional; however it provides the server with the application
    /// name that can aid in debugging issues with larger clusters where there
    /// are a lot of client (or application) connections.
    pub fn set_application_name<T>(
        &mut self,
        name: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        let len = name.len();
        let ptr = name.as_ptr() as *const c_char;
        unsafe { cass_cluster_set_application_name_n(self.inner(), ptr, len) };

        CassError::Ok
    }

    /// Sets the application version.
    ///
    /// This is optional; however it provides the server with the application
    /// version that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections that may have
    /// different versions in use.
    pub fn set_application_version<T>(
        &mut self,
        version: T,
    ) -> Result<(), DriverError>
    where
        T: AsRef<str>,
    {
        let version = version.as_ref();
        let len = version.len();
        let ptr = version.as_ptr() as *const c_char;
        unsafe {
            cass_cluster_set_application_version_n(self.inner(), ptr, len)
        };

        CassError::Ok
    }

    /// Sets the client identifier.
    ///
    /// This is optional; however it provides the server with the client
    /// identifier that can aid in debugging issues with larger clusters where
    /// there are a lot of client (or application) connections.
    ///
    /// Default value is a random UUID v4.
    pub fn set_client_id(&mut self, id: CassUuid) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_client_id(self.inner(), id.inner()) };

        CassError::Ok
    }

    /// Sets the amount of time in seconds between monitor reporting event
    /// messages.
    ///
    /// Setting this to zero disables the reporting of event messages.
    ///
    /// The default value is 300 seconds.
    pub fn set_monitor_reporting_interval(
        &mut self,
        interval: i64,
    ) -> Result<(), DriverError> {
        let interval = cass_try_into!(interval);
        unsafe {
            cass_cluster_set_monitor_reporting_interval(self.inner(), interval)
        };

        CassError::Ok
    }

    /// Sets the amount of time in milliseconds after which metric histograms
    /// should be refreshed.
    ///
    /// Upon refresh histograms are reset to zero, effectively dropping any
    /// history to that point. Refresh occurs when a snapshot is requested so
    /// this value should be thought of as a minimum time to refresh.
    ///
    /// If refresh is not enabled the driver will continue to accumulate
    /// histogram data over the life of a session; this is the default behaviour
    /// and replicates the behaviour of previous versions.
    pub fn set_histogram_refresh_interval(
        &mut self,
        interval: NonZeroI64,
    ) -> Result<(), DriverError> {
        let interval = cass_try_into!(interval.get());

        unsafe {
            cass_cluster_set_histogram_refresh_interval(self.inner(), interval)
        }
        .into()
    }

    /// Sets the timestamp generator used to assign timestamps to all requests
    /// unless overridden by setting the timestamp on a statement or a batch.
    ///
    /// The default value is monotonically increasing, client-side timestamp
    /// generator.
    pub fn set_timestamp_gen(
        &mut self,
        gen: &TimestampGen,
    ) -> Result<(), DriverError> {
        unsafe { cass_cluster_set_timestamp_gen(self.inner(), gen.inner()) };

        CassError::Ok
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Cluster {
    fn drop(&mut self) {
        unsafe { cass_cluster_free(self.inner()) }
    }
}
