#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.AccessGrant")]
#[serde(rename_all = "snake_case")]
pub struct AccessGrant {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Access", repeated, packed = "false", tag = "2")]
    #[serde(
        serialize_with = "serialize_access_vec",
        deserialize_with = "deserialize_access_vec"
    )]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
pub fn serialize_access_vec<S>(v: &Vec<i32>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeTuple;

    let mut access_vec: Vec<&str> = Vec::new();
    for access_i32 in v {
        let enum_value = Access::from_repr(*access_i32);
        match enum_value {
            Some(v) => {
                access_vec.push(v.as_str_name());
            }
            None => return Err(serde::ser::Error::custom("unknown value")),
        }
    }
    let mut seq = serializer.serialize_tuple(access_vec.len())?;
    for item in access_vec {
        seq.serialize_element(item)?;
    }
    seq.end()
}
fn deserialize_access_vec<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Deserialize, Error};

    let strs: Vec<String> = Vec::deserialize(deserializer)?;
    let mut ords: Vec<i32> = Vec::new();
    for str_name in strs {
        let enum_value = Access::from_str_name(&str_name)
            .ok_or_else(|| Error::custom(format!("unknown enum string: {}", str_name)))?;
        ords.push(enum_value as i32);
    }
    Ok(ords)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    Unspecified = 0,
    Mint = 1,
    Burn = 2,
    Deposit = 3,
    Withdraw = 4,
    Delete = 5,
    Admin = 6,
    Transfer = 7,
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
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerTransferAuthorization")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[deprecated]
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
    #[prost(string, tag = "4")]
    pub max_supply: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MarkerAccount")]
#[serde(rename_all = "snake_case")]
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
        serialize_with = "MarkerStatus::serialize",
        deserialize_with = "MarkerStatus::deserialize"
    )]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub supply: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerType", tag = "7")]
    #[serde(
        serialize_with = "MarkerType::serialize",
        deserialize_with = "MarkerType::deserialize"
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAdd")]
#[serde(rename_all = "snake_case")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAddAccess")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerAccess")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerDeleteAccess")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerFinalize")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerActivate")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerCancel")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerDelete")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerMint")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerBurn")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerWithdraw")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerTransfer")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventMarkerSetDenomMetadata")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.EventDenomUnit")]
#[serde(rename_all = "snake_case")]
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
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyIncreaseProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SupplyDecreaseProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetAdministratorProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.RemoveAdministratorProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.ChangeStatusProposal")]
#[serde(rename_all = "snake_case")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.WithdrawEscrowProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.SetDenomMetadataProposal")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryMarkerRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Marker",
    response_type = QueryMarkerResponse
)]
pub struct QueryMarkerRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryMarkerResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryMarkerResponse {
    #[prost(message, optional, tag = "1")]
    pub marker: ::core::option::Option<crate::shim::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryHoldingRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Holding",
    response_type = QueryHoldingResponse
)]
pub struct QueryHoldingRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryHoldingResponse")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QuerySupplyRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Supply",
    response_type = QuerySupplyResponse
)]
pub struct QuerySupplyRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QuerySupplyResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySupplyResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryEscrowRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Escrow",
    response_type = QueryEscrowResponse
)]
pub struct QueryEscrowRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryEscrowResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryEscrowResponse {
    #[prost(message, repeated, tag = "1")]
    pub escrow: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccessRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.marker.v1.Query/Access",
    response_type = QueryAccessResponse
)]
pub struct QueryAccessRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryAccessResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryAccessResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccessGrant>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryDenomMetadataRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.QueryDenomMetadataResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryDenomMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.Balance")]
#[serde(rename_all = "snake_case")]
pub struct Balance {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgGrantAllowanceRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgGrantAllowanceResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGrantAllowanceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddMarkerRequest")]
#[serde(rename_all = "snake_case")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddMarkerResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddMarkerResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddAccessRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddAccessResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddAccessResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteAccessRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteAccessResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteAccessResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgFinalizeRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgFinalizeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgFinalizeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgActivateRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgActivateResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgActivateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgCancelRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgCancelResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCancelResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgDeleteResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgMintRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgMintResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMintResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgBurnRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgBurnResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgBurnResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawRequest")]
#[serde(rename_all = "snake_case")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgWithdrawResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgWithdrawResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgTransferRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgTransferResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgTransferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgIbcTransferRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgIbcTransferRequest {
    #[prost(message, optional, tag = "1")]
    pub transfer:
        ::core::option::Option<super::super::super::ibc::applications::transfer::v1::MsgTransfer>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgIbcTransferResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgIbcTransferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSetDenomMetadataResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSetDenomMetadataResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddFinalizeActivateMarkerRequest")]
#[serde(rename_all = "snake_case")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgAddFinalizeActivateMarkerResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddFinalizeActivateMarkerResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyIncreaseProposalRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.marker.v1.MsgSupplyIncreaseProposalResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSupplyIncreaseProposalResponse {}
pub struct MarkerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MarkerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn marker(
        &self,
        id: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryMarkerResponse, cosmwasm_std::StdError> {
        QueryMarkerRequest { id }.query(self.querier)
    }
    pub fn holding(
        &self,
        id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryHoldingResponse, cosmwasm_std::StdError> {
        QueryHoldingRequest { id, pagination }.query(self.querier)
    }
    pub fn supply(
        &self,
        id: ::prost::alloc::string::String,
    ) -> std::result::Result<QuerySupplyResponse, cosmwasm_std::StdError> {
        QuerySupplyRequest { id }.query(self.querier)
    }
    pub fn escrow(
        &self,
        id: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryEscrowResponse, cosmwasm_std::StdError> {
        QueryEscrowRequest { id }.query(self.querier)
    }
    pub fn access(
        &self,
        id: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryAccessResponse, cosmwasm_std::StdError> {
        QueryAccessRequest { id }.query(self.querier)
    }
    pub fn denom_metadata(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryDenomMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomMetadataRequest { denom }.query(self.querier)
    }
}
