use provwasm_proc_macro::CosmwasmExt;
/// ListenEndBlockRequest is the request type for the ListenEndBlock RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.store.streaming.abci.ListenFinalizeBlockRequest")]
pub struct ListenFinalizeBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub req:
        ::core::option::Option<super::super::super::super::tendermint::abci::RequestFinalizeBlock>,
    #[prost(message, optional, tag = "2")]
    pub res:
        ::core::option::Option<super::super::super::super::tendermint::abci::ResponseFinalizeBlock>,
}
/// ListenEndBlockResponse is the response type for the ListenEndBlock RPC method
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.store.streaming.abci.ListenFinalizeBlockResponse")]
pub struct ListenFinalizeBlockResponse {}
/// ListenCommitRequest is the request type for the ListenCommit RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.store.streaming.abci.ListenCommitRequest")]
pub struct ListenCommitRequest {
    /// explicitly pass in block height as ResponseCommit does not contain this info
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, optional, tag = "2")]
    pub res: ::core::option::Option<super::super::super::super::tendermint::abci::ResponseCommit>,
    #[prost(message, repeated, tag = "3")]
    pub change_set: ::prost::alloc::vec::Vec<super::super::v1beta1::StoreKvPair>,
}
/// ListenCommitResponse is the response type for the ListenCommit RPC method
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.store.streaming.abci.ListenCommitResponse")]
pub struct ListenCommitResponse {}
