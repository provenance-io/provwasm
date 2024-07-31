use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object of the nft module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.nft.module.v1.Module")]
pub struct Module {}
