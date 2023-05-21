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
#[proto_message(type_url = "/cosmos.authz.v1beta1.GenericAuthorization")]
#[serde(rename_all = "snake_case")]
pub struct GenericAuthorization {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.Grant")]
#[serde(rename_all = "snake_case")]
pub struct Grant {
    #[prost(message, optional, tag = "1")]
    pub authorization: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.GrantAuthorization")]
#[serde(rename_all = "snake_case")]
pub struct GrantAuthorization {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub authorization: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "4")]
    pub expiration: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.GrantQueueItem")]
#[serde(rename_all = "snake_case")]
pub struct GrantQueueItem {
    #[prost(string, repeated, tag = "1")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
