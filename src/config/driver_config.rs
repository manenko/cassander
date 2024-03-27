use crate::ContactPoint;

/// The driver configuration.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClusterConfig {
    pub contact_points: Vec<ContactPoint>,
}
