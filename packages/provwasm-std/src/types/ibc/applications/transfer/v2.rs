use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// FungibleTokenPacketData defines a struct for the packet payload
/// See FungibleTokenPacketData spec:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
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
#[proto_message(type_url = "/ibc.applications.transfer.v2.FungibleTokenPacketData")]
pub struct FungibleTokenPacketData {
    /// the token denomination to be transferred
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// the token amount to be transferred
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// the sender address
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
    /// optional memo
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
}
