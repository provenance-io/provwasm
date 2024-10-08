use provwasm_proc_macro::CosmwasmExt;
/// BlockRequest requests a block for a specific height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.BlockRequest")]
pub struct BlockRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// NoBlockResponse informs the node that the peer does not have block at the requested height
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.NoBlockResponse")]
pub struct NoBlockResponse {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// BlockResponse returns block to the requested
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.BlockResponse")]
pub struct BlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::types::Block>,
    #[prost(message, optional, tag = "2")]
    pub ext_commit: ::core::option::Option<super::types::ExtendedCommit>,
}
/// StatusRequest requests the status of a peer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.StatusRequest")]
pub struct StatusRequest {}
/// StatusResponse is a peer response to inform their status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.StatusResponse")]
pub struct StatusResponse {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(int64, tag = "2")]
    pub base: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.blocksync.Message")]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1, 2, 3, 4, 5")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    use provwasm_proc_macro::CosmwasmExt;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof, ::schemars::JsonSchema)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        BlockRequest(super::BlockRequest),
        #[prost(message, tag = "2")]
        NoBlockResponse(super::NoBlockResponse),
        #[prost(message, tag = "3")]
        BlockResponse(super::BlockResponse),
        #[prost(message, tag = "4")]
        StatusRequest(super::StatusRequest),
        #[prost(message, tag = "5")]
        StatusResponse(super::StatusResponse),
    }
}
