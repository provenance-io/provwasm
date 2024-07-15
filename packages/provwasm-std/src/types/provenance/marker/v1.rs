use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// AccessGrant associates a collection of permissions with an address for delegated marker account control.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum Access {
    /// ACCESS_UNSPECIFIED defines a no-op vote option.
    Unspecified = 0,
    /// ACCESS_MINT is the ability to increase the supply of a marker.
    Mint = 1,
    /// ACCESS_BURN is the ability to decrease the supply of the marker using coin held by the marker.
    Burn = 2,
    /// ACCESS_DEPOSIT is the ability to transfer funds from another account to this marker account
    /// or to set a reference to this marker in the metadata/scopes module.
    Deposit = 3,
    /// ACCESS_WITHDRAW is the ability to transfer funds from this marker account to another account
    /// or to remove a reference to this marker in the metadata/scopes module.
    Withdraw = 4,
    /// ACCESS_DELETE is the ability to move a proposed, finalized or active marker into the cancelled state.
    /// This access also allows cancelled markers to be marked for deletion.
    Delete = 5,
    /// ACCESS_ADMIN is the ability to add access grants for accounts to the list of marker permissions.
    /// This access also gives the ability to update the marker's denom metadata.
    Admin = 6,
    /// ACCESS_TRANSFER is the ability to manage transfer settings and broker transfers of the marker.
    /// Accounts with this access can:
    ///   - Update the marker's required attributes.
    ///   - Update the send-deny list.
    ///   - Use the transfer or bank send endpoints to move marker funds out of their own account.
    /// This access right is only supported on RESTRICTED markers.
    Transfer = 7,
    /// ACCESS_FORCE_TRANSFER is the ability to transfer restricted coins from a 3rd-party account without their signature.
    /// This access right is only supported on RESTRICTED markers and only has meaning when allow_forced_transfer is true.
    ForceTransfer = 8,
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
            Access::ForceTransfer => "ACCESS_FORCE_TRANSFER",
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
            "ACCESS_FORCE_TRANSFER" => Some(Self::ForceTransfer),
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerTransferAuthorization")]
pub struct MarkerTransferAuthorization {
    /// transfer_limit is the total amount the grantee can transfer
    #[prost(message, repeated, tag = "1")]
    pub transfer_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// allow_list specifies an optional list of addresses to whom the grantee can send restricted coins on behalf of the
    /// granter. If omitted, any recipient is allowed.
    #[prost(string, repeated, tag = "2")]
    pub allow_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the set of params for the account module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.Params")]
pub struct Params {
    /// Deprecated: Prefer to use `max_supply` instead. Maximum amount of supply to allow a marker to be created with
    #[deprecated]
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
    /// maximum amount of supply to allow a marker to be created with
    #[prost(string, tag = "4")]
    pub max_supply: ::prost::alloc::string::String,
}
/// MarkerAccount holds the marker configuration information in addition to a base account structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
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
        serialize_with = "MarkerType::serialize",
        deserialize_with = "MarkerType::deserialize"
    )]
    pub marker_type: i32,
    /// A fixed supply will mint additional coin automatically if the total supply decreases below a set value.  This
    /// may occur if the coin is burned or an account holding the coin is slashed. (default: true)
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    /// indicates that governance based control is allowed for this marker
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
    /// Whether an admin can transfer restricted coins from a 3rd-party account without their signature.
    #[prost(bool, tag = "10")]
    pub allow_forced_transfer: bool,
    /// list of required attributes on restricted marker in order to send and receive transfers if sender does not have
    /// transfer authority
    #[prost(string, repeated, tag = "11")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// NetAssetValue defines a marker's net asset value
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.NetAssetValue")]
pub struct NetAssetValue {
    /// price is the complete value of the asset's volume
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// volume is the number of tokens of the marker that were purchased for the price
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub volume: u64,
    /// updated_block_height is the block height of last update
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub updated_block_height: u64,
}
/// EventMarkerAdd event emitted when marker is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    #[prost(string, tag = "6")]
    pub address: ::prost::alloc::string::String,
}
/// EventMarkerAddAccess event emitted when marker access is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
/// EventSetNetAssetValue event emitted when Net Asset Value for marker is update or added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventSetNetAssetValue")]
pub struct EventSetNetAssetValue {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
}
/// EventMarkerParamsUpdated event emitted when marker params are updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerParamsUpdated")]
pub struct EventMarkerParamsUpdated {
    #[prost(string, tag = "1")]
    pub enable_governance: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub unrestricted_denom_regex: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub max_supply: ::prost::alloc::string::String,
}
/// MarkerType defines the types of marker
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
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
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    /// list of marker net asset values
    #[prost(message, repeated, tag = "3")]
    pub net_asset_values: ::prost::alloc::vec::Vec<MarkerNetAssetValues>,
    /// list of denom based denied send addresses
    #[prost(message, repeated, tag = "4")]
    pub deny_send_addresses: ::prost::alloc::vec::Vec<DenySendAddress>,
}
/// DenySendAddress defines addresses that are denied sends for marker denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.DenySendAddress")]
pub struct DenySendAddress {
    /// marker_address is the marker's address for denied address
    #[prost(string, tag = "1")]
    pub marker_address: ::prost::alloc::string::String,
    /// deny_address defines all wallet addresses that are denied sends for the marker
    #[prost(string, tag = "2")]
    pub deny_address: ::prost::alloc::string::String,
}
/// MarkerNetAssetValues defines the net asset values for a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerNetAssetValues")]
pub struct MarkerNetAssetValues {
    /// address defines the marker address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// net_asset_values that are assigned to marker
    #[prost(message, repeated, tag = "2")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// AddMarkerProposal is deprecated and can no longer be used.
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgAddMarkerRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.AddMarkerProposal")]
#[deprecated]
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
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
    )]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    #[serde(
        serialize_with = "MarkerType::serialize",
        deserialize_with = "MarkerType::deserialize"
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgSupplyIncreaseProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyIncreaseProposal")]
#[deprecated]
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgSupplyDecreaseProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyDecreaseProposal")]
#[deprecated]
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgSetAdministratorProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetAdministratorProposal")]
#[deprecated]
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgRemoveAdministratorProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.RemoveAdministratorProposal")]
#[deprecated]
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgChangeStatusProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.ChangeStatusProposal")]
#[deprecated]
pub struct ChangeStatusProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    #[serde(
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
    )]
    pub new_status: i32,
}
/// WithdrawEscrowProposal defines a governance proposal to withdraw escrow coins from a marker
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgWithdrawEscrowProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.WithdrawEscrowProposal")]
#[deprecated]
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
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgSetDenomMetadataProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetDenomMetadataProposal")]
#[deprecated]
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryDenomMetadataResponse")]
pub struct QueryDenomMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// QueryAccountDataRequest is the request type for the Query/AccountData
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccountDataRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/AccountData",
    response_type = QueryAccountDataResponse
)]
pub struct QueryAccountDataRequest {
    /// The denomination to look up.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryAccountDataResponse is the response type for the Query/AccountData
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccountDataResponse")]
pub struct QueryAccountDataResponse {
    /// The accountdata for the requested denom.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Balance defines an account address and balance pair used in queries for accounts holding a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
/// QueryNetAssetValuesRequest is the request type for the Query/NetAssetValues method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryNetAssetValuesRequest")]
#[proto_query(
    path = "/provenance.marker.v1.Query/NetAssetValues",
    response_type = QueryNetAssetValuesResponse
)]
pub struct QueryNetAssetValuesRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryNetAssetValuesRequest is the response type for the Query/NetAssetValues method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryNetAssetValuesResponse")]
pub struct QueryNetAssetValuesResponse {
    /// net asset values for marker denom
    #[prost(message, repeated, tag = "1")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// SIPrefix represents an International System of Units (SI) Prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
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
/// MsgGrantAllowanceRequest validates permission to create a fee grant based on marker admin access. If
/// successful a feegrant is recorded where the marker account itself is the grantor
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgGrantAllowanceResponse")]
pub struct MsgGrantAllowanceResponse {}
/// MsgAddMarkerRequest defines the Msg/AddMarker request type.
/// If being provided as a governance proposal, set the from_address to the gov module's account address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
    )]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    #[serde(
        serialize_with = "MarkerType::serialize",
        deserialize_with = "MarkerType::deserialize"
    )]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "7")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
    #[prost(bool, tag = "10")]
    pub allow_forced_transfer: bool,
    #[prost(string, repeated, tag = "11")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(uint64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_cents: u64,
    #[prost(uint64, tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub volume: u64,
    #[prost(uint64, tag = "14")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_mills: u64,
}
/// MsgAddMarkerResponse defines the Msg/AddMarker response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
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
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataResponse")]
pub struct MsgSetDenomMetadataResponse {}
/// MsgAddFinalizeActivateMarkerRequest defines the Msg/AddFinalizeActivateMarker request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddFinalizeActivateMarkerRequest")]
pub struct MsgAddFinalizeActivateMarkerRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerType", tag = "5")]
    #[serde(
        serialize_with = "MarkerType::serialize",
        deserialize_with = "MarkerType::deserialize"
    )]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "6")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "7")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "8")]
    pub allow_governance_control: bool,
    #[prost(bool, tag = "9")]
    pub allow_forced_transfer: bool,
    #[prost(string, repeated, tag = "10")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(uint64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_cents: u64,
    #[prost(uint64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub volume: u64,
    #[prost(uint64, tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_mills: u64,
}
/// MsgAddFinalizeActivateMarkerResponse defines the Msg/AddFinalizeActivateMarker response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddFinalizeActivateMarkerResponse")]
pub struct MsgAddFinalizeActivateMarkerResponse {}
/// MsgSupplyIncreaseProposalRequest defines a governance proposal to administer a marker and increase total supply of
/// the marker through minting coin and placing it within the marker or assigning it directly to an account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyIncreaseProposalRequest")]
pub struct MsgSupplyIncreaseProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// an optional target address for the minted coin from this request
    #[prost(string, tag = "2")]
    pub target_address: ::prost::alloc::string::String,
    /// signer of the proposal
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSupplyIncreaseProposalResponse defines the Msg/SupplyIncreaseProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyIncreaseProposalResponse")]
pub struct MsgSupplyIncreaseProposalResponse {}
/// MsgSupplyDecreaseProposalRequest defines a governance proposal to decrease total supply of the marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyDecreaseProposalRequest")]
pub struct MsgSupplyDecreaseProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// signer of the proposal
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSupplyIncreaseProposalResponse defines the Msg/SupplyDecreaseProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyDecreaseProposalResponse")]
pub struct MsgSupplyDecreaseProposalResponse {}
/// MsgUpdateRequiredAttributesRequest defines a msg to update/add/remove required attributes from a resticted marker
/// signer must have transfer authority to change attributes, to update attribute add current to remove list and new to
/// add list
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateRequiredAttributesRequest")]
pub struct MsgUpdateRequiredAttributesRequest {
    /// The denomination of the marker to update.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// List of required attributes to remove from marker.
    #[prost(string, repeated, tag = "2")]
    pub remove_required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of required attributes to add to marker.
    #[prost(string, repeated, tag = "3")]
    pub add_required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The signer of the message.  Must have transfer authority to marker or be governance module account address.
    #[prost(string, tag = "4")]
    pub transfer_authority: ::prost::alloc::string::String,
}
/// MsgUpdateRequiredAttributesResponse defines the Msg/UpdateRequiredAttributes response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateRequiredAttributesResponse")]
pub struct MsgUpdateRequiredAttributesResponse {}
/// MsgUpdateForcedTransferRequest defines a msg to update the allow_forced_transfer field of a marker.
/// It is only usable via governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateForcedTransferRequest")]
pub struct MsgUpdateForcedTransferRequest {
    /// The denomination of the marker to update.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Whether an admin can transfer restricted coins from a 3rd-party account without their signature.
    #[prost(bool, tag = "2")]
    pub allow_forced_transfer: bool,
    /// The signer of this message. Must be the governance module account address.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateForcedTransferResponse defines the Msg/UpdateForcedTransfer response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateForcedTransferResponse")]
pub struct MsgUpdateForcedTransferResponse {}
/// MsgSetAccountDataRequest defines a msg to set/update/delete the account data for a marker.
/// Signer must have deposit authority or be a gov proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetAccountDataRequest")]
pub struct MsgSetAccountDataRequest {
    /// The denomination of the marker to update.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// The desired accountdata value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// The signer of this message. Must have deposit authority or be the governance module account address.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSetAccountDataResponse defines the Msg/SetAccountData response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetAccountDataResponse")]
pub struct MsgSetAccountDataResponse {}
/// MsgUpdateSendDenyListRequest defines a msg to add/remove addresses to send deny list for a resticted marker
/// signer must have transfer authority
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateSendDenyListRequest")]
pub struct MsgUpdateSendDenyListRequest {
    /// The denomination of the marker to update.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// List of bech32 addresses to remove from the deny send list.
    #[prost(string, repeated, tag = "2")]
    pub remove_denied_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of bech32 addresses to add to the deny send list.
    #[prost(string, repeated, tag = "3")]
    pub add_denied_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateSendDenyListResponse defines the Msg/UpdateSendDenyList response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateSendDenyListResponse")]
pub struct MsgUpdateSendDenyListResponse {}
/// MsgAddNetAssetValuesRequest defines the Msg/AddNetAssetValues request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddNetAssetValuesRequest")]
pub struct MsgAddNetAssetValuesRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// MsgAddNetAssetValuesResponse defines the Msg/AddNetAssetValue response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddNetAssetValuesResponse")]
pub struct MsgAddNetAssetValuesResponse {}
/// MsgSetAdministratorProposalRequest defines the Msg/SetAdministratorProposal request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetAdministratorProposalRequest")]
pub struct MsgSetAdministratorProposalRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub access: ::prost::alloc::vec::Vec<AccessGrant>,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSetAdministratorProposalResponse defines the Msg/SetAdministratorProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetAdministratorProposalResponse")]
pub struct MsgSetAdministratorProposalResponse {}
/// MsgRemoveAdministratorProposalRequest defines the Msg/RemoveAdministratorProposal request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgRemoveAdministratorProposalRequest")]
pub struct MsgRemoveAdministratorProposalRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub removed_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgRemoveAdministratorProposalResponse defines the Msg/RemoveAdministratorProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgRemoveAdministratorProposalResponse")]
pub struct MsgRemoveAdministratorProposalResponse {}
/// MsgChangeStatusProposalRequest defines the Msg/ChangeStatusProposal request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgChangeStatusProposalRequest")]
pub struct MsgChangeStatusProposalRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "2")]
    #[serde(
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
    )]
    pub new_status: i32,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgChangeStatusProposalResponse defines the Msg/ChangeStatusProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgChangeStatusProposalResponse")]
