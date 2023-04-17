use provwasm_std_derive::CosmwasmExt;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Access {
    Unspecified = 0,
    Mint = 1,
    Burn = 2,
    Deposit = 3,
    Withdraw = 4,
    Delete = 5,
    Admin = 6,
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
    #[prost(message, repeated, tag = "1")]
    pub transfer_limit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, repeated, tag = "2")]
    pub allow_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_total_supply: u64,
    #[prost(bool, tag = "2")]
    pub enable_governance: bool,
    #[prost(string, tag = "3")]
    pub unrestricted_denom_regex: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(string, tag = "2")]
    pub manager: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub access_control: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub supply: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerType", tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub marker_type: i32,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
    #[prost(bool, tag = "10")]
    pub allow_forced_transfer: bool,
    #[prost(string, repeated, tag = "11")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
}
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum MarkerType {
    Unspecified = 0,
    Coin = 1,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum MarkerStatus {
    Unspecified = 0,
    Proposed = 1,
    Finalized = 2,
    Active = 3,
    Cancelled = 4,
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
pub struct SupplyIncreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub target_address: ::prost::alloc::string::String,
}
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
pub struct SupplyDecreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
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
pub struct SetDenomMetadataProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
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
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
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
    #[prost(enumeration = "MarkerStatus", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum SiPrefix {
    None = 0,
    Deka = 1,
    Hecto = 2,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
    Peta = 15,
    Exa = 18,
    Zetta = 21,
    Yotta = 24,
    Deci = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano = -9,
    Pico = -12,
    Femto = -15,
    Atto = -18,
    Zepto = -21,
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
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub allowance: ::core::option::Option<crate::shim::Any>,
}
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
    #[prost(bool, tag = "10")]
    pub allow_forced_transfer: bool,
    #[prost(string, repeated, tag = "11")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
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
}
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
    #[prost(string, tag = "2")]
    pub target_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
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
