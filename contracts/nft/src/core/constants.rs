// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const STATE_KEY: &str = "state_key";
pub const NFT_KEY: &str = "nft_key";
pub const NFT_COUNT_KEY: &str = "nft_count_key";
pub const NFT_OWNER_KEY: &str = "nft_owner_key";

// Actions
pub const ACTION_ATTRIBUTE: &str = "action";
pub const ACTION_TYPE_INITIALIZE: &str = "initialize";
pub const ACTION_TYPE_EXECUTE: &str = "execute";
pub const ACTION_TYPE_MIGRATE: &str = "migrate";

// Events
