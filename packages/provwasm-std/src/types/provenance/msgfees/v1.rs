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
#[proto_message(type_url = "/provenance.msgfees.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[prost(message, optional, tag = "2")]
    pub floor_gas_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgFee")]
#[serde(rename_all = "snake_case")]
pub struct MsgFee {
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub recipient_basis_points: u32,
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
#[proto_message(type_url = "/provenance.msgfees.v1.EventMsgFee")]
#[serde(rename_all = "snake_case")]
pub struct EventMsgFee {
    #[prost(string, tag = "1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub count: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.msgfees.v1.EventMsgFees")]
#[serde(rename_all = "snake_case")]
pub struct EventMsgFees {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<EventMsgFee>,
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/Params",
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryParamsResponse")]
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgAssessCustomMsgFeeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub recipient_basis_points: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAssessCustomMsgFeeResponse {}
pub struct MsgfeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MsgfeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
