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
#[proto_message(type_url = "/provenance.msgfees.v1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "2")]
    pub floor_gas_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nhash_per_usd_mil: u64,
    #[prost(string, tag = "4")]
    pub conversion_fee_denom: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgFee")]
pub struct MsgFee {
    #[prost(string, tag = "1")]
    pub msg_type_url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub additional_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub recipient_basis_points: u32,
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
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryAllMsgFeesRequest")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/QueryAllMsgFees",
    response_type = QueryAllMsgFeesResponse
)]
pub struct QueryAllMsgFeesRequest {
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
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
#[proto_message(type_url = "/provenance.msgfees.v1.QueryAllMsgFeesResponse")]
pub struct QueryAllMsgFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub msg_fees: ::prost::alloc::vec::Vec<MsgFee>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/provenance.msgfees.v1.CalculateTxFeesRequest")]
#[proto_query(
    path = "/provenance.msgfees.v1.Query/CalculateTxFees",
    response_type = CalculateTxFeesResponse
)]
pub struct CalculateTxFeesRequest {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub tx_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub default_base_denom: ::prost::alloc::string::String,
    #[prost(float, tag = "3")]
    pub gas_adjustment: f32,
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
#[proto_message(type_url = "/provenance.msgfees.v1.CalculateTxFeesResponse")]
pub struct CalculateTxFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub additional_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub total_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub estimated_gas: u64,
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
#[proto_message(type_url = "/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest")]
pub struct MsgAssessCustomMsgFeeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub recipient_basis_points: ::prost::alloc::string::String,
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
