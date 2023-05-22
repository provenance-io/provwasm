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
#[proto_message(type_url = "/cosmos.staking.v1beta1.CommissionRates")]
#[serde(rename_all = "snake_case")]
pub struct CommissionRates {
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub max_change_rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Commission")]
#[serde(rename_all = "snake_case")]
pub struct Commission {
    #[prost(message, optional, tag = "1")]
    pub commission_rates: ::core::option::Option<CommissionRates>,
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Description")]
#[serde(rename_all = "snake_case")]
pub struct Description {
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Validator")]
#[serde(rename_all = "snake_case")]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub consensus_pubkey: ::core::option::Option<crate::shim::Any>,
    #[prost(bool, tag = "3")]
    pub jailed: bool,
    #[prost(enumeration = "BondStatus", tag = "4")]
    #[serde(
        serialize_with = "BondStatus::serialize",
        deserialize_with = "BondStatus::deserialize"
    )]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub delegator_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub description: ::core::option::Option<Description>,
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_height: i64,
    #[prost(message, optional, tag = "9")]
    pub unbonding_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub commission: ::core::option::Option<Commission>,
    #[prost(string, tag = "11")]
    pub min_self_delegation: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Delegation")]
#[serde(rename_all = "snake_case")]
pub struct Delegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub unbonding_time: ::core::option::Option<crate::shim::Duration>,
    #[prost(uint32, tag = "2")]
    pub max_validators: u32,
    #[prost(uint32, tag = "3")]
    pub max_entries: u32,
    #[prost(uint32, tag = "4")]
    pub historical_entries: u32,
    #[prost(string, tag = "5")]
    pub bond_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub min_commission_rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DelegationResponse")]
#[serde(rename_all = "snake_case")]
pub struct DelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation: ::core::option::Option<Delegation>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BondStatus {
    Unspecified = 0,
    Unbonded = 1,
    Unbonding = 2,
    Bonded = 3,
}
impl BondStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED",
            BondStatus::Unbonded => "BOND_STATUS_UNBONDED",
            BondStatus::Unbonding => "BOND_STATUS_UNBONDING",
            BondStatus::Bonded => "BOND_STATUS_BONDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOND_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BOND_STATUS_UNBONDED" => Some(Self::Unbonded),
            "BOND_STATUS_UNBONDING" => Some(Self::Unbonding),
            "BOND_STATUS_BONDED" => Some(Self::Bonded),
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Validator",
    response_type = QueryValidatorResponse
)]
pub struct QueryValidatorRequest {
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryValidatorResponse {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegationRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Delegation",
    response_type = QueryDelegationResponse
)]
pub struct QueryDelegationRequest {
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegationResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryDelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation_response: ::core::option::Option<DelegationResponse>,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Params",
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
pub struct StakingQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> StakingQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn validator(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryValidatorResponse, cosmwasm_std::StdError> {
        QueryValidatorRequest { validator_addr }.query(self.querier)
    }
    pub fn delegation(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        validator_addr: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryDelegationResponse, cosmwasm_std::StdError> {
        QueryDelegationRequest {
            delegator_addr,
            validator_addr,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
