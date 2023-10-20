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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCreated")]
#[serde(rename_all = "snake_case")]
pub struct EventOrderCreated {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub market_id: u32,
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCancelled")]
#[serde(rename_all = "snake_case")]
pub struct EventOrderCancelled {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub cancelled_by: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub market_id: u32,
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderFilled")]
#[serde(rename_all = "snake_case")]
pub struct EventOrderFilled {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub assets: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub fees: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub market_id: u32,
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderPartiallyFilled")]
#[serde(rename_all = "snake_case")]
pub struct EventOrderPartiallyFilled {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub assets: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub fees: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub market_id: u32,
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderExternalIDUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventOrderExternalIdUpdated {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketWithdraw")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketWithdraw {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub destination: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub withdrawn_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDetailsUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketDetailsUpdated {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketEnabled")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketEnabled {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDisabled")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketDisabled {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleEnabled")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketUserSettleEnabled {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleDisabled")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketUserSettleDisabled {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketPermissionsUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketPermissionsUpdated {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketReqAttrUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketReqAttrUpdated {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketCreated")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketCreated {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketFeesUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventMarketFeesUpdated {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventParamsUpdated")]
#[serde(rename_all = "snake_case")]
pub struct EventParamsUpdated {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MarketAccount")]
#[serde(rename_all = "snake_case")]
pub struct MarketAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MarketDetails")]
#[serde(rename_all = "snake_case")]
pub struct MarketDetails {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub website_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub icon_uri: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MarketBrief")]
#[serde(rename_all = "snake_case")]
pub struct MarketBrief {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub market_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
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
#[proto_message(type_url = "/provenance.exchange.v1.Market")]
#[serde(rename_all = "snake_case")]
pub struct Market {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(message, optional, tag = "2")]
    pub market_details: ::core::option::Option<MarketDetails>,
    #[prost(message, repeated, tag = "3")]
    pub fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "5")]
    pub fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "6")]
    pub fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    #[prost(message, repeated, tag = "7")]
    pub fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "8")]
    pub fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    #[prost(bool, tag = "9")]
    pub accepting_orders: bool,
    #[prost(bool, tag = "10")]
    pub allow_user_settlement: bool,
    #[prost(message, repeated, tag = "11")]
    pub access_grants: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(string, repeated, tag = "12")]
    pub req_attr_create_ask: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "13")]
    pub req_attr_create_bid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.FeeRatio")]
