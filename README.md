# Cassander

Cassander provides Rust developers with a wrapper for the DataStax C/C++ Driver, enabling interaction with Apache Cassandra and DataStax products. Currently under development, users should anticipate possible changes and some bugs.

The library was tested agains C/C++ driver version 2.17.1, but should work with older versions too.


To get started, include Cassander in your `Cargo.toml` as follows:

```toml
[dependencies]
cassander = "0.1"
```

## Prerequisites

The make sure you have the DataStax C/C++ Driver [installed](https://docs.datastax.com/en/developer/cpp-driver/2.17/topics/installation/) and available for dynamic loading.

## Optional Features

Cassander comes with several optional features to enhance its functionality for specific use cases.

### `serde`

The `serde` feature allows for serialization and deserialization of `SessionConfig` through the [`serde`](https://docs.rs/serde/latest/serde/) crate, facilitating the storage of driver configurations in a file for application loading.

Duration values are serialized as strings (e.g., `100ms`, `2s`, `1h10m`) using the [`duration_string`](https://docs.rs/duration-string/latest/duration_string/) crate. These implementation details, however, should not be relied upon by the user.

### `uuid`

This feature introduces conversions between Cassandra's `CqlUuid` and the `Uuid` from the [`uuid`](https://docs.rs/uuid/latest/uuid/) crate, simplifying the handling of UUID values.

## Examples

### Create a session via `SessionConfigBuilder`

The `SessionConfigBuilder` facilitates creating connections to Cassandra clusters with custom settings:

```rust
use anyhow::Result;
use cassander::{SessionConfigBuilder, Authenticator};

#[tokio::main]
async fn main() -> Result<()> {
    let _session = SessionConfigBuilder::new()
        .contact_point("cassandra.testserver.com")
        .authenticator(Authenticator::plain_text("test_user", "password"))
        .keyspace("test_keyspace")
        .page_size(5000)
        .build()
        .connect()
        .await?;

    Ok(())
}
```

### Deserializing `SessionConfig` from a TOML file

To utilize this feature, update your application's `Cargo.toml` to include necessary dependencies:

```toml
[dependencies]
anyhow    = "1"
cassander = { version = "0.1", features = ["serde"] }
serde     = { version = "1", features = ["derive"] }
tokio     = { version = "1", features = ["full"] }
toml      = "0.8"
```

In your `main.rs`, deserialize the TOML configuration as follows:

```rust
use anyhow::Result;
use cassander::SessionConfig;
use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
struct Config {
    pub cassandra: SessionConfig
    // Additional configuration keys and sections
    // ...
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_toml = fs::read_to_string("app.toml").await?;
    let config = toml::from_str(&config_toml)?;

    let _session = config.connect().await?;

    Ok(())
}
```

Example configuration file (`app.toml`):

```toml
[cassandra]
  contact_points      = ["127.0.0.1"]
  consistency         = "Quorum"
  reconnect_wait_time = "2554ms"
  page_size           = 5000
  application_name    = "Cassander Test Client"
  application_version = "0.0.1"
  client_id           = "37b9fd59-bc06-4a88-aaf3-bd829fd7b755"

[cassandra.authenticator.plain_text]
  username            = "test_user"
  password            = "secret"
```
