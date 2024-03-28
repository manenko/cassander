use std::time::Duration;

use crate::{
    Consistency,
    ContactPoint,
    DriverConfigBuilder,
    ProtocolVersion,
    Ssl,
};

// TODO: Implement the `Debug` trait for `DriverConfig`.

/// The driver configuration.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DriverConfig {
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
    /// so because it is all about security. So, deserialize the driver
    /// configuration and then set the SSL configuration manually.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub ssl: Option<Ssl>,

    /// The Apache Cassandra protocol version.
    ///
    /// This will automatically downgrade to the lowest supported protocol
    /// version.
    ///
    /// If not set, the default value is [`CassProtocolVersion::V4`].
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
    ///
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
}

impl DriverConfig {
    /// Creates a new driver configuration builder.
    pub fn builder() -> DriverConfigBuilder {
        DriverConfigBuilder::default()
    }
}