pub struct MsgChangeStatusProposalResponse {}
/// MsgWithdrawEscrowProposalRequest defines the Msg/WithdrawEscrowProposal request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawEscrowProposalRequest")]
pub struct MsgWithdrawEscrowProposalRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub target_address: ::prost::alloc::string::String,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgWithdrawEscrowProposalResponse defines the Msg/WithdrawEscrowProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawEscrowProposalResponse")]
pub struct MsgWithdrawEscrowProposalResponse {}
/// MsgSetDenomMetadataProposalRequest defines the Msg/SetDenomMetadataProposal request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataProposalRequest")]
pub struct MsgSetDenomMetadataProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
    /// The signer of the message.  Must have admin authority to marker or be governance module account address.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSetDenomMetadataProposalResponse defines the Msg/SetDenomMetadataProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataProposalResponse")]
pub struct MsgSetDenomMetadataProposalResponse {}
/// MsgUpdateParamsRequest is a request message for the UpdateParams endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateParamsRequest")]
pub struct MsgUpdateParamsRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params are the new param values to set.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse is a response message for the UpdateParams endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
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
    pub fn account_data(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryAccountDataResponse, cosmwasm_std::StdError> {
        QueryAccountDataRequest { denom }.query(self.querier)
    }
    pub fn net_asset_values(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryNetAssetValuesResponse, cosmwasm_std::StdError> {
        QueryNetAssetValuesRequest { id }.query(self.querier)
    }
}
