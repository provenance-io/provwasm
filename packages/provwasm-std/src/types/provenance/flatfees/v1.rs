use provwasm_proc_macro::CosmwasmExt;
/// EventParamsUpdated is emitted when UpdateParams successfully updates the flatfees params.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventParamsUpdated")]
pub struct EventParamsUpdated {}
/// EventConversionFactorUpdated is emitted when ConversionFactor successfully updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventConversionFactorUpdated")]
pub struct EventConversionFactorUpdated {
    /// definition_amount is the new definition_amount of the conversion factor as coin string, e.g. "1musd".
    #[prost(string, tag = "1")]
    pub definition_amount: ::prost::alloc::string::String,
    /// converted_amount is the new converted_amount of the conversion factor as a coin string, e.g. "2000nhash".
    #[prost(string, tag = "2")]
    pub converted_amount: ::prost::alloc::string::String,
}
/// EventMsgFeeSet is emitted when a msg fee is created or updated (from the SetMsgFee keeper helper).
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventMsgFeeSet")]
pub struct EventMsgFeeSet {
    /// msg_type_url is the type-url of the message that had its fee set.
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// EventMsgFeeUnset is emitted when a msg fee is removed (from the RemoveMsgFee keeper helper).
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventMsgFeeUnset")]
pub struct EventMsgFeeUnset {
    /// msg_type_url is the type-url of the message that had its fee removed.
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// EventOracleAddressAdded is emitted when AddOracleAddress successfully adds an oracle address.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventOracleAddressAdded")]
pub struct EventOracleAddressAdded {
    /// oracle_address is the address that was added to the oracle list.
    #[prost(string, tag = "1")]
    pub oracle_address: ::prost::alloc::string::String,
}
/// EventOracleAddressRemoved is emitted when RemoveOracleAddress successfully removes an oracle address.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.EventOracleAddressRemoved")]
pub struct EventOracleAddressRemoved {
    /// oracle_address is the address that was removed from the oracle list.
    #[prost(string, tag = "1")]
    pub oracle_address: ::prost::alloc::string::String,
}
/// Params defines the set of params for the flatfees module.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.Params")]
pub struct Params {
    /// default_cost is the amount a msg costs when there is no specific msg-fee defined for it.
    /// The denom used here should be the same as used to define the specific msg costs.
    /// The recommended denom is musd.
    #[prost(message, optional, tag = "1")]
    pub default_cost: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// conversion_factor is the ratio used to convert the msg-fees from their defined amounts into the fee denomination.
    /// The definition_amount should have the same denom as the default cost.
    /// The denom of the converted amount should be the denom that fees are paid in, e.g. nhash.
    #[prost(message, optional, tag = "3")]
    pub conversion_factor: ::core::option::Option<ConversionFactor>,
    /// oracle_addresses is the list of addresses allowed to update the conversion factor without governance.
    #[prost(string, repeated, tag = "4")]
    pub oracle_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConversionFactor equates the values of two coins in different denominations.
/// It is used to determine how much of the fee denomination is due.
/// actual cost = defined cost * converted_amount / definition_amount (truncated to an integer).
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.ConversionFactor")]
pub struct ConversionFactor {
    /// definition_amount is an amount (in the denomination used to define fees) that is equal to the converted_amount.
    /// This cannot have an amount of zero.
    /// If this has the same denomination as the converted_amount, then the amounts must also be equal.
    /// The denom of this field should be the same as the default cost, e.g. musd.
    #[prost(message, optional, tag = "1")]
    pub definition_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// converted_amount is an amount in the fee denomination equal to the definition_amount.
    /// If this is zero, all msgs will be free.
    /// If this has the same denomination as the definition_amount, then the amounts must also be equal.
    /// The denom of this field should be the fee denom, e.g. nhash.
    #[prost(message, optional, tag = "2")]
    pub converted_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFee defines the cost to use a specific message type.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgFee")]
pub struct MsgFee {
    /// msg_type_url is the type-url of the message, e.g. "/cosmos.bank.v1beta1.MsgSend".
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// cost is the Tx fee required for this msg_type_url.
    /// It should have the same denomination as the default cost and as the conversion factor's
    /// definition_amount, e.g. musdc. Any other denomination will be charged as defined.
    #[prost(message, repeated, tag = "2")]
    pub cost: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState contains a set of the flat fees module data, persisted from the store.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// msg_fees are the fees defined for specific msg types.
    #[prost(message, repeated, tag = "2")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
}
/// QueryParamsRequest is the request type for the Params query.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.flatfees.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Params query.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the x/flatfees module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllMsgFeesRequest is the request type for the AllMsgFees query.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryAllMsgFeesRequest")]
#[proto_query(
    path = "/provenance.flatfees.v1.Query/AllMsgFees",
    response_type = QueryAllMsgFeesResponse
)]
pub struct QueryAllMsgFeesRequest {
    /// do_not_convert, if true, will return the fees as defined (instead of as converted).
    #[prost(bool, tag = "1")]
    pub do_not_convert: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllMsgFeesResponse is the response type for the AllMsgFees query.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryAllMsgFeesResponse")]
pub struct QueryAllMsgFeesResponse {
    /// msg_fees contains the requested information.
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
    /// default_cost is the amount a msg costs when there is no specific msg-fee defined for it.
    #[prost(message, optional, tag = "2")]
    pub default_cost: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// pagination defines the pagination parameters of the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryMsgFeeRequest is the request type for the MsgFee query.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryMsgFeeRequest")]
#[proto_query(
    path = "/provenance.flatfees.v1.Query/MsgFee",
    response_type = QueryMsgFeeResponse
)]
pub struct QueryMsgFeeRequest {
    /// msg_type_url is the is the type-url of the message, e.g. "/cosmos.bank.v1beta1.MsgSend".
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// do_not_convert, if true, will return the fees as defined (instead of as converted).
    #[prost(bool, tag = "2")]
    pub do_not_convert: bool,
}
/// QueryMsgFeeResponse is the response type for the MsgFee query.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.QueryMsgFeeResponse")]
pub struct QueryMsgFeeResponse {
    /// msg_fee is the requested entry.
    #[prost(message, optional, tag = "1")]
    pub msg_fee: ::core::option::Option<MsgFee>,
}
/// MsgUpdateParamsRequest is the request for the UpdateParams governance endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateParamsRequest")]
pub struct MsgUpdateParamsRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params are the new param values to set.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse is the response for the UpdateParams governance endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgUpdateConversionFactorRequest is the request for the UpdateConversionFactor governance endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateConversionFactorRequest")]
pub struct MsgUpdateConversionFactorRequest {
    /// authority should be either the governance module account address or an oracle address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// conversion_factor is the new conversion factor that should be used.
    #[prost(message, optional, tag = "2")]
    pub conversion_factor: ::core::option::Option<ConversionFactor>,
}
/// MsgUpdateConversionFactorResponse is the response for the UpdateConversionFactor governance endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateConversionFactorResponse")]
pub struct MsgUpdateConversionFactorResponse {}
/// MsgUpdateMsgFeesRequest is the request for the UpdateMsgFees governance endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateMsgFeesRequest")]
pub struct MsgUpdateMsgFeesRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// to_set is the list of msg fees to create and/or update.
    #[prost(message, repeated, tag = "2")]
    pub to_set: ::prost::alloc::vec::Vec<MsgFee>,
    /// to_unset is the list of msg-type-urls that should have their msg fee entries
    /// deleted (they'll go back to using the default cost).
    #[prost(string, repeated, tag = "3")]
    pub to_unset: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgUpdateMsgFeesResponse is the response for the UpdateMsgFees governance endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgUpdateMsgFeesResponse")]
pub struct MsgUpdateMsgFeesResponse {}
/// MsgAddOracleAddressRequest is the request for the AddOracleAddress governance endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgAddOracleAddressRequest")]
pub struct MsgAddOracleAddressRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// oracle_address is the address to be added to the oracle list.
    #[prost(string, tag = "2")]
    pub oracle_address: ::prost::alloc::string::String,
}
/// MsgAddOracleAddressResponse is the response for the AddOracleAddress governance endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgAddOracleAddressResponse")]
pub struct MsgAddOracleAddressResponse {}
/// MsgRemoveOracleAddressRequest is the request for the RemoveOracleAddress governance endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgRemoveOracleAddressRequest")]
pub struct MsgRemoveOracleAddressRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// oracle_address is the address to be removed from the oracle list.
    #[prost(string, tag = "2")]
    pub oracle_address: ::prost::alloc::string::String,
}
/// MsgRemoveOracleAddressResponse is the response for the RemoveOracleAddress governance endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.flatfees.v1.MsgRemoveOracleAddressResponse")]
pub struct MsgRemoveOracleAddressResponse {}
pub struct FlatfeesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FlatfeesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn all_msg_fees(
        &self,
        do_not_convert: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllMsgFeesResponse, cosmwasm_std::StdError> {
        QueryAllMsgFeesRequest {
            do_not_convert,
            pagination,
        }
        .query(self.querier)
    }
    pub fn msg_fee(
        &self,
        msg_type_url: ::prost::alloc::string::String,
        do_not_convert: bool,
    ) -> Result<QueryMsgFeeResponse, cosmwasm_std::StdError> {
        QueryMsgFeeRequest {
            msg_type_url,
            do_not_convert,
        }
        .query(self.querier)
    }
}
