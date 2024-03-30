use std::time::Duration;

/// The latency-aware routing policy.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LatencyAwareRoutingPolicy {
    /// Latency-aware routing is disabled and has no effect.
    None,

    /// Under latency-aware routing all requests will be routed in priority to
    /// the replicas that have the lowest latency.
    ///
    /// It uses the base routing policy to determine locality
    /// (datacenter-aware) and/or placement (token-aware) before
    /// considering the latency.
    Standard {
        /// Controls how much worse the latency must be compared to the average
        /// latency of the best performing node before it penalized.
        ///
        /// If not set, the default is 2.0.
        exclusion_threshold: Option<f64>,
        /// Controls the weight given to older latencies when calculating the
        /// average latency of a node. A bigger scale will give more weight to
        /// older latency measurements.
        ///
        /// If not set, the default is 100 milliseconds.
        #[cfg_attr(
            feature = "serde",
            serde(with = "crate::serialization::opt_duration_as_string")
        )]
        scale:               Option<Duration>,
        /// The amount of time a node is penalized by the policy before being
        /// given a second chance when the current average latency exceeds the
        /// calculated threshold, which is:
        ///
        /// ```text
        /// exclusion_threshold * best_average_latency
        /// ```
        ///
        /// If not set, the default is 10 seconds.
        #[cfg_attr(
            feature = "serde",
            serde(with = "crate::serialization::opt_duration_as_string")
        )]
        retry_period:        Option<Duration>,
        /// The rate at which the best average latency is recomputed.
        ///
        /// If not set, the default is 100 milliseconds.
        #[cfg_attr(
            feature = "serde",
            serde(with = "crate::serialization::opt_duration_as_string")
        )]
        update_rate:         Option<Duration>,
        /// The minimum number of measurements per-host required to be
        /// considered by the policy.
        ///
        /// If not set, the default is 50.
        min_measures:        Option<usize>,
    },
}
