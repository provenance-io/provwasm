// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const STATE_KEY: &str = "state_key";

// Actions
pub const ACTION_ATTRIBUTE: &str = "action";
pub const ACTION_TYPE_INITIALIZE: &str = "initialize";
pub const ACTION_TYPE_MIGRATE: &str = "migrate";
pub const ACTION_TYPE_CHANGE_OWNER: &str = "change_owner";

// Events
pub const CHANGE_OWNER_EVENT: &str = ACTION_TYPE_CHANGE_OWNER;
pub const CHANGE_OWNER_PREVIOUS: &str = "previous_owner";
pub const CHANGE_OWNER_NEW: &str = "new_owner";

// Replies
pub const DEFAULT_REPLY: u64 = 0;
