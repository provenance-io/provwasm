use provwasm_std_derive::CosmwasmExt;
/// EventOptIn is an event emitted when an address opts into quarantine.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.EventOptIn")]
pub struct EventOptIn {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
}
/// EventOptOut is an event emitted when an address opts out of quarantine.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.EventOptOut")]
pub struct EventOptOut {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
}
/// EventFundsQuarantined is an event emitted when funds are quarantined.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.EventFundsQuarantined")]
pub struct EventFundsQuarantined {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// EventFundsReleased is an event emitted when quarantined funds are accepted and released.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.EventFundsReleased")]
pub struct EventFundsReleased {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// QuarantinedFunds defines structure that represents coins that have been quarantined.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QuarantinedFunds")]
pub struct QuarantinedFunds {
    /// to_address is the intended recipient of the coins that have been quarantined.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// unaccepted_from_addresses are the senders that have not been part of an accept yet for these coins.
    #[prost(string, repeated, tag = "2")]
    pub unaccepted_from_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// coins is the amount currently in quarantined for the two addresses.
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// declined is true if these funds were previously declined.
    #[prost(bool, tag = "4")]
    pub declined: bool,
}
/// AutoResponseEntry defines the auto response to one address from another.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.AutoResponseEntry")]
pub struct AutoResponseEntry {
    /// to_address is the receiving address.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// from_address is the sending address.
    #[prost(string, tag = "2")]
    pub from_address: ::prost::alloc::string::String,
    /// response is the auto-response setting for these two addresses.
    #[prost(enumeration = "AutoResponse", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub response: i32,
}
/// AutoResponseUpdate defines a quarantine auto response update that should be applied.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.AutoResponseUpdate")]
pub struct AutoResponseUpdate {
    /// from_address is the address that funds would be coming from.
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    /// response is the automatic action to take on funds sent from from_address.
    /// Provide AUTO_RESPONSE_UNSPECIFIED to turn off an auto-response.
    #[prost(enumeration = "AutoResponse", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub response: i32,
}
/// QuarantineRecord defines information regarding quarantined funds that is stored in state.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QuarantineRecord")]
pub struct QuarantineRecord {
    /// unaccepted_from_addresses are the senders that have not been part of an accept yet for these coins.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub unaccepted_from_addresses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// accepted_from_addresses are the senders that have already been part of an accept for these coins.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub accepted_from_addresses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// coins is the amount that has been quarantined.
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// declined is whether these funds have been declined.
    #[prost(bool, tag = "4")]
    pub declined: bool,
}
/// QuarantineRecordSuffixIndex defines a list of record suffixes that can be stored in state and used as an index.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QuarantineRecordSuffixIndex")]
pub struct QuarantineRecordSuffixIndex {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub record_suffixes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// AutoResponse enumerates the quarantine auto-response options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AutoResponse {
    /// AUTO_RESPONSE_UNSPECIFIED defines that an automatic response has not been specified.
    /// This means that no automatic action should be taken, i.e. this auto-response is off,
    /// and default quarantine behavior is used.
    Unspecified = 0,
    /// AUTO_RESPONSE_ACCEPT defines that sends should be automatically accepted, bypassing quarantine.
    Accept = 1,
    /// AUTO_RESPONSE_DECLINE defines that sends should be automatically declined.
    Decline = 2,
}
impl AutoResponse {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutoResponse::Unspecified => "AUTO_RESPONSE_UNSPECIFIED",
            AutoResponse::Accept => "AUTO_RESPONSE_ACCEPT",
            AutoResponse::Decline => "AUTO_RESPONSE_DECLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_RESPONSE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTO_RESPONSE_ACCEPT" => Some(Self::Accept),
            "AUTO_RESPONSE_DECLINE" => Some(Self::Decline),
            _ => None,
        }
    }
}
/// GenesisState defines the quarantine module's genesis state.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.GenesisState")]
pub struct GenesisState {
    /// quarantined_addresses defines account addresses that are opted into quarantine.
    #[prost(string, repeated, tag = "1")]
    pub quarantined_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// auto_responses defines the quarantine auto-responses for addresses.
    #[prost(message, repeated, tag = "2")]
    pub auto_responses: ::prost::alloc::vec::Vec<AutoResponseEntry>,
    /// quarantined_funds defines funds that are quarantined.
    #[prost(message, repeated, tag = "3")]
    pub quarantined_funds: ::prost::alloc::vec::Vec<QuarantinedFunds>,
}
/// QueryIsQuarantinedRequest defines the RPC request for checking if an account has opted into quarantine.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryIsQuarantinedRequest")]
#[proto_query(
    path = "/cosmos.quarantine.v1beta1.Query/IsQuarantined",
    response_type = QueryIsQuarantinedResponse
)]
pub struct QueryIsQuarantinedRequest {
    /// to_address is the address to check.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
}
/// QueryIsQuarantinedResponse defines the RPC response of an IsQuarantined query.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryIsQuarantinedResponse")]
pub struct QueryIsQuarantinedResponse {
    /// is_quarantined is true if the to_address has opted into quarantine.
    #[prost(bool, tag = "1")]
    pub is_quarantined: bool,
}
/// QueryQuarantinedFundsRequest defines the RPC request for looking up quarantined funds.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryQuarantinedFundsRequest")]
#[proto_query(
    path = "/cosmos.quarantine.v1beta1.Query/QuarantinedFunds",
    response_type = QueryQuarantinedFundsResponse
)]
pub struct QueryQuarantinedFundsRequest {
    /// to_address is the intended recipient of the coins that have been quarantined.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// from_address is the sender of the coins. If provided, a to_address must also be provided.
    #[prost(string, tag = "2")]
    pub from_address: ::prost::alloc::string::String,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryQuarantinedFundsResponse defines the RPC response of a QuarantinedFunds query.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryQuarantinedFundsResponse")]
pub struct QueryQuarantinedFundsResponse {
    /// quarantinedFunds is info about coins sitting in quarantine.
    #[prost(message, repeated, tag = "1")]
    pub quarantined_funds: ::prost::alloc::vec::Vec<QuarantinedFunds>,
    /// pagination defines the pagination parameters of the response.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryAutoResponsesRequest defines the RPC request for getting auto-response settings for an address.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryAutoResponsesRequest")]
#[proto_query(
    path = "/cosmos.quarantine.v1beta1.Query/AutoResponses",
    response_type = QueryAutoResponsesResponse
)]
pub struct QueryAutoResponsesRequest {
    /// to_address is the quarantined account to get info on.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// from_address is an optional sender address to limit results.
    #[prost(string, tag = "2")]
    pub from_address: ::prost::alloc::string::String,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAutoResponsesResponse defines the RPC response of a AutoResponses query.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.QueryAutoResponsesResponse")]
pub struct QueryAutoResponsesResponse {
    /// auto_responses are the auto-response entries from the provided query.
    #[prost(message, repeated, tag = "1")]
    pub auto_responses: ::prost::alloc::vec::Vec<AutoResponseEntry>,
    /// pagination defines the pagination parameters of the response.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgOptIn represents a message for opting in to account quarantine.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgOptIn")]
pub struct MsgOptIn {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
}
/// MsgOptInResponse defines the Msg/OptIn response type.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgOptInResponse")]
pub struct MsgOptInResponse {}
/// MsgOptOut represents a message for opting in to account quarantine.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgOptOut")]
pub struct MsgOptOut {
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
}
/// MsgOptOutResponse defines the Msg/OptOut response type.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgOptOutResponse")]
pub struct MsgOptOutResponse {}
/// MsgAccept represents a message for accepting quarantined funds.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgAccept")]
pub struct MsgAccept {
    /// to_address is the address of the quarantined account that is accepting funds.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// from_addresses is one or more addresses that have sent funds to the quarantined account.
    /// All funds quarantined for to_address from any from_addresses are marked as accepted and released if appropriate.
    /// At least one is required.
    #[prost(string, repeated, tag = "2")]
    pub from_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// permanent, if true, sets up auto-accept for the to_address from each from_address.
    /// If false (default), only the currently quarantined funds will be accepted.
    #[prost(bool, tag = "3")]
    pub permanent: bool,
}
/// MsgAcceptResponse defines the Msg/Accept response type.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgAcceptResponse")]
pub struct MsgAcceptResponse {
    /// funds_released is the amount that was quarantined but has now been released and sent to the requester.
    #[prost(message, repeated, tag = "1")]
    pub funds_released: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgDecline represents a message for declining quarantined funds.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgDecline")]
pub struct MsgDecline {
    /// to_address is the address of the quarantined account that is accepting funds.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// from_addresses is one or more addresses that have sent funds to the quarantined account.
    /// All funds quarantined for to_address from any from_addresses are marked as declined.
    /// At least one is required.
    #[prost(string, repeated, tag = "2")]
    pub from_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// permanent, if true, sets up auto-decline for the to_address from each from_address.
    /// If false (default), only the currently quarantined funds will be declined.
    #[prost(bool, tag = "3")]
    pub permanent: bool,
}
/// MsgDeclineResponse defines the Msg/Decline response type.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgDeclineResponse")]
pub struct MsgDeclineResponse {}
/// MsgUpdateAutoResponses represents a message for updating quarantine auto-responses for a receiving address.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgUpdateAutoResponses")]
pub struct MsgUpdateAutoResponses {
    /// to_address is the quarantined address that would be accepting or declining funds.
    #[prost(string, tag = "1")]
    pub to_address: ::prost::alloc::string::String,
    /// updates is the list of addresses and auto-responses that should be updated for the to_address.
    #[prost(message, repeated, tag = "2")]
    pub updates: ::prost::alloc::vec::Vec<AutoResponseUpdate>,
}
/// MsgUpdateAutoResponsesResponse defines the Msg/UpdateAutoResponse response type.
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
#[proto_message(type_url = "/cosmos.quarantine.v1beta1.MsgUpdateAutoResponsesResponse")]
pub struct MsgUpdateAutoResponsesResponse {}
pub struct QuarantineQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> QuarantineQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn is_quarantined(
        &self,
        to_address: ::prost::alloc::string::String,
    ) -> Result<QueryIsQuarantinedResponse, cosmwasm_std::StdError> {
        QueryIsQuarantinedRequest { to_address }.query(self.querier)
    }
    pub fn quarantined_funds(
        &self,
        to_address: ::prost::alloc::string::String,
        from_address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryQuarantinedFundsResponse, cosmwasm_std::StdError> {
        QueryQuarantinedFundsRequest {
            to_address,
            from_address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn auto_responses(
        &self,
        to_address: ::prost::alloc::string::String,
        from_address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAutoResponsesResponse, cosmwasm_std::StdError> {
        QueryAutoResponsesRequest {
            to_address,
            from_address,
            pagination,
        }
        .query(self.querier)
    }
}
