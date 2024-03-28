use crate::{
    Consistency,
    ContactPoint,
    DriverConfig,
    ProtocolVersion,
    Ssl,
};

/// A builder for the driver configuration.
pub struct DriverConfigBuilder {
    config: DriverConfig,
}

impl DriverConfigBuilder {
    /// Creates a new driver configuration builder.
    pub fn new() -> Self {
        Self {
            config: DriverConfig::default(),
        }
    }

    /// Adds a contact point to the driver configuration.
    ///
    /// If no contact points are added, the driver will connect to localhost by
    /// default.
    pub fn contact_point(mut self, contact_point: ContactPoint) -> Self {
        self.config.contact_points.push(contact_point);

        self
    }

    /// Adds multiple contact points to the driver configuration.
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
    /// The default value is [`CassProtocolVersion::V4`].
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

    /// Builds the driver configuration consuming the builder.
    pub fn build(mut self) -> DriverConfig {
        if self.config.contact_points.is_empty() {
            // Connect to localhost by default.
            self.config.contact_points.push(ContactPoint::default());
        }

        self.config
    }
}

impl Default for DriverConfigBuilder {
    /// Creates a new driver configuration builder.
    fn default() -> Self {
        Self::new()
    }
}
