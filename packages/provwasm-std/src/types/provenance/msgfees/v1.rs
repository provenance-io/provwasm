use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// Params defines the set of params for the msgfees module.
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
#[proto_message(type_url = "/provenance.msgfees.v1.Params")]
pub struct Params {
    /// floor_gas_price is the constant used to calculate fees when gas fees shares denom with msg fee.
    ///
    /// Conversions:
    ///    - x nhash/usd-mil = 1,000,000/x usd/hash
    ///    - y usd/hash = 1,000,000/y nhash/usd-mil
    ///
    /// Examples:
    ///    - 40,000,000 nhash/usd-mil = 1,000,000/40,000,000 usd/hash = $0.025/hash,
    ///    - $0.040/hash = 1,000,000/0.040 nhash/usd-mil = 25,000,000 nhash/usd-mil
    #[prost(message, optional, tag = "2")]
    pub floor_gas_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// nhash_per_usd_mil is the total nhash per usd mil for converting usd to nhash.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
    /// conversion_fee_denom is the denom usd is converted to.
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
}
/// MsgFee is the core of what gets stored on the blockchain to define a msg-based fee.
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgFee")]
pub struct MsgFee {
    /// msg_type_url is the type-url of the message with the added fee, e.g. "/cosmos.bank.v1beta1.MsgSend".
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional_fee is the extra fee that is required for the given message type (can be in any denom).
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// recipient is an option address that will receive a portion of the additional fee.
    /// There can only be a recipient if the recipient_basis_points is not zero.
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// recipient_basis_points is an optional portion of the additional fee to be sent to the recipient.
    /// Must be between 0 and 10,000 (inclusive).
    ///
    /// If there is a recipient, this must not be zero. If there is not a recipient, this must be zero.
    ///
    /// The recipient will receive additional_fee * recipient_basis_points / 10,000.
    /// The fee collector will receive the rest, i.e. additional_fee * (10,000 - recipient_basis_points) / 10,000.
    #[prost(uint32, tag = "4")]
    pub recipient_basis_points: u32,
}
/// EventMsgFee final event property for msg fee on type
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
#[proto_message(type_url = "/provenance.msgfees.v1.EventMsgFee")]
pub struct EventMsgFee {
    #[prost(string, tag = "1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub count: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
}
/// EventMsgFees event emitted with summary of msg fees
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
#[proto_message(type_url = "/provenance.msgfees.v1.EventMsgFees")]
pub struct EventMsgFees {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<EventMsgFee>,
}
/// GenesisState contains a set of msg fees, persisted from the store
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
#[proto_message(type_url = "/provenance.msgfees.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// msg_based_fees are the additional fees on specific tx msgs
    #[prost(message, repeated, tag = "2")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
}
/// AddMsgFeeProposal defines a governance proposal to add additional msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgAddMsgFeeProposalRequest in a governance proposal.
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
#[proto_message(type_url = "/provenance.msgfees.v1.AddMsgFeeProposal")]
#[deprecated]
pub struct AddMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg to add fee
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "4")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient to recieve basis points
    #[prost(string, tag = "5")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "6")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// UpdateMsgFeeProposal defines a governance proposal to update a current msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgUpdateMsgFeeProposalRequest in a governance proposal.
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
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateMsgFeeProposal")]
#[deprecated]
pub struct UpdateMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg to update fee
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "4")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient to recieve basis points
    #[prost(string, tag = "5")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "6")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// RemoveMsgFeeProposal defines a governance proposal to delete a current msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgRemoveMsgFeeProposalRequest in a governance proposal.
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
#[proto_message(type_url = "/provenance.msgfees.v1.RemoveMsgFeeProposal")]
#[deprecated]
pub struct RemoveMsgFeeProposal {
    /// propsal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// propsal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// type url of msg fee to remove
    #[prost(string, tag = "3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// UpdateNhashPerUsdMilProposal defines a governance proposal to update the nhash per usd mil param
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgUpdateNhashPerUsdMilProposalRequest in a governance proposal.
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
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateNhashPerUsdMilProposal")]
#[deprecated]
pub struct UpdateNhashPerUsdMilProposal {
    /// proposal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// proposal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// nhash_per_usd_mil is number of nhash per usd mil
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
}
/// UpdateConversionFeeDenomProposal defines a governance proposal to update the msg fee conversion denom
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgUpdateConversionFeeDenomProposalRequest in a governance proposal.
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
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateConversionFeeDenomProposal")]
#[deprecated]
pub struct UpdateConversionFeeDenomProposal {
    /// proposal title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// proposal description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// conversion_fee_denom is the denom that usd will be converted to
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllMsgFeesRequest queries all Msg which have fees associated with them.
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryAllMsgFeesRequest")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/QueryAllMsgFees",
    response_type = QueryAllMsgFeesResponse
)]
pub struct QueryAllMsgFeesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// response for querying all msg's with fees associated with them
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryAllMsgFeesResponse")]
pub struct QueryAllMsgFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgAssessCustomMsgFeeRequest defines an sdk.Msg type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest")]
pub struct MsgAssessCustomMsgFeeRequest {
    /// optional short name for custom msg fee, this will be emitted as a property of the event
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// amount of additional fee that must be paid
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient address, the basis points amount is sent to the recipient
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// the signer of the msg
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    /// optional basis points 0 - 10,000 for recipient defaults to 10,000
    #[prost(string, tag = "5")]
    pub recipient_basis_points: ::prost::alloc::string::String,
}
/// MsgAssessCustomMsgFeeResponse defines the Msg/AssessCustomMsgFeee response type.
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeResponse")]
pub struct MsgAssessCustomMsgFeeResponse {}
/// AddMsgFeeProposal defines a governance proposal to add additional msg based fee
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAddMsgFeeProposalRequest")]
pub struct MsgAddMsgFeeProposalRequest {
    /// type url of msg to add fee
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient to receive basis points
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "4")]
    pub recipient_basis_points: ::prost::alloc::string::String,
    /// the signing authority for the proposal
    #[prost(string, tag = "5")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgAddMsgFeeProposalResponse defines the Msg/AddMsgFeeProposal response type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAddMsgFeeProposalResponse")]
