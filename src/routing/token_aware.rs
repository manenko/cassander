/// Token-aware routing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenAwareRoutingPolicy {
    /// Token-aware routing is disabled and has no effect.
    None,

    /// Under token-aware routing all requests will be routed in priority to
    /// the replicas that own the data being queried.
    Standard {
        /// Whether to randomly shuffle the replicas before routing the
        /// request.
        ///
        /// This can help to distribute the load more evenly across the
        /// replicas. The downside is that it can reduce the effectiveness of
        /// server-side caching.
        ///
        /// This is enabled by default.
        shuffle_replicas: Option<bool>,
    },
}
