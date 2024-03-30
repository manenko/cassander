/// The load balancing policy to use when selecting a node to send a request to.
///
/// This configures which nodes the driver talks to, and in which order they are
/// tried.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadBalancingPolicy {
    /// The driver discovers all nodes in a cluster and cycles through them per
    /// request. All are considered 'local'.
    RoundRobin,
    /// For each query, all live nodes in a primary 'local' datacenter are
    /// tried first, followed by any node from other datacenters.
    ///
    /// This is the default and does not need to be called unless switching
    /// from another policy or changing settings. Without further
    /// configuration, a default `local_dc` is chosen from the first
    /// connected contact point, and no remote hosts are considered in
    /// query plans. If relying on this mechanism, be sure to use only
    /// contact points from the local datacenter.
    DatacenterAware {
        /// The name of the primary datacenter to use for locality.
        ///
        /// If not set, the first connected contact point's datacenter is used.
        local_dc: Option<String>,
    },
}
