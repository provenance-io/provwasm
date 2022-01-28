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
    Metadata,
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
    Uri,
    Int,
    Float,
    Proto,
    Unspecified,
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

/// A marker account
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Marker {
    pub address: Addr,
    #[serde(default)]
    pub coins: Vec<Coin>,
    pub account_number: u64,
    pub sequence: u64,
    #[serde(default)]
    pub manager: String,
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
}

/// Marker permissions granted to another account.
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
    Restricted,
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
    Unspecified,
}

/// A collection of records owned by one or more parties.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Scope {
    pub scope_id: String,
    pub specification_id: String,
    #[serde(default)]
    pub owners: Vec<Party>,
    #[serde(default)]
    pub data_access: Vec<Addr>,
    pub value_owner_address: Addr,
}

/// The final state of an execution context for a specification instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Session {
    pub name: String,
    pub session_id: String,
    pub specification_id: String,
    #[serde(default)]
    pub parties: Vec<Party>,
    pub context: Binary,
}

/// A group of sessions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Sessions {
    pub sessions: Vec<Session>,
}

/// A record of fact for a session.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    pub name: String,
    pub session_id: String,
    pub specification_id: String,
    pub process: Process,
    #[serde(default)]
    pub inputs: Vec<RecordInput>,
    #[serde(default)]
    pub outputs: Vec<RecordOutput>,
}

/// A group of records.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Records {
    pub records: Vec<Record>,
}

/// An address with an associated role.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Party {
    pub address: Addr,
    pub role: PartyType,
}

/// Roles that can be associated to a party.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PartyType {
    Originator,
    Servicer,
    Investor,
    Custodian,
    Owner,
    Affiliate,
    Omnibus,
    Provenance,
    Unspecified,
}

/// The process that generated a record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Process {
    pub process_id: ProcessId,
    pub method: String,
    pub name: String,
}

/// The representations of a process id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProcessId {
    /// The on-chain address of a process.
    Address { address: String },
    /// The hash of an off-chain process.
    Hash { hash: String },
}

/// The inputs used to produce a record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RecordInput {
    pub name: String,
    pub type_name: String,
    pub source: RecordInputSource,
    pub status: RecordInputStatus,
}

/// The representations of a record input source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordInputSource {
    /// The address of a record on chain (established records).
    Record { record_id: String },
    /// The hash of an off-chain piece of information (proposed records).
    Hash { hash: String },
}

/// Record input types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RecordInputStatus {
    Proposed,
    Record,
    Unspecified,
}

/// The output of a process recorded on chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RecordOutput {
    pub hash: String,
    pub status: ResultStatus,
}

/// Result status types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResultStatus {
    Pass,
    Skip,
    Fail,
    Unspecified,
}
