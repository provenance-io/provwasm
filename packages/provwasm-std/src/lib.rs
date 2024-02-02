/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

mod common;
mod serde;
pub mod shim;
pub mod types;
// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.
#[no_mangle]
extern "C" fn requires_provenance() {}
