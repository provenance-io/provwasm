use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object of the vesting module.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.vesting.module.v1.Module")]
pub struct Module {}
