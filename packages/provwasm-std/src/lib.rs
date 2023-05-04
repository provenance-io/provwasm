/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

mod serde;
pub mod shim;
pub mod types;
