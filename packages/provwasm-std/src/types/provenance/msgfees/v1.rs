use provwasm_std_derive::CosmwasmExt;
/// Params defines the set of params for the msgfees module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.Params")]
pub struct Params {
    /// constant used to calculate fees when gas fees shares denom with msg fee
    #[prost(message, optional, tag = "2")]
    pub floor_gas_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// total nhash per usd mil for converting usd to nhash
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
    /// conversion fee denom is the denom usd is converted to
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
}
/// MsgFee is the core of what gets stored on the blockchain
/// it consists of four parts
/// 1. the msg type url, i.e. /cosmos.bank.v1beta1.MsgSend
/// 2. minimum additional fees(can be of any denom)
/// 3. optional recipient of fee based on `recipient_basis_points`
/// 4. if recipient is declared they will recieve the basis points of the fee (0-10,000)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgFee")]
pub struct MsgFee {
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// additional_fee can pay in any Coin( basically a Denom and Amount, Amount can be zero)
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// optional recipient address, the amount is split between recipient and fee module
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// optional split of funds between the recipient and fee module defaults to 50:50,
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub recipient_basis_points: u32,
}
/// EventMsgFee final event property for msg fee on type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.AddMsgFeeProposal")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateMsgFeeProposal")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.RemoveMsgFeeProposal")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateNhashPerUsdMilProposal")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.UpdateConversionFeeDenomProposal")]
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
/// CalculateTxFeesRequest is the request type for the Query RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.CalculateTxFeesRequest")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/CalculateTxFees",
    response_type = CalculateTxFeesResponse
)]
pub struct CalculateTxFeesRequest {
    /// tx_bytes is the transaction to simulate.
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    /// default_base_denom is used to set the denom used for gas fees
    /// if not set it will default to nhash.
    #[prost(string, tag = "2")]
    pub default_base_denom: ::prost::alloc::string::String,
    /// gas_adjustment is the adjustment factor to be multiplied against the estimate returned by the tx simulation
    #[prost(float, tag = "3")]
    pub gas_adjustment: f32,
}
/// CalculateTxFeesResponse is the response type for the Query RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.CalculateTxFeesResponse")]
pub struct CalculateTxFeesResponse {
    /// additional_fees are the amount of coins to be for addition msg fees
    #[prost(message, repeated, tag = "1")]
    pub additional_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// total_fees are the total amount of fees needed for the transactions (msg fees + gas fee)
    /// note: the gas fee is calculated with the floor gas price module param.
    #[prost(message, repeated, tag = "2")]
    pub total_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// estimated_gas is the amount of gas needed for the transaction
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub estimated_gas: u64,
}
/// MsgAssessCustomMsgFeeRequest defines an sdk.Msg type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
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
    /// optional recipient address, the amount is split 50/50 between recipient and fee module. If
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// empty, whole amount goes to fee module
    ///
    /// the signer of the msg
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
}
/// MsgAssessCustomMsgFeeResponse defines the Msg/AssessCustomMsgFeee response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeResponse")]
pub struct MsgAssessCustomMsgFeeResponse {}
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
    pub fn calculate_tx_fees(
        &self,
        tx_bytes: ::prost::alloc::vec::Vec<u8>,
        default_base_denom: ::prost::alloc::string::String,
        gas_adjustment: f32,
    ) -> Result<CalculateTxFeesResponse, cosmwasm_std::StdError> {
        CalculateTxFeesRequest {
            tx_bytes,
            default_base_denom,
            gas_adjustment,
        }
        .query(self.querier)
    }
}
