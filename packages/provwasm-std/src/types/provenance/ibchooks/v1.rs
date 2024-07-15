use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// EventIBCHooksParamsUpdated defines the event emitted after updating ibchooks parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibchooks.v1.EventIBCHooksParamsUpdated")]
pub struct EventIbcHooksParamsUpdated {
    #[prost(string, repeated, tag = "1")]
    pub allowed_async_ack_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the allowed async ack contracts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibchooks.v1.Params")]
pub struct Params {
    #[prost(string, repeated, tag = "1")]
    pub allowed_async_ack_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState is the IBC Hooks genesis state (params)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibchooks.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/provenance.ibchooks.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.ibchooks.v1.Query/Params",
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
#[proto_message(type_url = "/provenance.ibchooks.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgEmitIBCAck is the IBC Acknowledgement
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibchooks.v1.MsgEmitIBCAck")]
pub struct MsgEmitIbcAck {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub packet_sequence: u64,
    #[prost(string, tag = "3")]
    pub channel: ::prost::alloc::string::String,
}
/// MsgEmitIBCAckResponse is the IBC Acknowledgement response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibchooks.v1.MsgEmitIBCAckResponse")]
pub struct MsgEmitIbcAckResponse {
    #[prost(string, tag = "1")]
    pub contract_result: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ibc_ack: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/provenance.ibchooks.v1.MsgUpdateParamsRequest")]
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
#[proto_message(type_url = "/provenance.ibchooks.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct IbchooksQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> IbchooksQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
