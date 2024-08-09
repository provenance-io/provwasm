pub mod tonic;
use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
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
include!("provenance.ibchooks.v1.tonic.rs");
