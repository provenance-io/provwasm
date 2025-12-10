use provwasm_proc_macro::CosmwasmExt;
/// ClientState defines the 09-localhost client state
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.lightclients.localhost.v2.ClientState")]
pub struct ClientState {
    /// the latest block height
    #[prost(message, optional, tag = "1")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
