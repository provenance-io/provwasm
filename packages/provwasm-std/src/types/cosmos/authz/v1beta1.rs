use provwasm_proc_macro::CosmwasmExt;
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.GenericAuthorization")]
pub struct GenericAuthorization {
    #[prost(string, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.Grant")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.authz.v1beta1.GrantAuthorization")]
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
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.authz.v1beta1.GrantQueueItem")]
pub struct GrantQueueItem {
    #[prost(string, repeated, tag = "1")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGrantsRequest")]
#[proto_query(
    path = "/cosmos.authz.v1beta1.Query/Grants",
    response_type = QueryGrantsResponse
)]
pub struct QueryGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGrantsResponse")]
pub struct QueryGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGranterGrantsRequest")]
#[proto_query(
    path = "/cosmos.authz.v1beta1.Query/GranterGrants",
    response_type = QueryGranterGrantsResponse
)]
pub struct QueryGranterGrantsRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGranterGrantsResponse")]
pub struct QueryGranterGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGranteeGrantsRequest")]
#[proto_query(
    path = "/cosmos.authz.v1beta1.Query/GranteeGrants",
    response_type = QueryGranteeGrantsResponse
)]
pub struct QueryGranteeGrantsRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.QueryGranteeGrantsResponse")]
pub struct QueryGranteeGrantsResponse {
    #[prost(message, repeated, tag = "1")]
    pub grants: ::prost::alloc::vec::Vec<GrantAuthorization>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgGrant")]
pub struct MsgGrant {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub grant: ::core::option::Option<Grant>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgExecResponse")]
pub struct MsgExecResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgExec")]
pub struct MsgExec {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub msgs: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgGrantResponse")]
pub struct MsgGrantResponse {}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgRevoke")]
pub struct MsgRevoke {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/cosmos.authz.v1beta1.MsgRevokeResponse")]
pub struct MsgRevokeResponse {}
pub struct AuthzQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AuthzQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn grants(
        &self,
        granter: ::prost::alloc::string::String,
        grantee: ::prost::alloc::string::String,
        msg_type_url: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGrantsResponse, cosmwasm_std::StdError> {
        QueryGrantsRequest {
            granter,
            grantee,
            msg_type_url,
            pagination,
        }
        .query(self.querier)
    }
    pub fn granter_grants(
        &self,
        granter: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGranterGrantsResponse, cosmwasm_std::StdError> {
        QueryGranterGrantsRequest {
            granter,
            pagination,
        }
        .query(self.querier)
    }
    pub fn grantee_grants(
        &self,
        grantee: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGranteeGrantsResponse, cosmwasm_std::StdError> {
        QueryGranteeGrantsRequest {
            grantee,
            pagination,
        }
        .query(self.querier)
    }
}
