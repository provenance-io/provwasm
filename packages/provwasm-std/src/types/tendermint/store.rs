use provwasm_proc_macro::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.store.BlockStoreState")]
pub struct BlockStoreState {
    #[prost(int64, tag = "1")]
    pub base: i64,
    #[prost(int64, tag = "2")]
    pub height: i64,
}
