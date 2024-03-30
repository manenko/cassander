use std::num::NonZeroUsize;
use std::time::Duration;

/// A policy that decides if the driver will send speculative queries to the
/// next nodes when the current node takes too long to respond.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpeculativeExecutionPolicy {
    /// A policy that never triggers speculative executions.
    None,
    /// A policy that schedules a configurable number of speculative
    /// executions, separated by a fixed delay.
    Constant {
        /// The delay between each speculative execution.
        ///
        /// A zero delay means it should immediately send `max_executions`
        /// requests along with the original request.
        delay:          Duration,
        /// The maximum number of speculative executions.
        max_executions: NonZeroUsize,
    },
}
