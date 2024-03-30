pub mod opt_duration_as_string {
    use std::time::Duration;

    use duration_string::{
        self,
        DurationString,
    };
    use serde::{
        Deserialize,
        Deserializer,
        Serializer,
    };

    ///  Serializes an `Option<Duration>` as a string.
    pub fn serialize<S>(
        duration: &Option<Duration>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(duration) = duration {
            let value = DurationString::new(*duration);
            serializer.serialize_some(&value)
        } else {
            serializer.serialize_none()
        }
    }

    /// Deserializes an `Option<Duration>` from a string.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let duration = Option::<DurationString>::deserialize(deserializer)?
            .map(Into::into);

        Ok(duration)
    }
}

pub mod duration_as_string {
    use std::time::Duration;

    use duration_string::{
        self,
        DurationString,
    };
    use serde::{
        Deserialize,
        Deserializer,
        Serializer,
    };

    ///  Serializes a `Duration` as a string.
    pub fn serialize<S>(
        duration: &Duration,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = DurationString::new(*duration);

        serializer.serialize_some(&value)
    }

    /// Deserializes a `Duration` from a string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        DurationString::deserialize(deserializer).map(Into::into)
    }
}
