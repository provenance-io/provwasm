#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub community_tax: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_proposer_reward: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub bonus_proposer_reward: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub withdraw_addr_enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.ValidatorAccumulatedCommission")]
#[serde(rename_all = "snake_case")]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag = "1")]
    pub commission: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.distribution.v1beta1.Query/Params",
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
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryParamsResponse")]
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
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryValidatorCommissionRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.distribution.v1beta1.Query/ValidatorCommission",
    response_type = QueryValidatorCommissionResponse
)]
pub struct QueryValidatorCommissionRequest {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryValidatorCommissionResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryValidatorCommissionResponse {
    #[prost(message, optional, tag = "1")]
    pub commission: ::core::option::Option<ValidatorAccumulatedCommission>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
    response_type = QueryDelegatorWithdrawAddressResponse
)]
pub struct QueryDelegatorWithdrawAddressRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryDelegatorWithdrawAddressResponse {
    #[prost(string, tag = "1")]
    pub withdraw_address: ::prost::alloc::string::String,
}
pub struct DistributionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> DistributionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn validator_commission(
        &self,
        validator_address: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryValidatorCommissionResponse, cosmwasm_std::StdError> {
        QueryValidatorCommissionRequest { validator_address }.query(self.querier)
    }
    pub fn delegator_withdraw_address(
        &self,
        delegator_address: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryDelegatorWithdrawAddressResponse, cosmwasm_std::StdError> {
        QueryDelegatorWithdrawAddressRequest { delegator_address }.query(self.querier)
    }
}
