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
#[proto_message(type_url = "/provenance.trigger.v1.Trigger")]
#[serde(rename_all = "snake_case")]
pub struct Trigger {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<crate::shim::Any>,
    #[prost(message, repeated, tag = "4")]
    pub actions: ::prost::alloc::vec::Vec<crate::shim::Any>,
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
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggerByIDRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.trigger.v1.Query/TriggerByID",
    response_type = QueryTriggerByIdResponse
)]
pub struct QueryTriggerByIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
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
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggerByIDResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryTriggerByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
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
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggersRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.trigger.v1.Query/Triggers",
    response_type = QueryTriggersResponse
)]
pub struct QueryTriggersRequest {
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
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
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggersResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryTriggersResponse {
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/provenance.trigger.v1.MsgCreateTriggerRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateTriggerRequest {
    #[prost(string, repeated, tag = "1")]
    pub authorities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<crate::shim::Any>,
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<crate::shim::Any>,
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
#[proto_message(type_url = "/provenance.trigger.v1.MsgCreateTriggerResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgCreateTriggerResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
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
#[proto_message(type_url = "/provenance.trigger.v1.MsgDestroyTriggerRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgDestroyTriggerRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.trigger.v1.MsgDestroyTriggerResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDestroyTriggerResponse {}
pub struct TriggerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TriggerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn trigger_by_id(
        &self,
        id: u64,
    ) -> std::result::Result<QueryTriggerByIdResponse, cosmwasm_std::StdError> {
        QueryTriggerByIdRequest { id }.query(self.querier)
    }
    pub fn triggers(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryTriggersResponse, cosmwasm_std::StdError> {
        QueryTriggersRequest { pagination }.query(self.querier)
    }
}
