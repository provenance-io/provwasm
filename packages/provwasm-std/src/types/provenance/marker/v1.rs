use provwasm_std_derive::CosmwasmExt;
/// AccessGrant associates a colelction of permisssions with an address for delegated marker account control.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.AccessGrant")]
pub struct AccessGrant {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Access", repeated, packed = "false", tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
/// Access defines the different types of permissions that a marker supports granting to an address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Access {
    /// ACCESS_UNSPECIFIED defines a no-op vote option.
    Unspecified = 0,
    /// ACCESS_MINT is the ability to increase the supply of a marker
    Mint = 1,
    /// ACCESS_BURN is the ability to decrease the supply of the marker using coin held by the marker.
    Burn = 2,
    /// ACCESS_DEPOSIT is the ability to set a marker reference to this marker in the metadata/scopes module
    Deposit = 3,
    /// ACCESS_WITHDRAW is the ability to remove marker references to this marker in from metadata/scopes or
    /// transfer coin from this marker account to another account.
    Withdraw = 4,
    /// ACCESS_DELETE is the ability to move a proposed, finalized or active marker into the cancelled state. This
    /// access also allows cancelled markers to be marked for deletion
    Delete = 5,
    /// ACCESS_ADMIN is the ability to add access grants for accounts to the list of marker permissions.
    Admin = 6,
    /// ACCESS_TRANSFER is the ability to invoke a send operation using the marker module to facilitate exchange.
    /// This access right is only supported on RESTRICTED markers.
    Transfer = 7,
}
impl Access {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Access::Unspecified => "ACCESS_UNSPECIFIED",
            Access::Mint => "ACCESS_MINT",
            Access::Burn => "ACCESS_BURN",
            Access::Deposit => "ACCESS_DEPOSIT",
            Access::Withdraw => "ACCESS_WITHDRAW",
            Access::Delete => "ACCESS_DELETE",
            Access::Admin => "ACCESS_ADMIN",
            Access::Transfer => "ACCESS_TRANSFER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCESS_MINT" => Some(Self::Mint),
            "ACCESS_BURN" => Some(Self::Burn),
            "ACCESS_DEPOSIT" => Some(Self::Deposit),
            "ACCESS_WITHDRAW" => Some(Self::Withdraw),
            "ACCESS_DELETE" => Some(Self::Delete),
            "ACCESS_ADMIN" => Some(Self::Admin),
            "ACCESS_TRANSFER" => Some(Self::Transfer),
            _ => None,
        }
    }
}
/// Params defines the set of params for the account module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.Params")]
pub struct Params {
    /// maximum amount of supply to allow a marker to be created with
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_total_supply: u64,
    /// indicates if governance based controls of markers is allowed.
    #[prost(bool, tag = "2")]
    pub enable_governance: bool,
    /// a regular expression used to validate marker denom values from normal create requests (governance
    /// requests are only subject to platform coin validation denom expression)
    #[prost(string, tag = "3")]
    pub unrestricted_denom_regex: ::prost::alloc::string::String,
}
/// MarkerAccount holds the marker configuration information in addition to a base account structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerAccount")]
pub struct MarkerAccount {
    /// base cosmos account information including address and coin holdings.
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// Address that owns the marker configuration.  This account must sign any requests
    /// to change marker config (only valid for statuses prior to finalization)
    #[prost(string, tag = "2")]
    pub manager: ::prost::alloc::string::String,
    /// Access control lists
    #[prost(message, repeated, tag = "3")]
    pub access_control: ::prost::alloc::vec::Vec<AccessGrant>,
    /// Indicates the current status of this marker record.
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// value denomination and total supply for the token.
    #[prost(string, tag = "5")]
    pub denom: ::prost::alloc::string::String,
    /// the total supply expected for a marker.  This is the amount that is minted when a marker is created.
    #[prost(string, tag = "6")]
    pub supply: ::prost::alloc::string::String,
    /// Marker type information
    #[prost(enumeration = "MarkerType", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub marker_type: i32,
    /// A fixed supply will mint additional coin automatically if the total supply decreases below a set value.  This
    /// may occur if the coin is burned or an account holding the coin is slashed. (default: true)
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    /// indicates that governance based control is allowed for this marker
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// EventMarkerAdd event emitted when marker is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAdd")]
pub struct EventMarkerAdd {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub marker_type: ::prost::alloc::string::String,
}
/// EventMarkerAddAccess event emitted when marker access is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAddAccess")]
pub struct EventMarkerAddAccess {
    #[prost(message, optional, tag = "1")]
    pub access: ::core::option::Option<EventMarkerAccess>,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerAccess event access permissions for address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAccess")]
pub struct EventMarkerAccess {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventMarkerDeleteAccess event emitted when marker access is revoked
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerDeleteAccess")]
pub struct EventMarkerDeleteAccess {
    #[prost(string, tag = "1")]
    pub remove_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerFinalize event emitted when marker is finalized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerFinalize")]
pub struct EventMarkerFinalize {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerActivate event emitted when marker is activated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerActivate")]
pub struct EventMarkerActivate {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerCancel event emitted when marker is cancelled
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerCancel")]
pub struct EventMarkerCancel {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerDelete event emitted when marker is deleted
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerDelete")]
pub struct EventMarkerDelete {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerMint event emitted when additional marker supply is minted
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerMint")]
pub struct EventMarkerMint {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerBurn event emitted when coin is burned from marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerBurn")]
pub struct EventMarkerBurn {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerWithdraw event emitted when coins are withdrew from marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerWithdraw")]
pub struct EventMarkerWithdraw {
    #[prost(string, tag = "1")]
    pub coins: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_address: ::prost::alloc::string::String,
}
/// EventMarkerTransfer event emitted when coins are transfered to from account to another
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerTransfer")]
pub struct EventMarkerTransfer {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub from_address: ::prost::alloc::string::String,
}
/// EventMarkerSetDenomMetadata event emitted when metadata is set on marker with denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerSetDenomMetadata")]
pub struct EventMarkerSetDenomMetadata {
    #[prost(string, tag = "1")]
    pub metadata_base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub metadata_description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub metadata_display: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub metadata_denom_units: ::prost::alloc::vec::Vec<EventDenomUnit>,
    #[prost(string, tag = "5")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub metadata_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub metadata_symbol: ::prost::alloc::string::String,
}
/// EventDenomUnit denom units for set denom metadata event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventDenomUnit")]
pub struct EventDenomUnit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exponent: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MarkerType defines the types of marker
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarkerType {
    /// MARKER_TYPE_UNSPECIFIED is an invalid/unknown marker type.
    Unspecified = 0,
    /// MARKER_TYPE_COIN is a marker that represents a standard fungible coin (default).
    Coin = 1,
    /// MARKER_TYPE_RESTRICTED is a marker that represents a denom with send_enabled = false.
    Restricted = 2,
}
impl MarkerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarkerType::Unspecified => "MARKER_TYPE_UNSPECIFIED",
            MarkerType::Coin => "MARKER_TYPE_COIN",
            MarkerType::Restricted => "MARKER_TYPE_RESTRICTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKER_TYPE_COIN" => Some(Self::Coin),
            "MARKER_TYPE_RESTRICTED" => Some(Self::Restricted),
            _ => None,
        }
    }
}
/// MarkerStatus defines the various states a marker account can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarkerStatus {
    /// MARKER_STATUS_UNSPECIFIED - Unknown/Invalid Marker Status
    Unspecified = 0,
    /// MARKER_STATUS_PROPOSED - Initial configuration period, updates allowed, token supply not created.
    Proposed = 1,
    /// MARKER_STATUS_FINALIZED - Configuration finalized, ready for supply creation
    Finalized = 2,
    /// MARKER_STATUS_ACTIVE - Supply is created, rules are in force.
    Active = 3,
    /// MARKER_STATUS_CANCELLED - Marker has been cancelled, pending destroy
    Cancelled = 4,
    /// MARKER_STATUS_DESTROYED - Marker supply has all been recalled, marker is considered destroyed and no further
    /// actions allowed.
    Destroyed = 5,
}
impl MarkerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarkerStatus::Unspecified => "MARKER_STATUS_UNSPECIFIED",
            MarkerStatus::Proposed => "MARKER_STATUS_PROPOSED",
            MarkerStatus::Finalized => "MARKER_STATUS_FINALIZED",
            MarkerStatus::Active => "MARKER_STATUS_ACTIVE",
            MarkerStatus::Cancelled => "MARKER_STATUS_CANCELLED",
            MarkerStatus::Destroyed => "MARKER_STATUS_DESTROYED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKER_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKER_STATUS_PROPOSED" => Some(Self::Proposed),
            "MARKER_STATUS_FINALIZED" => Some(Self::Finalized),
            "MARKER_STATUS_ACTIVE" => Some(Self::Active),
            "MARKER_STATUS_CANCELLED" => Some(Self::Cancelled),
            "MARKER_STATUS_DESTROYED" => Some(Self::Destroyed),
            _ => None,
        }
    }
}
/// GenesisState defines the account module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// A collection of marker accounts to create on start
    #[prost(message, repeated, tag = "2")]
    pub markers: ::prost::alloc::vec::Vec<MarkerAccount>,
}
/// AddMarkerProposal defines defines a governance proposal to create a new marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.AddMarkerProposal")]
pub struct AddMarkerProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub manager: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "7")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// SupplyIncreaseProposal defines a governance proposal to administer a marker and increase total supply of the marker
/// through minting coin and placing it within the marker or assigning it directly to an account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyIncreaseProposal")]
pub struct SupplyIncreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// an optional target address for the minted coin from this request
    #[prost(string, tag = "4")]
    pub target_address: ::prost::alloc::string::String,
}
/// SupplyDecreaseProposal defines a governance proposal to administer a marker and decrease the total supply through
/// burning coin held within the marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyDecreaseProposal")]
pub struct SupplyDecreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// SetAdministratorProposal defines a governance proposal to administer a marker and set administrators with specific
/// access on the marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetAdministratorProposal")]
pub struct SetAdministratorProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub access: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// RemoveAdministratorProposal defines a governance proposal to administer a marker and remove all permissions for a
/// given address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.RemoveAdministratorProposal")]
pub struct RemoveAdministratorProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub removed_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ChangeStatusProposal defines a governance proposal to administer a marker to change its status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.ChangeStatusProposal")]
pub struct ChangeStatusProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_status: i32,
}
/// WithdrawEscrowProposal defines a governance proposal to withdraw escrow coins from a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.WithdrawEscrowProposal")]
pub struct WithdrawEscrowProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub target_address: ::prost::alloc::string::String,
}
/// SetDenomMetadataProposal defines a governance proposal to set the metadata for a denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetDenomMetadataProposal")]
pub struct SetDenomMetadataProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllMarkersRequest is the request type for the Query/AllMarkers method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAllMarkersRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/AllMarkers",
    response_type = QueryAllMarkersResponse
)]
pub struct QueryAllMarkersRequest {
    /// Optional status to filter request
    #[prost(enumeration = "MarkerStatus", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllMarkersResponse is the response type for the Query/AllMarkers method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAllMarkersResponse")]
pub struct QueryAllMarkersResponse {
    #[prost(message, repeated, tag = "1")]
    pub markers: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryMarkerRequest is the request type for the Query/Marker method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryMarkerRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Marker",
    response_type = QueryMarkerResponse
)]
pub struct QueryMarkerRequest {
    /// the address or denom of the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryMarkerResponse is the response type for the Query/Marker method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryMarkerResponse")]
pub struct QueryMarkerResponse {
    #[prost(message, optional, tag = "1")]
    pub marker: ::core::option::Option<crate::shim::Any>,
}
/// QueryHoldingRequest is the request type for the Query/MarkerHolders method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryHoldingRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Holding",
    response_type = QueryHoldingResponse
)]
pub struct QueryHoldingRequest {
    /// the address or denom of the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryHoldingResponse is the response type for the Query/MarkerHolders method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryHoldingResponse")]
pub struct QueryHoldingResponse {
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySupplyRequest is the request type for the Query/MarkerSupply method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QuerySupplyRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Supply",
    response_type = QuerySupplyResponse
)]
pub struct QuerySupplyRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/MarkerSupply method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QuerySupplyResponse")]
pub struct QuerySupplyResponse {
    /// amount is the supply of the marker.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEscrowRequest is the request type for the Query/MarkerEscrow method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryEscrowRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Escrow",
    response_type = QueryEscrowResponse
)]
pub struct QueryEscrowRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryEscrowResponse is the response type for the Query/MarkerEscrow method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryEscrowResponse")]
pub struct QueryEscrowResponse {
    #[prost(message, repeated, tag = "1")]
    pub escrow: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryAccessRequest is the request type for the Query/MarkerAccess method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccessRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Access",
    response_type = QueryAccessResponse
)]
pub struct QueryAccessRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryAccessResponse is the response type for the Query/MarkerAccess method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccessResponse")]
pub struct QueryAccessResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// QueryDenomMetadataRequest is the request type for Query/DenomMetadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryDenomMetadataRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/DenomMetadata",
    response_type = QueryDenomMetadataResponse
)]
pub struct QueryDenomMetadataRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomMetadataResponse is the response type for the Query/DenomMetadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryDenomMetadataResponse")]
pub struct QueryDenomMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// Balance defines an account address and balance pair used in queries for accounts holding a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.Balance")]
pub struct Balance {
    /// address is the address of the balance holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// coins defines the different coins this balance holds.
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// SIPrefix represents an International System of Units (SI) Prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SiPrefix {
    /// 10^0    (none)
    None = 0,
    /// 10^1    deka   da
    Deka = 1,
    /// 10^2    hecto   h
    Hecto = 2,
    /// 10^3    kilo    k
    Kilo = 3,
    /// 10^6    mega    M
    Mega = 6,
    /// 10^9    giga    G
    Giga = 9,
    /// 10^12   tera    T
    Tera = 12,
    /// 10^15   peta    P
    Peta = 15,
    /// 10^18   exa     E
    Exa = 18,
    /// 10^21   zetta   Z
    Zetta = 21,
    /// 10^24   yotta   Y
    Yotta = 24,
    /// 10^-1   deci    d
    Deci = -1,
    /// 10^-2   centi   c
    Centi = -2,
    /// 10^-3   milli   m
    Milli = -3,
    /// 10^-6   micro   µ
    Micro = -6,
    /// 10^-9   nano    n
    Nano = -9,
    /// 10^-12  pico    p
    Pico = -12,
    /// 10^-15  femto   f
    Femto = -15,
    /// 10^-18  atto    a
    Atto = -18,
    /// 10^-21  zepto   z
    Zepto = -21,
    /// 10^-24  yocto   y
    Yocto = -24,
}
impl SiPrefix {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SiPrefix::None => "SI_PREFIX_NONE",
            SiPrefix::Deka => "SI_PREFIX_DEKA",
            SiPrefix::Hecto => "SI_PREFIX_HECTO",
            SiPrefix::Kilo => "SI_PREFIX_KILO",
            SiPrefix::Mega => "SI_PREFIX_MEGA",
            SiPrefix::Giga => "SI_PREFIX_GIGA",
            SiPrefix::Tera => "SI_PREFIX_TERA",
            SiPrefix::Peta => "SI_PREFIX_PETA",
            SiPrefix::Exa => "SI_PREFIX_EXA",
            SiPrefix::Zetta => "SI_PREFIX_ZETTA",
            SiPrefix::Yotta => "SI_PREFIX_YOTTA",
            SiPrefix::Deci => "SI_PREFIX_DECI",
            SiPrefix::Centi => "SI_PREFIX_CENTI",
            SiPrefix::Milli => "SI_PREFIX_MILLI",
            SiPrefix::Micro => "SI_PREFIX_MICRO",
            SiPrefix::Nano => "SI_PREFIX_NANO",
            SiPrefix::Pico => "SI_PREFIX_PICO",
            SiPrefix::Femto => "SI_PREFIX_FEMTO",
            SiPrefix::Atto => "SI_PREFIX_ATTO",
            SiPrefix::Zepto => "SI_PREFIX_ZEPTO",
            SiPrefix::Yocto => "SI_PREFIX_YOCTO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SI_PREFIX_NONE" => Some(Self::None),
            "SI_PREFIX_DEKA" => Some(Self::Deka),
            "SI_PREFIX_HECTO" => Some(Self::Hecto),
            "SI_PREFIX_KILO" => Some(Self::Kilo),
            "SI_PREFIX_MEGA" => Some(Self::Mega),
            "SI_PREFIX_GIGA" => Some(Self::Giga),
            "SI_PREFIX_TERA" => Some(Self::Tera),
            "SI_PREFIX_PETA" => Some(Self::Peta),
            "SI_PREFIX_EXA" => Some(Self::Exa),
            "SI_PREFIX_ZETTA" => Some(Self::Zetta),
            "SI_PREFIX_YOTTA" => Some(Self::Yotta),
            "SI_PREFIX_DECI" => Some(Self::Deci),
            "SI_PREFIX_CENTI" => Some(Self::Centi),
            "SI_PREFIX_MILLI" => Some(Self::Milli),
            "SI_PREFIX_MICRO" => Some(Self::Micro),
            "SI_PREFIX_NANO" => Some(Self::Nano),
            "SI_PREFIX_PICO" => Some(Self::Pico),
            "SI_PREFIX_FEMTO" => Some(Self::Femto),
            "SI_PREFIX_ATTO" => Some(Self::Atto),
            "SI_PREFIX_ZEPTO" => Some(Self::Zepto),
            "SI_PREFIX_YOCTO" => Some(Self::Yocto),
            _ => None,
        }
    }
}
/// MarkerTransferAuthorization gives the grantee permissions to execute
/// a marker transfer on behalf of the granter's account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerTransferAuthorization")]
pub struct MarkerTransferAuthorization {
    /// transfer_limit is the total amount the grantee can transfer
    #[prost(message, repeated, tag = "1")]
    pub transfer_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgGrantAllowanceRequest validates permission to create a fee grant based on marker admin access. If
/// successful a feegrant is recorded where the marker account itself is the grantor
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgGrantAllowanceRequest")]
pub struct MsgGrantAllowanceRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic and filtered fee allowance (fee FeeGrant module).
    #[prost(message, optional, tag = "4")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
}
/// MsgGrantAllowanceResponse defines the Msg/GrantAllowanceResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgGrantAllowanceResponse")]
pub struct MsgGrantAllowanceResponse {}
/// MsgAddMarkerRequest defines the Msg/AddMarker request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddMarkerRequest")]
pub struct MsgAddMarkerRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "7")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// MsgAddMarkerResponse defines the Msg/AddMarker response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddMarkerResponse")]
pub struct MsgAddMarkerResponse {}
/// MsgAddAccessRequest defines the Msg/AddAccess request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddAccessRequest")]
pub struct MsgAddAccessRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub access: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// MsgAddAccessResponse defines the Msg/AddAccess response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddAccessResponse")]
pub struct MsgAddAccessResponse {}
/// MsgDeleteAccessRequest defines the Msg/DeleteAccess request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteAccessRequest")]
pub struct MsgDeleteAccessRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub removed_address: ::prost::alloc::string::String,
}
/// MsgDeleteAccessResponse defines the Msg/DeleteAccess response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteAccessResponse")]
pub struct MsgDeleteAccessResponse {}
/// MsgFinalizeRequest defines the Msg/Finalize request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgFinalizeRequest")]
pub struct MsgFinalizeRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgFinalizeResponse defines the Msg/Finalize response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgFinalizeResponse")]
pub struct MsgFinalizeResponse {}
/// MsgActivateRequest defines the Msg/Activate request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgActivateRequest")]
pub struct MsgActivateRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgActivateResponse defines the Msg/Activate response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgActivateResponse")]
pub struct MsgActivateResponse {}
/// MsgCancelRequest defines the Msg/Cancel request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgCancelRequest")]
pub struct MsgCancelRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgCancelResponse defines the Msg/Cancel response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgCancelResponse")]
pub struct MsgCancelResponse {}
/// MsgDeleteRequest defines the Msg/Delete request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteRequest")]
pub struct MsgDeleteRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgDeleteResponse defines the Msg/Delete response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteResponse")]
pub struct MsgDeleteResponse {}
/// MsgMintRequest defines the Msg/Mint request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgMintRequest")]
pub struct MsgMintRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/Mint response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgMintResponse")]
pub struct MsgMintResponse {}
/// MsgBurnRequest defines the Msg/Burn request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgBurnRequest")]
pub struct MsgBurnRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgBurnResponse defines the Msg/Burn response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgBurnResponse")]
pub struct MsgBurnResponse {}
/// MsgWithdrawRequest defines the Msg/Withdraw request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawRequest")]
pub struct MsgWithdrawRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawResponse defines the Msg/Withdraw response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawResponse")]
pub struct MsgWithdrawResponse {}
/// MsgTransferRequest defines the Msg/Transfer request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgTransferRequest")]
pub struct MsgTransferRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub to_address: ::prost::alloc::string::String,
}
/// MsgTransferResponse defines the Msg/Transfer response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgTransferResponse")]
pub struct MsgTransferResponse {}
/// MsgIbcTransferRequest defines the Msg/IbcTransfer request type for markers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgIbcTransferRequest")]
pub struct MsgIbcTransferRequest {
    #[prost(message, optional, tag = "1")]
    pub transfer:
        ::core::option::Option<super::super::super::ibc::applications::transfer::v1::MsgTransfer>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgIbcTransferResponse defines the Msg/IbcTransfer response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgIbcTransferResponse")]
pub struct MsgIbcTransferResponse {}
/// MsgSetDenomMetadataRequest defines the Msg/SetDenomMetadata request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataRequest")]
pub struct MsgSetDenomMetadataRequest {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgSetDenomMetadataResponse defines the Msg/SetDenomMetadata response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataResponse")]
pub struct MsgSetDenomMetadataResponse {}
pub struct MarkerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MarkerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn all_markers(
        &self,
        status: i32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllMarkersResponse, cosmwasm_std::StdError> {
        QueryAllMarkersRequest { status, pagination }.query(self.querier)
    }
    pub fn marker(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryMarkerResponse, cosmwasm_std::StdError> {
        QueryMarkerRequest { id }.query(self.querier)
    }
    pub fn holding(
        &self,
        id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryHoldingResponse, cosmwasm_std::StdError> {
        QueryHoldingRequest { id, pagination }.query(self.querier)
    }
    pub fn supply(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QuerySupplyResponse, cosmwasm_std::StdError> {
        QuerySupplyRequest { id }.query(self.querier)
    }
    pub fn escrow(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryEscrowResponse, cosmwasm_std::StdError> {
        QueryEscrowRequest { id }.query(self.querier)
    }
    pub fn access(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryAccessResponse, cosmwasm_std::StdError> {
        QueryAccessRequest { id }.query(self.querier)
    }
    pub fn denom_metadata(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryDenomMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomMetadataRequest { denom }.query(self.querier)
    }
}
