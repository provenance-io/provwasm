use provwasm_std_derive::CosmwasmExt;
/// EventAddressSanctioned is an event emitted when an address is sanctioned.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.EventAddressSanctioned")]
pub struct EventAddressSanctioned {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventAddressUnsanctioned is an event emitted when an address is unsanctioned.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.EventAddressUnsanctioned")]
pub struct EventAddressUnsanctioned {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventTempAddressSanctioned is an event emitted when an address is temporarily sanctioned.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.EventTempAddressSanctioned")]
pub struct EventTempAddressSanctioned {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventTempAddressUnsanctioned is an event emitted when an address is temporarily unsanctioned.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.EventTempAddressUnsanctioned")]
pub struct EventTempAddressUnsanctioned {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventParamsUpdated is an event emitted when the sanction module params are updated.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.EventParamsUpdated")]
pub struct EventParamsUpdated {}
/// Params defines the configurable parameters of the sanction module.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.Params")]
pub struct Params {
    /// immediate_sanction_min_deposit is the minimum deposit for a sanction to happen immediately.
    /// If this is zero, immediate sanctioning is not available.
    /// Otherwise, if a sanction governance proposal is issued with a deposit at least this large, a temporary sanction
    /// will be immediately issued that will expire when voting ends on the governance proposal.
    #[prost(message, repeated, tag = "1")]
    pub immediate_sanction_min_deposit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// immediate_unsanction_min_deposit is the minimum deposit for an unsanction to happen immediately.
    /// If this is zero, immediate unsanctioning is not available.
    /// Otherwise, if an unsanction governance proposal is issued with a deposit at least this large, a temporary
    /// unsanction will be immediately issued that will expire when voting ends on the governance proposal.
    #[prost(message, repeated, tag = "2")]
    pub immediate_unsanction_min_deposit:
        ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// TemporaryEntry defines the information involved in a temporary sanction or unsanction.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.TemporaryEntry")]
pub struct TemporaryEntry {
    /// address is the address of this temporary entry.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// proposal_id is the governance proposal id associated with this temporary entry.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// status is whether the entry is a sanction or unsanction.
    #[prost(enumeration = "TempStatus", tag = "3")]
    pub status: i32,
}
/// TempStatus is whether a temporary entry is a sanction or unsanction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum TempStatus {
    /// TEMP_STATUS_UNSPECIFIED represents and unspecified status value.
    Unspecified = 0,
    /// TEMP_STATUS_SANCTIONED indicates a sanction is in place.
    Sanctioned = 1,
    /// TEMP_STATUS_UNSANCTIONED indicates an unsanctioned is in place.
    Unsanctioned = 2,
}
impl TempStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TempStatus::Unspecified => "TEMP_STATUS_UNSPECIFIED",
            TempStatus::Sanctioned => "TEMP_STATUS_SANCTIONED",
            TempStatus::Unsanctioned => "TEMP_STATUS_UNSANCTIONED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEMP_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "TEMP_STATUS_SANCTIONED" => Some(Self::Sanctioned),
            "TEMP_STATUS_UNSANCTIONED" => Some(Self::Unsanctioned),
            _ => None,
        }
    }
}
/// GenesisState defines the sanction module's genesis state.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params are the sanction module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// sanctioned_addresses defines account addresses that are sanctioned.
    #[prost(string, repeated, tag = "2")]
    pub sanctioned_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// temporary_entries defines the temporary entries associated with on-going governance proposals.
    #[prost(message, repeated, tag = "3")]
    pub temporary_entries: ::prost::alloc::vec::Vec<TemporaryEntry>,
}
/// QueryIsSanctionedRequest defines the RPC request for checking if an account is sanctioned.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryIsSanctionedRequest")]
#[proto_query(
    path = "/cosmos.sanction.v1beta1.Query/IsSanctioned",
    response_type = QueryIsSanctionedResponse
)]
pub struct QueryIsSanctionedRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryIsSanctionedResponse defines the RPC response of an IsSanctioned query.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryIsSanctionedResponse")]
pub struct QueryIsSanctionedResponse {
    /// is_sanctioned is true if the address is sanctioned.
    #[prost(bool, tag = "1")]
    pub is_sanctioned: bool,
}
/// QuerySanctionedAddressesRequest defines the RPC request for listing sanctioned accounts.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QuerySanctionedAddressesRequest")]
#[proto_query(
    path = "/cosmos.sanction.v1beta1.Query/SanctionedAddresses",
    response_type = QuerySanctionedAddressesResponse
)]
pub struct QuerySanctionedAddressesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySanctionedAddressesResponse defines the RPC response of a SanctionedAddresses query.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QuerySanctionedAddressesResponse")]
pub struct QuerySanctionedAddressesResponse {
    /// addresses is the list of sanctioned account addresses.
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryTemporaryEntriesRequest defines the RPC request for listing temporary sanction/unsanction entries.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryTemporaryEntriesRequest")]
#[proto_query(
    path = "/cosmos.sanction.v1beta1.Query/TemporaryEntries",
    response_type = QueryTemporaryEntriesResponse
)]
pub struct QueryTemporaryEntriesRequest {
    /// address is an optional address to restrict results to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryTemporaryEntriesResponse defines the RPC response of a TemporaryEntries query.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryTemporaryEntriesResponse")]
pub struct QueryTemporaryEntriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<TemporaryEntry>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryParamsRequest defines the RPC request for getting the sanction module params.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.sanction.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the RPC response of a Params query.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params are the sanction module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgSanction represents a message for the governance operation of sanctioning addresses.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgSanction")]
pub struct MsgSanction {
    /// addresses are the addresses to sanction.
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// authority is the address of the account with the authority to enact sanctions (most likely the governance module
    /// account).
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgOptInResponse defines the Msg/Sanction response type.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgSanctionResponse")]
pub struct MsgSanctionResponse {}
/// MsgSanction represents a message for the governance operation of unsanctioning addresses.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgUnsanction")]
pub struct MsgUnsanction {
    /// addresses are the addresses to unsanction.
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// authority is the address of the account with the authority to retract sanctions (most likely the governance module
    /// account).
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgOptInResponse defines the Msg/Unsanction response type.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgUnsanctionResponse")]
pub struct MsgUnsanctionResponse {}
/// MsgUpdateParams represents a message for the governance operation of updating the sanction module params.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// params are the sanction module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// authority is the address of the account with the authority to update params (most likely the governance module
    /// account).
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateParamsResponse defined the Msg/UpdateParams response type.
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
#[proto_message(type_url = "/cosmos.sanction.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct SanctionQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SanctionQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn is_sanctioned(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryIsSanctionedResponse, cosmwasm_std::StdError> {
        QueryIsSanctionedRequest { address }.query(self.querier)
    }
    pub fn sanctioned_addresses(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QuerySanctionedAddressesResponse, cosmwasm_std::StdError> {
        QuerySanctionedAddressesRequest { pagination }.query(self.querier)
    }
    pub fn temporary_entries(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryTemporaryEntriesResponse, cosmwasm_std::StdError> {
        QueryTemporaryEntriesRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
