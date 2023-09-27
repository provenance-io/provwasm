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
pub const OPERATOR: &str = "operator";
pub const RECIPIENT: &str = "recipient";
// sender is not the same as spender. should be refactored to something like `owner`
pub const SENDER: &str = "sender";
pub const SPENDER: &str = "spender";
pub const TOKEN_ID: &str = "token_id";

// Approve Event
pub const EVENT_APPROVE: &str = ACTION_TYPE_APPROVE;
pub const EVENT_APPROVE_SPENDER: &str = SPENDER;
pub const EVENT_APPROVE_TOKEN_ID: &str = TOKEN_ID;

// Approve All Event
pub const EVENT_APPROVE_ALL: &str = ACTION_TYPE_APPROVE_ALL;
pub const EVENT_APPROVE_ALL_OPERATOR: &str = OPERATOR;
pub const EVENT_APPROVE_ALL_SENDER: &str = SENDER;

// Burn Event
pub const EVENT_BURN: &str = ACTION_TYPE_BURN;
pub const EVENT_BURN_TOKEN_ID: &str = TOKEN_ID;

// Mint Event
pub const EVENT_MINT: &str = ACTION_TYPE_MINT;
pub const EVENT_MINT_RECIPIENT: &str = RECIPIENT;
pub const EVENT_MINT_TOKEN_ID: &str = TOKEN_ID;

// Revoke Event
pub const EVENT_REVOKE: &str = ACTION_TYPE_REVOKE;
pub const EVENT_REVOKE_SPENDER: &str = SPENDER;
pub const EVENT_REVOKE_TOKEN_ID: &str = TOKEN_ID;

// Revoke All Event
pub const EVENT_REVOKE_ALL: &str = ACTION_TYPE_REVOKE_ALL;
pub const EVENT_REVOKE_ALL_OPERATOR: &str = OPERATOR;
pub const EVENT_REVOKE_ALL_SENDER: &str = SENDER;

// Send Event
pub const EVENT_SEND: &str = ACTION_TYPE_SEND;
pub const EVENT_SEND_CONTRACT: &str = "contract";
pub const EVENT_SEND_TOKEN_ID: &str = TOKEN_ID;

// Transfer Event
pub const EVENT_TRANSFER: &str = ACTION_TYPE_TRANSFER;
pub const EVENT_TRANSFER_RECIPIENT: &str = RECIPIENT;
pub const EVENT_TRANSFER_TOKEN_ID: &str = TOKEN_ID;
