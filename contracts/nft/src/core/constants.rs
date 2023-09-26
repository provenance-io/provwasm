// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const CONTRACT_INFO_KEY: &str = "contract_info";
pub const STATE_KEY: &str = "state";
pub const NFT_KEY: &str = "nft";
pub const NFT_COUNT_KEY: &str = "nft_count";
pub const NFT_OWNER_KEY: &str = "nft_owner";
pub const OPERATORS_KEY: &str = "operators";

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

// Query
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;

// Events
pub const TOKEN_ID: &str = "token_id";
// Approve Event
pub const EVENT_APPROVE: &str = ACTION_TYPE_APPROVE;
pub const EVENT_APPROVE_SPENDER: &str = "spender";
pub const EVENT_APPROVE_TOKEN_ID: &str = TOKEN_ID;
// Approve All Event
pub const EVENT_APPROVE_ALL: &str = ACTION_TYPE_APPROVE_ALL;
pub const EVENT_APPROVE_ALL_OPERATOR: &str = "operator";
pub const EVENT_APPROVE_ALL_SENDER: &str = "sender";
