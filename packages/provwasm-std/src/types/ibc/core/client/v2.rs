use provwasm_proc_macro::CosmwasmExt;
/// Config is a **per-client** configuration struct that sets which relayers are allowed to relay v2 IBC messages
/// for a given client.
/// If it is set, then only relayers in the allow list can send v2 messages
/// If it is not set, then the client allows permissionless relaying of v2 messages
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.Config")]
pub struct Config {
    /// allowed_relayers defines the set of allowed relayers for IBC V2 protocol for the given client
    #[prost(string, repeated, tag = "1")]
    pub allowed_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CounterpartyInfo defines the key that the counterparty will use to message our client
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.CounterpartyInfo")]
pub struct CounterpartyInfo {
    /// merkle prefix key is the prefix that ics provable keys are stored under
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub merkle_prefix: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// client identifier is the identifier used to send packet messages to our client
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
}
/// GenesisCounterpartyInfo defines the state associating a client with a counterparty.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.GenesisCounterpartyInfo")]
pub struct GenesisCounterpartyInfo {
    /// ClientId is the ID of the given client.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// CounterpartyInfo is the counterparty info of the given client.
    #[prost(message, optional, tag = "2")]
    pub counterparty_info: ::core::option::Option<CounterpartyInfo>,
}
/// GenesisState defines the ibc client v2 submodule's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.GenesisState")]
pub struct GenesisState {
    /// counterparty info for each client
    #[prost(message, repeated, tag = "1")]
    pub counterparty_infos: ::prost::alloc::vec::Vec<GenesisCounterpartyInfo>,
}
/// QueryCounterpartyInfoRequest is the request type for the Query/CounterpartyInfo RPC
/// method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.QueryCounterpartyInfoRequest")]
#[proto_query(
    path = "/ibc.core.client.v2.Query/CounterpartyInfo",
    response_type = QueryCounterpartyInfoResponse
)]
pub struct QueryCounterpartyInfoRequest {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryCounterpartyInfoResponse is the response type for the
/// Query/CounterpartyInfo RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.QueryCounterpartyInfoResponse")]
pub struct QueryCounterpartyInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub counterparty_info: ::core::option::Option<CounterpartyInfo>,
}
/// QueryConfigRequest is the request type for the Query/Config RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.QueryConfigRequest")]
#[proto_query(
    path = "/ibc.core.client.v2.Query/Config",
    response_type = QueryConfigResponse
)]
pub struct QueryConfigRequest {
    /// client state unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryConfigResponse is the response type for the Query/Config RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.QueryConfigResponse")]
pub struct QueryConfigResponse {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<Config>,
}
/// MsgRegisterCounterparty defines a message to register a counterparty on a client
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.MsgRegisterCounterparty")]
pub struct MsgRegisterCounterparty {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// counterparty merkle prefix
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub counterparty_merkle_prefix: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// counterparty client identifier
    #[prost(string, tag = "3")]
    pub counterparty_client_id: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRegisterCounterpartyResponse defines the Msg/RegisterCounterparty response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.MsgRegisterCounterpartyResponse")]
pub struct MsgRegisterCounterpartyResponse {}
/// MsgUpdateClientConfig defines the sdk.Msg type to update the configuration for a given client
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.MsgUpdateClientConfig")]
pub struct MsgUpdateClientConfig {
    /// client identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// allowed relayers
    ///
    /// NOTE: All fields in the config must be supplied.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<Config>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpdateClientConfigResponse defines the MsgUpdateClientConfig response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.client.v2.MsgUpdateClientConfigResponse")]
pub struct MsgUpdateClientConfigResponse {}
pub struct ClientQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ClientQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn counterparty_info(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryCounterpartyInfoResponse, cosmwasm_std::StdError> {
        QueryCounterpartyInfoRequest { client_id }.query(self.querier)
    }
    pub fn config(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryConfigResponse, cosmwasm_std::StdError> {
        QueryConfigRequest { client_id }.query(self.querier)
    }
}
