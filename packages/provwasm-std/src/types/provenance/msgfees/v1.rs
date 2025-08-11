use provwasm_proc_macro::CosmwasmExt;
/// AddMsgFeeProposal defines a governance proposal to add additional msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgAddMsgFeeProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
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
    pub nhash_per_usd_mil: u64,
}
/// UpdateConversionFeeDenomProposal defines a governance proposal to update the msg fee conversion denom
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by providing a MsgUpdateConversionFeeDenomProposalRequest in a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
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
/// MsgAssessCustomMsgFeeRequest defines an sdk.Msg type
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest")]
#[deprecated]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeResponse")]
pub struct MsgAssessCustomMsgFeeResponse {}
/// AddMsgFeeProposal defines a governance proposal to add additional msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAddMsgFeeProposalRequest")]
#[deprecated]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAddMsgFeeProposalResponse")]
pub struct MsgAddMsgFeeProposalResponse {}
/// UpdateMsgFeeProposal defines a governance proposal to update a current msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateMsgFeeProposalRequest")]
#[deprecated]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateMsgFeeProposalResponse")]
pub struct MsgUpdateMsgFeeProposalResponse {}
/// RemoveMsgFeeProposal defines a governance proposal to delete a current msg based fee
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgRemoveMsgFeeProposalRequest")]
#[deprecated]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgRemoveMsgFeeProposalResponse")]
pub struct MsgRemoveMsgFeeProposalResponse {}
/// UpdateNhashPerUsdMilProposal defines a governance proposal to update the nhash per usd mil param
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateNhashPerUsdMilProposalRequest")]
#[deprecated]
pub struct MsgUpdateNhashPerUsdMilProposalRequest {
    /// nhash_per_usd_mil is number of nhash per usd mil
    #[prost(uint64, tag = "1")]
    pub nhash_per_usd_mil: u64,
    /// the signing authority for the proposal
    ///
    ///
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgUpdateNhashPerUsdMilProposalResponse defines the Msg/UpdateNhashPerUsdMilProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateNhashPerUsdMilProposalResponse")]
pub struct MsgUpdateNhashPerUsdMilProposalResponse {}
/// UpdateConversionFeeDenomProposal defines a governance proposal to update the msg fee conversion denom
/// Deprecated: This message is no longer usable. It is only still included for
/// backwards compatibility (e.g. looking up old governance proposals).
/// It is replaced by functionality in the flatfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateConversionFeeDenomProposalRequest")]
#[deprecated]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgUpdateConversionFeeDenomProposalResponse")]
pub struct MsgUpdateConversionFeeDenomProposalResponse {}
pub struct MsgfeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MsgfeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
}
