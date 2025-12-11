/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

pub mod metadata_address;
pub mod shim;
#[allow(
    deprecated,
    unused_imports,
    clippy::large_enum_variant,
    clippy::too_many_arguments,
    clippy::doc_overindented_list_items,
    clippy::doc_lazy_continuation
)]
pub mod types;

// Coin conversion functions are only available when cosmos-base feature is enabled
#[cfg(feature = "cosmos-base")]
pub use shim::{cosmwasm_to_proto_coins, try_proto_to_cosmwasm_coins};

// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.
#[no_mangle]
extern "C" fn requires_provenance() {}
