use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object of the gov module.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.gov.module.v1.Module")]
pub struct Module {
    /// max_metadata_len defines the maximum proposal metadata length.
    /// Defaults to 255 if not explicitly set.
    #[prost(uint64, tag = "1")]
    pub max_metadata_len: u64,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
