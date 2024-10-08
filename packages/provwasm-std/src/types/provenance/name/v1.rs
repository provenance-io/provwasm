use provwasm_proc_macro::CosmwasmExt;
/// Params defines the set of params for the name module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.Params")]
pub struct Params {
    /// maximum length of name segment to allow
    #[prost(uint32, tag = "1")]
    pub max_segment_length: u32,
    /// minimum length of name segment to allow
    #[prost(uint32, tag = "2")]
    pub min_segment_length: u32,
    /// maximum number of name segments to allow.  Example: `foo.bar.baz` would be 3
    #[prost(uint32, tag = "3")]
    pub max_name_levels: u32,
    /// determines if unrestricted name keys are allowed or not
    #[prost(bool, tag = "4")]
    pub allow_unrestricted_names: bool,
}
/// NameRecord is a structure used to bind ownership of a name hierarchy to a collection of addresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.NameRecord")]
pub struct NameRecord {
    /// the bound name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the address the name resolved to
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// whether owner signature is required to add sub-names
    #[prost(bool, tag = "3")]
    pub restricted: bool,
}
/// CreateRootNameProposal details a proposal to create a new root name
/// that is controlled by a given owner and optionally restricted to the owner
/// for the sole creation of sub names.
/// Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
/// proposals, see MsgCreateRootNameRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.CreateRootNameProposal")]
#[deprecated]
pub struct CreateRootNameProposal {
    /// proposal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// proposal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the bound name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// the address the name will resolve to
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
    /// a flag that indicates if an owner signature is required to add sub-names
    #[prost(bool, tag = "5")]
    pub restricted: bool,
}
/// Event emitted when name is bound.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.EventNameBound")]
pub struct EventNameBound {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub restricted: bool,
}
/// Event emitted when name is unbound.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.EventNameUnbound")]
pub struct EventNameUnbound {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub restricted: bool,
}
/// Event emitted when name is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.EventNameUpdate")]
pub struct EventNameUpdate {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub restricted: bool,
}
/// EventNameParamsUpdated event emitted when name params are updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.EventNameParamsUpdated")]
pub struct EventNameParamsUpdated {
    #[prost(string, tag = "1")]
    pub allow_unrestricted_names: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_name_levels: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub min_segment_length: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub max_segment_length: ::prost::alloc::string::String,
}
/// GenesisState defines the name module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// bindings defines all the name records present at genesis
    #[prost(message, repeated, tag = "2")]
    pub bindings: ::prost::alloc::vec::Vec<NameRecord>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.name.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryResolveRequest is the request type for the Query/Resolve method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryResolveRequest")]
#[proto_query(
    path = "/provenance.name.v1.Query/Resolve",
    response_type = QueryResolveResponse
)]
pub struct QueryResolveRequest {
    /// name to resolve the address for
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryResolveResponse is the response type for the Query/Resolve method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryResolveResponse")]
pub struct QueryResolveResponse {
    /// a string containing the address the name resolves to
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Whether owner signature is required to add sub-names.
    #[prost(bool, tag = "2")]
    pub restricted: bool,
}
/// QueryReverseLookupRequest is the request type for the Query/ReverseLookup method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryReverseLookupRequest")]
#[proto_query(
    path = "/provenance.name.v1.Query/ReverseLookup",
    response_type = QueryReverseLookupResponse
)]
pub struct QueryReverseLookupRequest {
    /// address to find name records for
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryReverseLookupResponse is the response type for the Query/Resolve method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.QueryReverseLookupResponse")]
pub struct QueryReverseLookupResponse {
    /// an array of names bound against a given address
    #[prost(string, repeated, tag = "1")]
    pub name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgBindNameRequest defines an sdk.Msg type that is used to add an address/name binding under an optional parent name.
/// The record may optionally be restricted to prevent additional names from being added under this one without the
/// owner signing the request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgBindNameRequest")]
pub struct MsgBindNameRequest {
    /// The parent record to bind this name under.
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<NameRecord>,
    /// The name record to bind under the parent
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<NameRecord>,
}
/// MsgBindNameResponse defines the Msg/BindName response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgBindNameResponse")]
pub struct MsgBindNameResponse {}
/// MsgDeleteNameRequest defines an sdk.Msg type that is used to remove an existing address/name binding.  The binding
/// may not have any child names currently bound for this request to be successful. All associated attributes on account
/// addresses will be deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgDeleteNameRequest")]
pub struct MsgDeleteNameRequest {
    /// The record being removed
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<NameRecord>,
}
/// MsgDeleteNameResponse defines the Msg/DeleteName response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgDeleteNameResponse")]
pub struct MsgDeleteNameResponse {}
/// MsgCreateRootNameRequest defines an sdk.Msg type to create a new root name
/// that is controlled by a given owner and optionally restricted to the owner
/// for the sole creation of sub names.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgCreateRootNameRequest")]
pub struct MsgCreateRootNameRequest {
    /// The signing authority for the request
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// NameRecord is a structure used to bind ownership of a name hierarchy to a collection of addresses
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<NameRecord>,
}
/// MsgCreateRootNameResponse defines Msg/CreateRootName response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgCreateRootNameResponse")]
pub struct MsgCreateRootNameResponse {}
/// MsgModifyNameRequest defines a governance method that is used to update an existing address/name binding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgModifyNameRequest")]
pub struct MsgModifyNameRequest {
    /// The address signing the message
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The record being updated
    #[prost(message, optional, tag = "2")]
    pub record: ::core::option::Option<NameRecord>,
}
/// MsgModifyNameResponse defines the Msg/ModifyName response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgModifyNameResponse")]
pub struct MsgModifyNameResponse {}
/// MsgUpdateParamsRequest is a request message for the UpdateParams endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgUpdateParamsRequest")]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.name.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct NameQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> NameQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn resolve(
        &self,
        name: ::prost::alloc::string::String,
    ) -> Result<QueryResolveResponse, cosmwasm_std::StdError> {
        QueryResolveRequest { name }.query(self.querier)
    }
    pub fn reverse_lookup(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryReverseLookupResponse, cosmwasm_std::StdError> {
        QueryReverseLookupRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
}
