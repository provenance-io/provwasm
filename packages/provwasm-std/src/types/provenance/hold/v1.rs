use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// EventHoldAdded is an event indicating that some funds were placed on hold in an account.
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
#[proto_message(type_url = "/provenance.hold.v1.EventHoldAdded")]
pub struct EventHoldAdded {
    /// address is the bech32 address string of the account with the funds.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// amount is a Coins string of the funds placed on hold.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// reason is a human-readable indicator of why this hold was added.
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
}
/// EventHoldReleased is an event indicating that some funds were released from hold for an account.
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
#[proto_message(type_url = "/provenance.hold.v1.EventHoldReleased")]
pub struct EventHoldReleased {
    /// address is the bech32 address string of the account with the funds.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// amount is a Coins string of the funds released from hold.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// AccountHold associates an address with an amount on hold for that address.
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
#[proto_message(type_url = "/provenance.hold.v1.AccountHold")]
pub struct AccountHold {
    /// address is the account address that holds the funds on hold.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// amount is the balances that are on hold for the address.
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the attribute module's genesis state.
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
#[proto_message(type_url = "/provenance.hold.v1.GenesisState")]
pub struct GenesisState {
    /// holds defines the funds on hold at genesis.
    #[prost(message, repeated, tag = "1")]
    pub holds: ::prost::alloc::vec::Vec<AccountHold>,
}
/// GetHoldsRequest is the request type for the Query/GetHolds query.
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
#[proto_message(type_url = "/provenance.hold.v1.GetHoldsRequest")]
#[proto_query(
    path = "/provenance.hold.v1.Query/GetHolds",
    response_type = GetHoldsResponse
)]
pub struct GetHoldsRequest {
    /// address is the account address to get on-hold balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// GetHoldsResponse is the response type for the Query/GetHolds query.
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
#[proto_message(type_url = "/provenance.hold.v1.GetHoldsResponse")]
pub struct GetHoldsResponse {
    /// amount is the total on hold for the requested address.
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GetAllHoldsRequest is the request type for the Query/GetAllHolds query.
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
#[proto_message(type_url = "/provenance.hold.v1.GetAllHoldsRequest")]
#[proto_query(
    path = "/provenance.hold.v1.Query/GetAllHolds",
    response_type = GetAllHoldsResponse
)]
pub struct GetAllHoldsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// GetAllHoldsResponse is the response type for the Query/GetAllHolds query.
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
#[proto_message(type_url = "/provenance.hold.v1.GetAllHoldsResponse")]
pub struct GetAllHoldsResponse {
    /// holds is a list of addresses with funds on hold and the amounts being held.
    #[prost(message, repeated, tag = "1")]
    pub holds: ::prost::alloc::vec::Vec<AccountHold>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
pub struct HoldQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> HoldQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_holds(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<GetHoldsResponse, cosmwasm_std::StdError> {
        GetHoldsRequest { address }.query(self.querier)
    }
    pub fn get_all_holds(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<GetAllHoldsResponse, cosmwasm_std::StdError> {
        GetAllHoldsRequest { pagination }.query(self.querier)
    }
}
