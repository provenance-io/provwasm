use cosmwasm_std::{Addr, Binary, Coin, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Supported provenance module router keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceRoute {
    Attribute,
    Marker,
    Name,
}

/// A collection of bound names.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Names {
    pub records: Vec<Name>,
}

/// A name bound to an address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Name {
    pub name: String,
    pub address: Addr,
    pub restricted: bool,
}

/// A type for name bindings
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NameBinding {
    Restricted,
    Unrestricted,
}

/// Bind names as restricted by default
impl Default for NameBinding {
    fn default() -> Self {
        NameBinding::Restricted
    }
}

/// A collection of attributes associated with an account address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attributes {
    pub address: Addr,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// Allowed attribute value types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeValueType {
    Uuid,
    Json,
    String,
    Bytes,
}

/// A typed key-value pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    pub name: String,
    pub value: Binary,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub value_type: AttributeValueType,
}

// A marker account
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Marker {
    pub address: Addr,
    #[serde(default)]
    pub coins: Vec<Coin>,
    pub account_number: u64,
    pub sequence: u64,
    #[serde(default)]
    manager: String, // Keep private and force use of public get_manager()
    #[serde(default)]
    pub permissions: Vec<AccessGrant>,
    pub status: MarkerStatus,
    pub denom: String,
    pub total_supply: Decimal,
    pub marker_type: MarkerType,
    pub supply_fixed: bool,
}

impl Marker {
    /// Determines whether a marker requires restricted transfers.
    pub fn bank_sends_disabled(&self) -> bool {
        matches!(self.marker_type, MarkerType::Restricted)
    }

    /// Returns the human address for the marker manager if defined.
    pub fn get_manager(&self) -> Option<Addr> {
        if !self.manager.is_empty() {
            let address = Addr::unchecked(self.manager.clone());
            Some(address)
        } else {
            None
        }
    }
}

// Marker permissions granted to another account.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessGrant {
    pub permissions: Vec<MarkerAccess>,
    pub address: Addr,
}

/// Marker permission types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerAccess {
    Admin,
    Burn,
    Deposit,
    Delete,
    Mint,
    Transfer,
    // Query only
    Unspecified,
    Withdraw,
}

impl MarkerAccess {
    /// A helper that returns all permissions that can be granted.
    pub fn all() -> Vec<MarkerAccess> {
        vec![
            MarkerAccess::Admin,
            MarkerAccess::Burn,
            MarkerAccess::Deposit,
            MarkerAccess::Delete,
            MarkerAccess::Mint,
            MarkerAccess::Transfer,
            MarkerAccess::Withdraw,
        ]
    }
}

/// Marker types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerType {
    Coin,
    // Means "restricted coin"
    Restricted,
    // Query only
    Unspecified,
}

/// Marker status types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerStatus {
    Active,
    Cancelled,
    Destroyed,
    Finalized,
    Proposed,
    // Query only
    Unspecified,
}
