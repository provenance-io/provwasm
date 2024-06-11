use provwasm_proc_macro::CosmwasmExt;
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
#[proto_message(type_url = "/provenance.exchange.v1.AccountAmount")]
pub struct AccountAmount {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.NetAssetPrice")]
pub struct NetAssetPrice {
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCreated")]
pub struct EventOrderCreated {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "4")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCancelled")]
pub struct EventOrderCancelled {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "2")]
    pub cancelled_by: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "4")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderFilled")]
pub struct EventOrderFilled {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "6")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderPartiallyFilled")]
pub struct EventOrderPartiallyFilled {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "6")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderExternalIDUpdated")]
pub struct EventOrderExternalIdUpdated {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "3")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketWithdraw")]
pub struct EventMarketWithdraw {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDetailsUpdated")]
pub struct EventMarketDetailsUpdated {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketEnabled")]
#[deprecated]
pub struct EventMarketEnabled {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDisabled")]
#[deprecated]
pub struct EventMarketDisabled {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleEnabled")]
pub struct EventMarketUserSettleEnabled {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleDisabled")]
pub struct EventMarketUserSettleDisabled {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketPermissionsUpdated")]
pub struct EventMarketPermissionsUpdated {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketReqAttrUpdated")]
pub struct EventMarketReqAttrUpdated {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketCreated")]
pub struct EventMarketCreated {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketFeesUpdated")]
pub struct EventMarketFeesUpdated {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.MarketAccount")]
pub struct MarketAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MarketDetails")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MarketBrief")]
pub struct MarketBrief {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Market")]
pub struct Market {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[prost(bool, tag = "14")]
    pub accepting_commitments: bool,
    #[prost(message, repeated, tag = "15")]
    pub fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint32, tag = "16")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub commitment_settlement_bips: u32,
    #[prost(string, tag = "17")]
    pub intermediary_denom: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "18")]
    pub req_attr_create_commitment: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.FeeRatio")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.AccessGrant")]
pub struct AccessGrant {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Permission", repeated, tag = "2")]
    #[serde(
        serialize_with = "Permission::serialize_vec",
        deserialize_with = "Permission::deserialize_vec"
    )]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
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

    pub fn serialize_vec<S>(v: &Vec<i32>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut enum_strs: Vec<&str> = Vec::new();
        for ord in v {
            let enum_value = Self::try_from(*ord);
            match enum_value {
                Ok(v) => {
                    enum_strs.push(v.as_str_name());
                }
                Err(e) => return Err(serde::ser::Error::custom(e)),
            }
        }
        let mut seq = serializer.serialize_tuple(enum_strs.len())?;
        for item in enum_strs {
            seq.serialize_element(item)?;
        }
        seq.end()
    }

    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};

