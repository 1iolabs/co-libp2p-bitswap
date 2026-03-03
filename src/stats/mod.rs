#[cfg(feature = "metrics")]
mod metrics;
#[cfg(feature = "metrics")]
pub use metrics::*;

#[cfg(not(feature = "metrics"))]
#[allow(dead_code)]
mod noop;
#[cfg(not(feature = "metrics"))]
pub use noop::*;

/// The timer type used for query duration tracking.
#[cfg(feature = "metrics")]
pub type Timer = prometheus::HistogramTimer;
#[cfg(not(feature = "metrics"))]
pub type Timer = noop::NoopTimer;
