// Crate
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Storage
pub const STATE_KEY: &str = "state_key";
pub const ASSET_SECURITY_KEY: &str = "asset_security_key";
pub const SECURITY_TO_ASSET_KEY: &str = "security_to_asset_key";
pub const SECURITY_TYPE_KEY: &str = "security_type_key";

// Actions
pub const ACTION_ATTRIBUTE: &str = "action";
pub const ACTION_TYPE_INITIALIZE: &str = "initialize";
pub const ACTION_TYPE_MIGRATE: &str = "migrate";
pub const ACTION_TYPE_CHANGE_OWNER: &str = "change_owner";
pub const ACTION_TYPE_SET_SECURITY: &str = "set_security";
pub const ACTION_TYPE_SET_SECURITY_MULTIPLE: &str = "set_security_multiple";
pub const ACTION_TYPE_REMOVE_SECURITY: &str = "remove_security";
pub const ACTION_TYPE_ADD_SECURITIES: &str = "add_security_types";
pub const ACTION_TYPE_REMOVE_SECURITIES: &str = "remove_security_types";

// Events
pub const CHANGE_OWNER_EVENT: &str = ACTION_TYPE_CHANGE_OWNER;
pub const SET_SECURITY_EVENT: &str = ACTION_TYPE_SET_SECURITY;
pub const REMOVE_SECURITY_EVENT: &str = ACTION_TYPE_REMOVE_SECURITY;
pub const SET_SECURITY_MULTIPLE_EVENT: &str = ACTION_TYPE_SET_SECURITY_MULTIPLE;
pub const UPDATE_SECURITY_TYPES_EVENT: &str = "update_security_types";
pub const CHANGE_OWNER_PREVIOUS: &str = "previous_owner";
pub const CHANGE_OWNER_NEW: &str = "new_owner";
pub const SET_SECURITY_ASSET: &str = "asset_address";
pub const SET_SECURITY_VALUE: &str = "security";
pub const SET_SECURITY_MULTIPLE_NUM: &str = "num_assets";
pub const SET_SECURITY_MULTIPLE_VALUE: &str = "security";
pub const REMOVE_SECURITY_ASSET: &str = "asset_address";
pub const REMOVE_SECURITY_SECURITY: &str = "security";

// Limits
pub const DEFAULT_WITH_SECURITY_LIMIT: u64 = 10;
pub const MAX_WITH_SECURITY_LIMIT: u64 = 100;
pub const DEFAULT_SECURITY_TYPES_LIMIT: u64 = 10;
pub const MAX_SECURITY_TYPES_LIMIT: u64 = 100;
