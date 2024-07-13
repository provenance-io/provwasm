use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// EventAckRevertFailure is emitted when an Ack revert fails
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.EventAckRevertFailure")]
pub struct EventAckRevertFailure {
    /// module is the name of the module that emitted it.
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// packet is the packet received on acknowledgement.
    #[prost(string, tag = "2")]
    pub packet: ::prost::alloc::string::String,
    /// ack is the packet's inner acknowledgement message.
    #[prost(string, tag = "3")]
    pub ack: ::prost::alloc::string::String,
}
/// EventTimeoutRevertFailure is emitted when a Timeout revert fails
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.EventTimeoutRevertFailure")]
pub struct EventTimeoutRevertFailure {
    /// module is the name of the module that emitted it.
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// packet is the packet received on timeout.
    #[prost(string, tag = "2")]
    pub packet: ::prost::alloc::string::String,
}
/// EventParamsUpdated is an event emitted when the ibcratelimit module's params have been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.EventParamsUpdated")]
pub struct EventParamsUpdated {}
/// Params defines the parameters for the ibcratelimit module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.Params")]
pub struct Params {
    /// contract_address is the address of the rate limiter contract.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// GenesisState defines the ibcratelimit module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.GenesisState")]
pub struct GenesisState {
    /// params are all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// ParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.ParamsRequest")]
#[proto_query(
    path = "/provenance.ibcratelimit.v1.Query/Params",
    response_type = ParamsResponse
)]
pub struct ParamsRequest {}
/// ParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.ParamsResponse")]
pub struct ParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgGovUpdateParamsRequest is a request message for the GovUpdateParams endpoint.
/// Deprecated: Use MsgUpdateParamsRequest instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.MsgGovUpdateParamsRequest")]
#[deprecated]
pub struct MsgGovUpdateParamsRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params are the new param values to set.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgGovUpdateParamsResponse is a response message for the GovUpdateParams endpoint.
/// Deprecated: Use MsgUpdateParamsResponse instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.ibcratelimit.v1.MsgGovUpdateParamsResponse")]
#[deprecated]
pub struct MsgGovUpdateParamsResponse {}
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
#[proto_message(type_url = "/provenance.ibcratelimit.v1.MsgUpdateParamsRequest")]
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
#[proto_message(type_url = "/provenance.ibcratelimit.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct IbcratelimitQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> IbcratelimitQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<ParamsResponse, cosmwasm_std::StdError> {
        ParamsRequest {}.query(self.querier)
    }
}
