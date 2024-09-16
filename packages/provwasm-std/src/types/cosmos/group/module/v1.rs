use provwasm_proc_macro::CosmwasmExt;
/// Module is the config object of the group module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.group.module.v1.Module")]
pub struct Module {
    /// max_execution_period defines the max duration after a proposal's voting period ends that members can send a MsgExec
    /// to execute the proposal.
    #[prost(message, optional, tag = "1")]
    pub max_execution_period: ::core::option::Option<crate::shim::Duration>,
    /// max_metadata_len defines the max length of the metadata bytes field for various entities within the group module.
    /// Defaults to 255 if not explicitly set.
    #[prost(uint64, tag = "2")]
    pub max_metadata_len: u64,
}
