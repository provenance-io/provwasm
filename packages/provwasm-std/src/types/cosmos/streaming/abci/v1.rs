use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// ListenBeginBlockRequest is the request type for the ListenBeginBlock RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenBeginBlockRequest")]
pub struct ListenBeginBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub req:
        ::core::option::Option<super::super::super::super::tendermint::abci::RequestBeginBlock>,
    #[prost(message, optional, tag = "2")]
    pub res:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseBeginBlock>,
}
/// ListenBeginBlockResponse is the response type for the ListenBeginBlock RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenBeginBlockResponse")]
pub struct ListenBeginBlockResponse {}
/// ListenEndBlockRequest is the request type for the ListenEndBlock RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenEndBlockRequest")]
pub struct ListenEndBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub req: ::core::option::Option<super::super::super::super::tendermint::abci::RequestEndBlock>,
    #[prost(message, optional, tag = "2")]
    pub res: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseEndBlock>,
}
/// ListenEndBlockResponse is the response type for the ListenEndBlock RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenEndBlockResponse")]
pub struct ListenEndBlockResponse {}
/// ListenDeliverTxRequest is the request type for the ListenDeliverTx RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenDeliverTxRequest")]
pub struct ListenDeliverTxRequest {
    /// explicitly pass in block height as neither RequestDeliverTx or ResponseDeliverTx contain it
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: i64,
    #[prost(message, optional, tag = "2")]
    pub req: ::core::option::Option<super::super::super::super::tendermint::abci::RequestDeliverTx>,
    #[prost(message, optional, tag = "3")]
    pub res:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseDeliverTx>,
}
/// ListenDeliverTxResponse is the response type for the ListenDeliverTx RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenDeliverTxResponse")]
pub struct ListenDeliverTxResponse {}
/// ListenCommitRequest is the request type for the ListenCommit RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenCommitRequest")]
pub struct ListenCommitRequest {
    /// explicitly pass in block height as ResponseCommit does not contain this info
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block_height: i64,
    #[prost(message, optional, tag = "2")]
    pub res: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseCommit>,
    #[prost(message, repeated, tag = "3")]
    pub change_set:
        ::prost::alloc::vec::Vec<super::super::super::base::store::v1beta1::StoreKvPair>,
}
/// ListenCommitResponse is the response type for the ListenCommit RPC method
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
#[proto_message(type_url = "/cosmos.streaming.abci.v1.ListenCommitResponse")]
pub struct ListenCommitResponse {}
