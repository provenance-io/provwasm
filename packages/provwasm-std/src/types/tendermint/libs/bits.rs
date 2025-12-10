use provwasm_proc_macro::CosmwasmExt;
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/tendermint.libs.bits.BitArray")]
pub struct BitArray {
    #[prost(int64, tag = "1")]
    pub bits: i64,
    #[prost(uint64, repeated, tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<u64>,
}
