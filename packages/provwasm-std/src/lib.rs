/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

mod serde;
pub mod shim;
#[allow(
    deprecated,
    unused_imports,
    clippy::large_enum_variant,
    clippy::too_many_arguments
)]
pub mod types;

pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};

// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.
#[no_mangle]
extern "C" fn requires_provenance() {}
