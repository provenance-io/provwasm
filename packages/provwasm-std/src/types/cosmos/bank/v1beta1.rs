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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[deprecated]
    #[prost(message, repeated, tag = "1")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
    #[prost(bool, tag = "2")]
    pub default_send_enabled: bool,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.SendEnabled")]
#[serde(rename_all = "snake_case")]
pub struct SendEnabled {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Input")]
#[serde(rename_all = "snake_case")]
pub struct Input {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Output")]
#[serde(rename_all = "snake_case")]
pub struct Output {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.DenomUnit")]
#[serde(rename_all = "snake_case")]
pub struct DenomUnit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub exponent: u32,
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Metadata")]
#[serde(rename_all = "snake_case")]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub denom_units: ::prost::alloc::vec::Vec<DenomUnit>,
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub display: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub uri_hash: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryBalanceResponse {
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SupplyOf",
    response_type = QuerySupplyOfResponse
)]
pub struct QuerySupplyOfRequest {
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfResponse")]
#[serde(rename_all = "snake_case")]
pub struct QuerySupplyOfResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/Params",
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryParamsResponse")]
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomMetadataRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/DenomMetadata",
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomMetadataResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryDenomMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<Metadata>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSend")]
#[serde(rename_all = "snake_case")]
pub struct MsgSend {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSendResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSendResponse {}
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgMultiSend")]
#[serde(rename_all = "snake_case")]
pub struct MsgMultiSend {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgMultiSendResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgMultiSendResponse {}
pub struct BankQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> BankQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn balance(
        &self,
        address: ::prost::alloc::string::String,
        denom: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest { address, denom }.query(self.querier)
    }
    pub fn supply_of(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> std::result::Result<QuerySupplyOfResponse, cosmwasm_std::StdError> {
        QuerySupplyOfRequest { denom }.query(self.querier)
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn denom_metadata(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryDenomMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomMetadataRequest { denom }.query(self.querier)
    }
}