        let strs: Vec<String> = Vec::deserialize(deserializer)?;
        let mut ords: Vec<i32> = Vec::new();
        for str_name in strs {
            let enum_value = Self::from_str_name(&str_name)
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Order")]
pub struct Order {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    use provwasm_proc_macro::CosmwasmExt;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.AskOrder")]
pub struct AskOrder {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.BidOrder")]
pub struct BidOrder {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.Params")]
pub struct Params {
    #[prost(uint32, tag = "1")]
    pub default_split: u32,
    #[prost(message, repeated, tag = "2")]
    pub denom_splits: ::prost::alloc::vec::Vec<DenomSplit>,
    #[prost(message, repeated, tag = "3")]
    pub fee_create_payment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub fee_accept_payment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.exchange.v1.DenomSplit")]
pub struct DenomSplit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub split: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.Payment")]
pub struct Payment {
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub source_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub target_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskResponse")]
pub struct MsgCreateAskResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidResponse")]
pub struct MsgCreateBidResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCommitFundsRequest")]
pub struct MsgCommitFundsRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub creation_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub event_tag: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCommitFundsResponse")]
pub struct MsgCommitFundsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderRequest")]
pub struct MsgCancelOrderRequest {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderResponse")]
pub struct MsgCancelOrderResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsRequest")]
pub struct MsgFillBidsRequest {
    #[prost(string, tag = "1")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub total_assets: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, repeated, tag = "4")]
    #[serde(alias = "bid_orderIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsResponse")]
pub struct MsgFillBidsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksRequest")]
pub struct MsgFillAsksRequest {
    #[prost(string, tag = "1")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub total_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, repeated, tag = "4")]
    #[serde(alias = "ask_orderIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksResponse")]
pub struct MsgFillAsksResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleRequest")]
pub struct MsgMarketSettleRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(uint64, repeated, tag = "3")]
    #[serde(alias = "ask_orderIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub ask_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, repeated, tag = "4")]
    #[serde(alias = "bid_orderIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub bid_order_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "5")]
    pub expect_partial: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleResponse")]
pub struct MsgMarketSettleResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketCommitmentSettleRequest")]
pub struct MsgMarketCommitmentSettleRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<AccountAmount>,
    #[prost(message, repeated, tag = "4")]
    pub outputs: ::prost::alloc::vec::Vec<AccountAmount>,
    #[prost(message, repeated, tag = "5")]
    pub fees: ::prost::alloc::vec::Vec<AccountAmount>,
    #[prost(message, repeated, tag = "6")]
    pub navs: ::prost::alloc::vec::Vec<NetAssetPrice>,
    #[prost(string, tag = "7")]
    pub event_tag: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketCommitmentSettleResponse")]
pub struct MsgMarketCommitmentSettleResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketReleaseCommitmentsRequest")]
pub struct MsgMarketReleaseCommitmentsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub to_release: ::prost::alloc::vec::Vec<AccountAmount>,
    #[prost(string, tag = "4")]
    pub event_tag: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDRequest")]
pub struct MsgMarketSetOrderExternalIdRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "orderID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    #[prost(string, tag = "4")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDResponse")]
pub struct MsgMarketSetOrderExternalIdResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawRequest")]
pub struct MsgMarketWithdrawRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawResponse")]
pub struct MsgMarketWithdrawResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsRequest")]
pub struct MsgMarketUpdateDetailsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsResponse")]
pub struct MsgMarketUpdateDetailsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledRequest")]
#[deprecated]
pub struct MsgMarketUpdateEnabledRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[deprecated]
    #[prost(bool, tag = "3")]
    pub accepting_orders: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledResponse")]
#[deprecated]
pub struct MsgMarketUpdateEnabledResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingOrdersRequest")]
pub struct MsgMarketUpdateAcceptingOrdersRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(bool, tag = "3")]
    pub accepting_orders: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingOrdersResponse")]
pub struct MsgMarketUpdateAcceptingOrdersResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleRequest")]
pub struct MsgMarketUpdateUserSettleRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(bool, tag = "3")]
    pub allow_user_settlement: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleResponse")]
pub struct MsgMarketUpdateUserSettleResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingCommitmentsRequest")]
pub struct MsgMarketUpdateAcceptingCommitmentsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(bool, tag = "3")]
    pub accepting_commitments: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingCommitmentsResponse")]
pub struct MsgMarketUpdateAcceptingCommitmentsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateIntermediaryDenomRequest")]
pub struct MsgMarketUpdateIntermediaryDenomRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "3")]
    pub intermediary_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateIntermediaryDenomResponse")]
pub struct MsgMarketUpdateIntermediaryDenomResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsRequest")]
pub struct MsgMarketManagePermissionsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsResponse")]
pub struct MsgMarketManagePermissionsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsRequest")]
pub struct MsgMarketManageReqAttrsRequest {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, repeated, tag = "3")]
    pub create_ask_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub create_ask_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub create_bid_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub create_bid_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub create_commitment_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub create_commitment_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsResponse")]
pub struct MsgMarketManageReqAttrsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreatePaymentRequest")]
pub struct MsgCreatePaymentRequest {
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreatePaymentResponse")]
pub struct MsgCreatePaymentResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgAcceptPaymentRequest")]
pub struct MsgAcceptPaymentRequest {
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgAcceptPaymentResponse")]
pub struct MsgAcceptPaymentResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentRequest")]
pub struct MsgRejectPaymentRequest {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentResponse")]
pub struct MsgRejectPaymentResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentsRequest")]
pub struct MsgRejectPaymentsRequest {
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentsResponse")]
pub struct MsgRejectPaymentsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelPaymentsRequest")]
pub struct MsgCancelPaymentsRequest {
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    #[serde(alias = "externalIDs")]
    pub external_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelPaymentsResponse")]
pub struct MsgCancelPaymentsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgChangePaymentTargetRequest")]
pub struct MsgChangePaymentTargetRequest {
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_target: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgChangePaymentTargetResponse")]
pub struct MsgChangePaymentTargetResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketResponse")]
pub struct MsgGovCreateMarketResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesRequest")]
pub struct MsgGovManageFeesRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
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
    #[prost(message, repeated, tag = "15")]
    pub add_fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "16")]
    pub remove_fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint32, tag = "17")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub set_fee_commitment_settlement_bips: u32,
    #[prost(bool, tag = "18")]
    pub unset_fee_commitment_settlement_bips: bool,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesResponse")]
pub struct MsgGovManageFeesResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCloseMarketRequest")]
pub struct MsgGovCloseMarketRequest {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCloseMarketResponse")]
pub struct MsgGovCloseMarketResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsResponse")]
pub struct MsgGovUpdateParamsResponse {}
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrder",
    response_type = QueryGetOrderResponse
)]
pub struct QueryGetOrderRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderResponse")]
pub struct QueryGetOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrderByExternalID",
    response_type = QueryGetOrderByExternalIdResponse
)]
pub struct QueryGetOrderByExternalIdRequest {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDResponse")]
pub struct QueryGetOrderByExternalIdResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarketOrders",
    response_type = QueryGetMarketOrdersResponse
)]
pub struct QueryGetMarketOrdersRequest {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    #[serde(alias = "after_orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersRequest")]
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
    #[serde(alias = "after_orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersRequest")]
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
    #[serde(alias = "after_orderID")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarket",
    response_type = QueryGetMarketResponse
)]
pub struct QueryGetMarketRequest {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/Params",
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsResponse")]
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketResponse")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateMarket",
    response_type = QueryValidateMarketResponse
)]
pub struct QueryValidateMarketRequest {
    #[prost(uint32, tag = "1")]
    #[serde(alias = "marketID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub market_id: u32,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketResponse")]
pub struct QueryValidateMarketResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesRequest")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesResponse")]
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
    ) -> Result<QueryOrderFeeCalcResponse, cosmwasm_std::StdError> {
        QueryOrderFeeCalcRequest {
            ask_order,
            bid_order,
        }
        .query(self.querier)
    }
    pub fn get_order(
        &self,
        order_id: u64,
    ) -> Result<QueryGetOrderResponse, cosmwasm_std::StdError> {
        QueryGetOrderRequest { order_id }.query(self.querier)
    }
    pub fn get_order_by_external_id(
        &self,
        market_id: u32,
        external_id: ::prost::alloc::string::String,
    ) -> Result<QueryGetOrderByExternalIdResponse, cosmwasm_std::StdError> {
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
    ) -> Result<QueryGetMarketOrdersResponse, cosmwasm_std::StdError> {
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
    ) -> Result<QueryGetOwnerOrdersResponse, cosmwasm_std::StdError> {
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
    ) -> Result<QueryGetAssetOrdersResponse, cosmwasm_std::StdError> {
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
    ) -> Result<QueryGetAllOrdersResponse, cosmwasm_std::StdError> {
        QueryGetAllOrdersRequest { pagination }.query(self.querier)
    }
    pub fn get_market(
        &self,
        market_id: u32,
    ) -> Result<QueryGetMarketResponse, cosmwasm_std::StdError> {
        QueryGetMarketRequest { market_id }.query(self.querier)
    }
    pub fn get_all_markets(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAllMarketsResponse, cosmwasm_std::StdError> {
        QueryGetAllMarketsRequest { pagination }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn validate_create_market(
        &self,
        create_market_request: ::core::option::Option<MsgGovCreateMarketRequest>,
    ) -> Result<QueryValidateCreateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateCreateMarketRequest {
            create_market_request,
        }
        .query(self.querier)
    }
    pub fn validate_market(
        &self,
        market_id: u32,
    ) -> Result<QueryValidateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateMarketRequest { market_id }.query(self.querier)
    }
    pub fn validate_manage_fees(
        &self,
        manage_fees_request: ::core::option::Option<MsgGovManageFeesRequest>,
    ) -> Result<QueryValidateManageFeesResponse, cosmwasm_std::StdError> {
        QueryValidateManageFeesRequest {
            manage_fees_request,
        }
        .query(self.querier)
    }
}
