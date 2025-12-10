// Conditionally compile IBC modules based on feature flags
#[cfg(any(feature = "ibc-all", feature = "ibc-applications"))]
pub mod applications;

#[cfg(any(feature = "ibc-all", feature = "ibc-core"))]
pub mod core;

#[cfg(any(feature = "ibc-all", feature = "ibc-lightclients"))]
pub mod lightclients;
