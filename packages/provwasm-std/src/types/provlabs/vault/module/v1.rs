use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object for the module.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.module.v1.Module")]
pub struct Module {
    /// authority defines the custom module authority.
    /// If not set, defaults to the governance module.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