#[serde(rename_all = "snake_case")]
pub struct FeeRatio {
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.AccessGrant")]
#[serde(rename_all = "snake_case")]
pub struct AccessGrant {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Permission", repeated, tag = "2")]
    #[serde(
        serialize_with = "Permission::serialize",
        deserialize_with = "Permission::deserialize"
    )]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Unspecified = 0,
    Settle = 1,
    SetIds = 2,
    Cancel = 3,
    Withdraw = 4,
    Update = 5,
    Permissions = 6,
    Attributes = 7,
}
impl Permission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Permission::Unspecified => "PERMISSION_UNSPECIFIED",
            Permission::Settle => "PERMISSION_SETTLE",
            Permission::SetIds => "PERMISSION_SET_IDS",
            Permission::Cancel => "PERMISSION_CANCEL",
            Permission::Withdraw => "PERMISSION_WITHDRAW",
            Permission::Update => "PERMISSION_UPDATE",
            Permission::Permissions => "PERMISSION_PERMISSIONS",
            Permission::Attributes => "PERMISSION_ATTRIBUTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_SETTLE" => Some(Self::Settle),
            "PERMISSION_SET_IDS" => Some(Self::SetIds),
            "PERMISSION_CANCEL" => Some(Self::Cancel),
            "PERMISSION_WITHDRAW" => Some(Self::Withdraw),
            "PERMISSION_UPDATE" => Some(Self::Update),
            "PERMISSION_PERMISSIONS" => Some(Self::Permissions),
            "PERMISSION_ATTRIBUTES" => Some(Self::Attributes),
            _ => None,
        }
    }
    pub fn serialize<S>(v: &Vec<i32>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut permission_vec: Vec<&str> = Vec::new();
        for permission_i32 in v {
            let enum_value = Permission::from_repr(*permission_i32);
            match enum_value {
                Some(v) => {
                    permission_vec.push(v.as_str_name());
                }
                None => return Err(serde::ser::Error::custom("unknown value")),
            }
        }
        let mut seq = serializer.serialize_tuple(permission_vec.len())?;
        for item in permission_vec {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
    fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};

        let strs: Vec<String> = Vec::deserialize(deserializer)?;
        let mut ords: Vec<i32> = Vec::new();
        for str_name in strs {
            let enum_value = Permission::from_str_name(&str_name)
                .ok_or_else(|| Error::custom(format!("unknown enum string: {}", str_name)))?;
            ords.push(enum_value as i32);
        }
        Ok(ords)
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
#[proto_message(type_url = "/provenance.exchange.v1.Order")]
#[serde(rename_all = "snake_case")]
pub struct Order {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(oneof = "order::Order", tags = "2, 3")]
    pub order: ::core::option::Option<order::Order>,
}
/// Nested message and enum types in `Order`.
pub mod order {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone, PartialEq, ::prost::Oneof, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
    )]
    #[serde(rename_all = "snake_case")]
    pub enum Order {
        #[prost(message, tag = "2")]
        AskOrder(super::AskOrder),
        #[prost(message, tag = "3")]
        BidOrder(super::BidOrder),
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
#[proto_message(type_url = "/provenance.exchange.v1.AskOrder")]
#[serde(rename_all = "snake_case")]
pub struct AskOrder {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub seller: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub seller_settlement_flat_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(bool, tag = "6")]
    pub allow_partial: bool,
    #[prost(string, tag = "7")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.BidOrder")]
#[serde(rename_all = "snake_case")]
pub struct BidOrder {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "5")]
    pub buyer_settlement_fees:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(bool, tag = "6")]
    pub allow_partial: bool,
    #[prost(string, tag = "7")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[prost(uint32, tag = "1")]
    pub default_split: u32,
    #[prost(message, repeated, tag = "2")]
    pub denom_splits: ::prost::alloc::vec::Vec<DenomSplit>,
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
#[proto_message(type_url = "/provenance.exchange.v1.DenomSplit")]
#[serde(rename_all = "snake_case")]
pub struct DenomSplit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub split: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateAskRequest {
    #[prost(message, optional, tag = "1")]
    pub ask_order: ::core::option::Option<AskOrder>,
    #[prost(message, optional, tag = "2")]
    pub order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateAskResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateBidRequest {
    #[prost(message, optional, tag = "1")]
    pub bid_order: ::core::option::Option<BidOrder>,
    #[prost(message, optional, tag = "2")]
    pub order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateBidResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgCancelOrderRequest {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCancelOrderResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgFillBidsRequest {
    #[prost(string, tag = "1")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub total_assets: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, repeated, tag = "4")]
    pub bid_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "5")]
    pub seller_settlement_flat_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub ask_order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgFillBidsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgFillAsksRequest {
    #[prost(string, tag = "1")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub total_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, repeated, tag = "4")]
    pub ask_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, repeated, tag = "5")]
    pub buyer_settlement_fees:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub bid_order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgFillAsksResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketSettleRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(uint64, repeated, tag = "3")]
    pub ask_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "4")]
    pub bid_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "5")]
    pub expect_partial: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketSettleResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketSetOrderExternalIdRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketSetOrderExternalIdResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketWithdrawRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketWithdrawResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateDetailsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateDetailsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateEnabledRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(bool, tag = "3")]
    pub accepting_orders: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateEnabledResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateUserSettleRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(bool, tag = "3")]
    pub allow_user_settlement: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketUpdateUserSettleResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketManagePermissionsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(string, repeated, tag = "3")]
    pub revoke_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub to_revoke: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(message, repeated, tag = "5")]
    pub to_grant: ::prost::alloc::vec::Vec<AccessGrant>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketManagePermissionsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketManageReqAttrsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(string, repeated, tag = "3")]
    pub create_ask_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub create_ask_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub create_bid_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub create_bid_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMarketManageReqAttrsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovCreateMarketRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub market: ::core::option::Option<Market>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovCreateMarketResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovManageFeesRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub add_fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub remove_fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "5")]
    pub add_fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "6")]
    pub remove_fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "7")]
    pub add_fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "8")]
    pub remove_fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "9")]
    pub add_fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    #[prost(message, repeated, tag = "10")]
    pub remove_fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    #[prost(message, repeated, tag = "11")]
    pub add_fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "12")]
    pub remove_fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "13")]
    pub add_fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    #[prost(message, repeated, tag = "14")]
    pub remove_fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovManageFeesResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovUpdateParamsRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgGovUpdateParamsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/OrderFeeCalc",
    response_type = QueryOrderFeeCalcResponse
)]
pub struct QueryOrderFeeCalcRequest {
    #[prost(message, optional, tag = "2")]
    pub ask_order: ::core::option::Option<AskOrder>,
    #[prost(message, optional, tag = "3")]
    pub bid_order: ::core::option::Option<BidOrder>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryOrderFeeCalcResponse {
    #[prost(message, repeated, tag = "1")]
    pub creation_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub settlement_flat_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub settlement_ratio_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrder",
    response_type = QueryGetOrderResponse
)]
pub struct QueryGetOrderRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrderByExternalID",
    response_type = QueryGetOrderByExternalIdResponse
)]
pub struct QueryGetOrderByExternalIdRequest {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetOrderByExternalIdResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarketOrders",
    response_type = QueryGetMarketOrdersResponse
)]
pub struct QueryGetMarketOrdersRequest {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetMarketOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOwnerOrders",
    response_type = QueryGetOwnerOrdersResponse
)]
pub struct QueryGetOwnerOrdersRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetOwnerOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAssetOrders",
    response_type = QueryGetAssetOrdersResponse
)]
pub struct QueryGetAssetOrdersRequest {
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetAssetOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllOrders",
    response_type = QueryGetAllOrdersResponse
)]
pub struct QueryGetAllOrdersRequest {
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetAllOrdersResponse {
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarket",
    response_type = QueryGetMarketResponse
)]
pub struct QueryGetMarketRequest {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetMarketResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub market: ::core::option::Option<Market>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllMarkets",
    response_type = QueryGetAllMarketsResponse
)]
pub struct QueryGetAllMarketsRequest {
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryGetAllMarketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<MarketBrief>,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/Params",
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsResponse")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateCreateMarket",
    response_type = QueryValidateCreateMarketResponse
)]
pub struct QueryValidateCreateMarketRequest {
    #[prost(message, optional, tag = "1")]
    pub create_market_request: ::core::option::Option<MsgGovCreateMarketRequest>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryValidateCreateMarketResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub gov_prop_will_pass: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateMarket",
    response_type = QueryValidateMarketResponse
)]
pub struct QueryValidateMarketRequest {
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryValidateMarketResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateManageFees",
    response_type = QueryValidateManageFeesResponse
)]
pub struct QueryValidateManageFeesRequest {
    #[prost(message, optional, tag = "1")]
    pub manage_fees_request: ::core::option::Option<MsgGovManageFeesRequest>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryValidateManageFeesResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub gov_prop_will_pass: bool,
}
pub struct ExchangeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ExchangeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn order_fee_calc(
        &self,
        ask_order: ::core::option::Option<AskOrder>,
        bid_order: ::core::option::Option<BidOrder>,
    ) -> std::result::Result<QueryOrderFeeCalcResponse, cosmwasm_std::StdError> {
        QueryOrderFeeCalcRequest {
            ask_order,
            bid_order,
        }
        .query(self.querier)
    }
    pub fn get_order(
        &self,
        order_id: u64,
    ) -> std::result::Result<QueryGetOrderResponse, cosmwasm_std::StdError> {
        QueryGetOrderRequest { order_id }.query(self.querier)
    }
    pub fn get_order_by_external_id(
        &self,
        market_id: u32,
        external_id: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryGetOrderByExternalIdResponse, cosmwasm_std::StdError> {
        QueryGetOrderByExternalIdRequest {
            market_id,
            external_id,
        }
        .query(self.querier)
    }
    pub fn get_market_orders(
        &self,
        market_id: u32,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGetMarketOrdersResponse, cosmwasm_std::StdError> {
        QueryGetMarketOrdersRequest {
            market_id,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_owner_orders(
        &self,
        owner: ::prost::alloc::string::String,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGetOwnerOrdersResponse, cosmwasm_std::StdError> {
        QueryGetOwnerOrdersRequest {
            owner,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_asset_orders(
        &self,
        asset: ::prost::alloc::string::String,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGetAssetOrdersResponse, cosmwasm_std::StdError> {
        QueryGetAssetOrdersRequest {
            asset,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_all_orders(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGetAllOrdersResponse, cosmwasm_std::StdError> {
        QueryGetAllOrdersRequest { pagination }.query(self.querier)
    }
    pub fn get_market(
        &self,
        market_id: u32,
    ) -> std::result::Result<QueryGetMarketResponse, cosmwasm_std::StdError> {
        QueryGetMarketRequest { market_id }.query(self.querier)
    }
    pub fn get_all_markets(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryGetAllMarketsResponse, cosmwasm_std::StdError> {
        QueryGetAllMarketsRequest { pagination }.query(self.querier)
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn validate_create_market(
        &self,
        create_market_request: ::core::option::Option<MsgGovCreateMarketRequest>,
    ) -> std::result::Result<QueryValidateCreateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateCreateMarketRequest {
            create_market_request,
        }
        .query(self.querier)
    }
    pub fn validate_market(
        &self,
        market_id: u32,
    ) -> std::result::Result<QueryValidateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateMarketRequest { market_id }.query(self.querier)
    }
    pub fn validate_manage_fees(
        &self,
        manage_fees_request: ::core::option::Option<MsgGovManageFeesRequest>,
    ) -> std::result::Result<QueryValidateManageFeesResponse, cosmwasm_std::StdError> {
        QueryValidateManageFeesRequest {
            manage_fees_request,
        }
        .query(self.querier)
    }
}
