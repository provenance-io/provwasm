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
#[proto_message(type_url = "/ibc.applications.fee.v1.Fee")]
#[serde(rename_all = "snake_case")]
pub struct Fee {
    #[prost(message, repeated, tag = "1")]
    pub recv_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub ack_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub timeout_fee:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.PacketFee")]
#[serde(rename_all = "snake_case")]
pub struct PacketFee {
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<Fee>,
    #[prost(string, tag = "2")]
    pub refund_address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterPayee")]
#[serde(rename_all = "snake_case")]
pub struct MsgRegisterPayee {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub payee: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterPayeeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRegisterPayeeResponse {}
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterCounterpartyPayee")]
#[serde(rename_all = "snake_case")]
pub struct MsgRegisterCounterpartyPayee {
    #[prost(string, tag = "1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub counterparty_payee: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterCounterpartyPayeeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgRegisterCounterpartyPayeeResponse {}
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFee")]
#[serde(rename_all = "snake_case")]
pub struct MsgPayPacketFee {
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<Fee>,
    #[prost(string, tag = "2")]
    pub source_port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgPayPacketFeeResponse {}
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeAsync")]
#[serde(rename_all = "snake_case")]
pub struct MsgPayPacketFeeAsync {
    #[prost(message, optional, tag = "1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    #[prost(message, optional, tag = "2")]
    pub packet_fee: ::core::option::Option<PacketFee>,
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeAsyncResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgPayPacketFeeAsyncResponse {}
pub struct FeeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
}
