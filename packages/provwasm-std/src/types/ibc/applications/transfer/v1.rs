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
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransfer")]
#[serde(rename_all = "snake_case")]
pub struct MsgTransfer {
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_timestamp: u64,
    #[prost(string, tag = "8")]
    pub memo: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransferResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgTransferResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
