use provwasm_proc_macro::CosmwasmExt;
/// EventTriggerCreated is an event for when a trigger is created
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.EventTriggerCreated")]
pub struct EventTriggerCreated {
    /// trigger_id is a unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub trigger_id: ::prost::alloc::string::String,
}
/// EventTriggerDestroyed is an event for when a trigger is destroyed
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.EventTriggerDestroyed")]
pub struct EventTriggerDestroyed {
    /// trigger_id is a unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub trigger_id: ::prost::alloc::string::String,
}
/// EventTriggerDetected is an event for when a trigger's event is detected
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.EventTriggerDetected")]
pub struct EventTriggerDetected {
    /// trigger_id is a unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub trigger_id: ::prost::alloc::string::String,
}
/// EventTriggerExecuted is an event for when a trigger is executed.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.EventTriggerExecuted")]
pub struct EventTriggerExecuted {
    /// trigger_id is a unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub trigger_id: ::prost::alloc::string::String,
    /// owner is the creator of the trigger.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// success indicates if all executed actions were successful.
    #[prost(bool, tag = "3")]
    pub success: bool,
}
/// Trigger
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.Trigger")]
pub struct Trigger {
    /// An integer to uniquely identify the trigger.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The owner of the trigger.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// The event that must be detected for the trigger to fire.
    #[prost(message, optional, tag = "3")]
    pub event: ::core::option::Option<crate::shim::Any>,
    /// The messages to run when the trigger fires.
    #[prost(message, repeated, tag = "4")]
    pub actions: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// QueuedTrigger
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.QueuedTrigger")]
pub struct QueuedTrigger {
    /// The block height the trigger was detected and queued.
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
    /// The time the trigger was detected and queued.
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// The trigger that was detected.
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<Trigger>,
}
/// BlockHeightEvent
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.BlockHeightEvent")]
pub struct BlockHeightEvent {
    /// The height that the trigger should fire at.
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
}
/// BlockTimeEvent
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.BlockTimeEvent")]
pub struct BlockTimeEvent {
    /// The time the trigger should fire at.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
}
/// TransactionEvent
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.TransactionEvent")]
pub struct TransactionEvent {
    /// The name of the event for a match.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The attributes that must be present for a match.
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
/// Attribute
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.Attribute")]
pub struct Attribute {
    /// The name of the attribute that the event must have to be considered a match.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the attribute that the event must have to be considered a match.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// GenesisState defines the trigger module's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.GenesisState")]
pub struct GenesisState {
    /// Trigger id is the next auto incremented id to be assigned to the next created trigger
    #[prost(uint64, tag = "1")]
    pub trigger_id: u64,
    /// Queue start is the starting index of the queue.
    #[prost(uint64, tag = "2")]
    pub queue_start: u64,
    /// Triggers to initially start with.
    #[prost(message, repeated, tag = "3")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    /// Maximum amount of gas that the triggers can use.
    /// Deprecated: We no longer need to keep track of gas limits since we use flat fees.
    #[prost(message, repeated, tag = "4")]
    pub gas_limits: ::prost::alloc::vec::Vec<GasLimit>,
    /// Triggers to initially start with in the queue.
    #[prost(message, repeated, tag = "5")]
    pub queued_triggers: ::prost::alloc::vec::Vec<QueuedTrigger>,
}
/// GasLimit defines the trigger module's grouping of a trigger and a gas limit
/// Deprecated: The GasLimit type is no longer used since we use flat fees.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.GasLimit")]
pub struct GasLimit {
    /// The identifier of the trigger this GasLimit belongs to.
    /// Deprecated: The GasLimit type is no longer used since we use flat fees.
    #[prost(uint64, tag = "1")]
    pub trigger_id: u64,
    /// The maximum amount of gas that the trigger can use.
    /// Deprecated: The GasLimit type is no longer used since we use flat fees.
    #[prost(uint64, tag = "2")]
    pub amount: u64,
}
/// QueryTriggerByIDRequest queries for the Trigger with an identifier of id.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggerByIDRequest")]
#[proto_query(
    path = "/provenance.trigger.v1.Query/TriggerByID",
    response_type = QueryTriggerByIdResponse
)]
pub struct QueryTriggerByIdRequest {
    /// The id of the trigger to query.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// QueryTriggerByIDResponse contains the requested Trigger.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggerByIDResponse")]
pub struct QueryTriggerByIdResponse {
    /// The trigger object that was queried for.
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
/// QueryTriggersRequest queries for all triggers.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggersRequest")]
#[proto_query(
    path = "/provenance.trigger.v1.Query/Triggers",
    response_type = QueryTriggersResponse
)]
pub struct QueryTriggersRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryTriggersResponse contains the list of Triggers.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.QueryTriggersResponse")]
pub struct QueryTriggersResponse {
    /// List of Trigger objects.
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgCreateTriggerRequest is the request type for creating a trigger RPC
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.MsgCreateTriggerRequest")]
pub struct MsgCreateTriggerRequest {
    /// The signing authorities for the request
    #[prost(string, repeated, tag = "1")]
    pub authorities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The event that must be detected for the trigger to fire.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<crate::shim::Any>,
    /// The messages to run when the trigger fires.
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// MsgCreateTriggerResponse is the response type for creating a trigger RPC
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.MsgCreateTriggerResponse")]
pub struct MsgCreateTriggerResponse {
    /// trigger id that is generated on creation.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// MsgDestroyTriggerRequest is the request type for creating a trigger RPC
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.MsgDestroyTriggerRequest")]
pub struct MsgDestroyTriggerRequest {
    /// the id of the trigger to destroy.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// The signing authority for the request
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgDestroyTriggerResponse is the response type for creating a trigger RPC
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.trigger.v1.MsgDestroyTriggerResponse")]
pub struct MsgDestroyTriggerResponse {}
pub struct TriggerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TriggerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn trigger_by_id(
        &self,
        id: u64,
    ) -> Result<QueryTriggerByIdResponse, cosmwasm_std::StdError> {
        QueryTriggerByIdRequest { id }.query(self.querier)
    }
    pub fn triggers(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryTriggersResponse, cosmwasm_std::StdError> {
        QueryTriggersRequest { pagination }.query(self.querier)
    }
}
