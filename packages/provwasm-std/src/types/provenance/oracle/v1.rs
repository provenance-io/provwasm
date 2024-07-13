use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// EventOracleQuerySuccess is an event for when the chain receives a successful response from an oracle query
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
#[proto_message(type_url = "/provenance.oracle.v1.EventOracleQuerySuccess")]
pub struct EventOracleQuerySuccess {
    /// channel is the local channel that the oracle query response was received from
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    /// sequence_id is a unique identifier of the query
    #[prost(string, tag = "2")]
    pub sequence_id: ::prost::alloc::string::String,
    /// result is the data received from the query
    #[prost(string, tag = "3")]
    pub result: ::prost::alloc::string::String,
}
/// EventOracleQueryError is an event for when the chain receives an error response from an oracle query
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
#[proto_message(type_url = "/provenance.oracle.v1.EventOracleQueryError")]
pub struct EventOracleQueryError {
    /// channel is the local channel that the oracle query response was received from
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    /// sequence_id is a unique identifier of the query
    #[prost(string, tag = "2")]
    pub sequence_id: ::prost::alloc::string::String,
    /// error is the error message received from the query
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
/// EventOracleQueryTimeout is an event for when the chain receives a timeout from an oracle query
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
#[proto_message(type_url = "/provenance.oracle.v1.EventOracleQueryTimeout")]
pub struct EventOracleQueryTimeout {
    /// channel is the local channel that the oracle timeout was received from
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    /// sequence_id is a unique identifier of the query
    #[prost(string, tag = "2")]
    pub sequence_id: ::prost::alloc::string::String,
}
/// GenesisState defines the oracle module's genesis state.
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
#[proto_message(type_url = "/provenance.oracle.v1.GenesisState")]
pub struct GenesisState {
    /// The port to assign to the module
    #[prost(string, tag = "2")]
    pub port_id: ::prost::alloc::string::String,
    /// The address of the oracle
    #[prost(string, tag = "3")]
    pub oracle: ::prost::alloc::string::String,
}
/// QueryOracleAddressRequest queries for the address of the oracle.
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
#[proto_message(type_url = "/provenance.oracle.v1.QueryOracleAddressRequest")]
#[proto_query(
    path = "/provenance.oracle.v1.Query/OracleAddress",
    response_type = QueryOracleAddressResponse
)]
pub struct QueryOracleAddressRequest {}
/// QueryOracleAddressResponse contains the address of the oracle.
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
#[proto_message(type_url = "/provenance.oracle.v1.QueryOracleAddressResponse")]
pub struct QueryOracleAddressResponse {
    /// The address of the oracle
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryOracleRequest queries the module's oracle.
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
#[proto_message(type_url = "/provenance.oracle.v1.QueryOracleRequest")]
#[proto_query(
    path = "/provenance.oracle.v1.Query/Oracle",
    response_type = QueryOracleResponse
)]
pub struct QueryOracleRequest {
    /// Query contains the query data passed to the oracle.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub query: ::prost::alloc::vec::Vec<u8>,
}
/// QueryOracleResponse contains the result of the query sent to the oracle.
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
#[proto_message(type_url = "/provenance.oracle.v1.QueryOracleResponse")]
pub struct QueryOracleResponse {
    /// Data contains the json data returned from the oracle.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgSendQueryOracleRequest queries an oracle on another chain
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
#[proto_message(type_url = "/provenance.oracle.v1.MsgSendQueryOracleRequest")]
pub struct MsgSendQueryOracleRequest {
    /// Query contains the query data passed to the oracle.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub query: ::prost::alloc::vec::Vec<u8>,
    /// Channel is the channel to the oracle.
    #[prost(string, tag = "3")]
    pub channel: ::prost::alloc::string::String,
    /// The signing authority for the request
    #[prost(string, tag = "4")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSendQueryOracleResponse contains the id of the oracle query.
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
#[proto_message(type_url = "/provenance.oracle.v1.MsgSendQueryOracleResponse")]
pub struct MsgSendQueryOracleResponse {
    /// The sequence number that uniquely identifies the query.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// MsgUpdateOracleRequest is the request type for updating an oracle's contract address
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
#[proto_message(type_url = "/provenance.oracle.v1.MsgUpdateOracleRequest")]
pub struct MsgUpdateOracleRequest {
    /// The address of the oracle's contract
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// The signing authorities for the request
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateOracleResponse is the response type for updating the oracle.
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
#[proto_message(type_url = "/provenance.oracle.v1.MsgUpdateOracleResponse")]
pub struct MsgUpdateOracleResponse {}
pub struct OracleQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> OracleQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn oracle_address(&self) -> Result<QueryOracleAddressResponse, cosmwasm_std::StdError> {
        QueryOracleAddressRequest {}.query(self.querier)
    }
    pub fn oracle(
        &self,
        query: ::prost::alloc::vec::Vec<u8>,
    ) -> Result<QueryOracleResponse, cosmwasm_std::StdError> {
        QueryOracleRequest { query }.query(self.querier)
    }
}
