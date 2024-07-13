use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// QueryAppVersionRequest is the request type for the Query/AppVersion RPC method
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
#[proto_message(type_url = "/ibc.core.port.v1.QueryAppVersionRequest")]
#[proto_query(
    path = "/ibc.core.port.v1.Query/AppVersion",
    response_type = QueryAppVersionResponse
)]
pub struct QueryAppVersionRequest {
    /// port unique identifier
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// connection unique identifier
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// whether the channel is ordered or unordered
    #[prost(enumeration = "super::super::channel::v1::Order", tag = "3")]
    #[serde(
        serialize_with = "super::super::channel::v1::Order::serialize",
        deserialize_with = "super::super::channel::v1::Order::deserialize"
    )]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag = "4")]
    pub counterparty: ::core::option::Option<super::super::channel::v1::Counterparty>,
    /// proposed version
    #[prost(string, tag = "5")]
    pub proposed_version: ::prost::alloc::string::String,
}
/// QueryAppVersionResponse is the response type for the Query/AppVersion RPC method.
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
#[proto_message(type_url = "/ibc.core.port.v1.QueryAppVersionResponse")]
pub struct QueryAppVersionResponse {
    /// port id associated with the request identifiers
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    /// supported app version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
pub struct PortQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> PortQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn app_version(
        &self,
        port_id: ::prost::alloc::string::String,
        connection_id: ::prost::alloc::string::String,
        ordering: i32,
        counterparty: ::core::option::Option<super::super::channel::v1::Counterparty>,
        proposed_version: ::prost::alloc::string::String,
    ) -> Result<QueryAppVersionResponse, cosmwasm_std::StdError> {
        QueryAppVersionRequest {
            port_id,
            connection_id,
            ordering,
            counterparty,
            proposed_version,
        }
        .query(self.querier)
    }
}
