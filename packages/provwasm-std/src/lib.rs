/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

mod serde;
pub mod shim;
#[allow(clippy::too_many_arguments, dead_code, deprecated, unused_imports)]
pub mod types;
// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.

pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};

#[no_mangle]
extern "C" fn requires_provenance() {}
