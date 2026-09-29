use provwasm_proc_macro::CosmwasmExt;
/// Op is a message describing a benchmark operation.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.benchmark.v1.Op")]
pub struct Op {
    #[prost(uint64, tag = "1")]
    pub seed: u64,
    #[prost(string, tag = "2")]
    pub actor: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub key_length: u64,
    #[prost(uint64, tag = "4")]
    pub value_length: u64,
    #[prost(uint32, tag = "5")]
    pub iterations: u32,
    #[prost(bool, tag = "6")]
    pub delete: bool,
    #[prost(bool, tag = "7")]
    pub exists: bool,
}
/// MsgLoadTestOps defines a message containing a sequence of load test operations.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.benchmark.v1.MsgLoadTest")]
pub struct MsgLoadTest {
    #[prost(bytes = "vec", tag = "1")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub ops: ::prost::alloc::vec::Vec<Op>,
}
/// MsgLoadTestResponse defines a message containing the results of a load test operation.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.benchmark.v1.MsgLoadTestResponse")]
pub struct MsgLoadTestResponse {
    #[prost(uint64, tag = "1")]
    pub total_time: u64,
    #[prost(uint64, tag = "2")]
    pub total_errors: u64,
}
