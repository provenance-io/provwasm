use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object of the mint module.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.mint.module.v1.Module")]
pub struct Module {
    #[prost(string, tag = "1")]
    pub fee_collector_name: ::prost::alloc::string::String,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