pub struct MsgAddMsgFeeProposalResponse {}
/// UpdateMsgFeeProposal defines a governance proposal to update a current msg based fee
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateMsgFeeProposalRequest")]
pub struct MsgUpdateMsgFeeProposalRequest {
    /// type url of msg to update fee
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional fee for msg type
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient to receive basis points
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// basis points to use when recipient is present (1 - 10,000)
    #[prost(string, tag = "4")]
    pub recipient_basis_points: ::prost::alloc::string::String,
    /// the signing authority for the proposal
    #[prost(string, tag = "5")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateMsgFeeProposalResponse defines the Msg/RemoveMsgFeeProposal response type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateMsgFeeProposalResponse")]
pub struct MsgUpdateMsgFeeProposalResponse {}
/// RemoveMsgFeeProposal defines a governance proposal to delete a current msg based fee
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgRemoveMsgFeeProposalRequest")]
pub struct MsgRemoveMsgFeeProposalRequest {
    /// type url of msg fee to remove
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// the signing authority for the proposal
    ///
    ///
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgRemoveMsgFeeProposalResponse defines the Msg/RemoveMsgFeeProposal response type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgRemoveMsgFeeProposalResponse")]
pub struct MsgRemoveMsgFeeProposalResponse {}
/// UpdateNhashPerUsdMilProposal defines a governance proposal to update the nhash per usd mil param
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateNhashPerUsdMilProposalRequest")]
pub struct MsgUpdateNhashPerUsdMilProposalRequest {
    /// nhash_per_usd_mil is number of nhash per usd mil
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
    /// the signing authority for the proposal
    ///
    ///
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateNhashPerUsdMilProposalResponse defines the Msg/UpdateNhashPerUsdMilProposal response type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateNhashPerUsdMilProposalResponse")]
pub struct MsgUpdateNhashPerUsdMilProposalResponse {}
/// UpdateConversionFeeDenomProposal defines a governance proposal to update the msg fee conversion denom
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateConversionFeeDenomProposalRequest")]
pub struct MsgUpdateConversionFeeDenomProposalRequest {
    /// conversion_fee_denom is the denom that usd will be converted to
    #[prost(string, tag = "1")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
    /// the signing authority for the proposal
    ///
    ///
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateConversionFeeDenomProposalResponse defines the Msg/UpdateConversionFeeDenomProposal response type
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateConversionFeeDenomProposalResponse")]
pub struct MsgUpdateConversionFeeDenomProposalResponse {}
pub struct MsgfeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MsgfeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn query_all_msg_fees(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllMsgFeesResponse, cosmwasm_std::StdError> {
        QueryAllMsgFeesRequest { pagination }.query(self.querier)
    }
}
