use provwasm_proc_macro::CosmwasmExt;
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.mempool.Txs")]
pub struct Txs {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.mempool.Message")]
pub struct Message {
    #[prost(oneof = "message::Sum", tags = "1")]
    pub sum: ::core::option::Option<message::Sum>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    use provwasm_proc_macro::CosmwasmExt;
    #[derive(Clone, PartialEq, Eq, ::prost::Oneof, ::schemars::JsonSchema)]
    pub enum Sum {
        #[prost(message, tag = "1")]
        Txs(super::Txs),
    }
}
