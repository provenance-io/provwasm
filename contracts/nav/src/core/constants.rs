// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const STATE_KEY: &str = "state_key";
pub const ASSET_TAG_KEY: &str = "asset_tag_key";
pub const ASSET_TYPE_KEY: &str = "asset_type_key";

// Actions
pub const ACTION_ATTRIBUTE: &str = "action";
pub const ACTION_TYPE_INITIALIZE: &str = "initialize";
pub const ACTION_TYPE_MIGRATE: &str = "migrate";
pub const ACTION_TYPE_CHANGE_OWNER: &str = "change_owner";
pub const ACTION_TYPE_SET_TAG: &str = "set_tag";

// Events
pub const CHANGE_OWNER_EVENT: &str = ACTION_TYPE_CHANGE_OWNER;
pub const SET_TAG_EVENT: &str = ACTION_TYPE_SET_TAG;
pub const CHANGE_OWNER_PREVIOUS: &str = "previous_owner";
pub const CHANGE_OWNER_NEW: &str = "new_owner";
pub const SET_TAG_ASSET: &str = "asset_address";
pub const SET_TAG_VALUE: &str = "tag";

// Replies
pub const DEFAULT_REPLY: u64 = 0;
