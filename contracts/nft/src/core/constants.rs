// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const STATE_KEY: &str = "state_key";
pub const NFT_KEY: &str = "nft_key";
pub const NFT_COUNT_KEY: &str = "nft_count_key";
pub const NFT_OWNER_KEY: &str = "nft_owner_key";
pub const OPERATORS_KEY: &str = "operators_key";

// Actions
pub const ACTION_ATTRIBUTE: &str = "action";
pub const ACTION_TYPE_APPROVE: &str = "approve";
pub const ACTION_TYPE_APPROVE_ALL: &str = "approve_all";
pub const ACTION_TYPE_BURN: &str = "burn";
pub const ACTION_TYPE_INITIALIZE: &str = "initialize";
pub const ACTION_TYPE_MINT: &str = "mint";
pub const ACTION_TYPE_REVOKE: &str = "revoke";
pub const ACTION_TYPE_REVOKE_ALL: &str = "revoke_all";
pub const ACTION_TYPE_SEND: &str = "send";
pub const ACTION_TYPE_TRANSFER: &str = "transfer";
pub const ACTION_TYPE_MIGRATE: &str = "migrate";

// Events
