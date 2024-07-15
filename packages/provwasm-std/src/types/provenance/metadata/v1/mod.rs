pub mod p8e;
use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// EventTxCompleted is an event message indicating that a TX has completed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventTxCompleted")]
pub struct EventTxCompleted {
    /// module is the module the TX belongs to.
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// endpoint is the rpc endpoint that was just completed.
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    /// signers are the bech32 address strings of the signers of this TX.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventScopeCreated is an event message indicating a scope has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeCreated")]
pub struct EventScopeCreated {
    /// scope_addr is the bech32 address string of the scope id that was created.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeUpdated is an event message indicating a scope has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeUpdated")]
pub struct EventScopeUpdated {
    /// scope_addr is the bech32 address string of the scope id that was updated.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeDeleted is an event message indicating a scope has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeDeleted")]
pub struct EventScopeDeleted {
    /// scope_addr is the bech32 address string of the scope id that was deleted.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionCreated is an event message indicating a session has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventSessionCreated")]
pub struct EventSessionCreated {
    /// session_addr is the bech32 address string of the session id that was created.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionUpdated is an event message indicating a session has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventSessionUpdated")]
pub struct EventSessionUpdated {
    /// session_addr is the bech32 address string of the session id that was updated.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionDeleted is an event message indicating a session has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventSessionDeleted")]
pub struct EventSessionDeleted {
    /// session_addr is the bech32 address string of the session id that was deleted.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordCreated is an event message indicating a record has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordCreated")]
pub struct EventRecordCreated {
    /// record_addr is the bech32 address string of the record id that was created.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// session_addr is the bech32 address string of the session id this record belongs to.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belongs to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordUpdated is an event message indicating a record has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordUpdated")]
pub struct EventRecordUpdated {
    /// record_addr is the bech32 address string of the record id that was updated.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// session_addr is the bech32 address string of the session id this record belongs to.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belongs to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordDeleted is an event message indicating a record has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordDeleted")]
pub struct EventRecordDeleted {
    /// record is the bech32 address string of the record id that was deleted.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belonged to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationCreated is an event message indicating a scope specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeSpecificationCreated")]
pub struct EventScopeSpecificationCreated {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// created.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationUpdated is an event message indicating a scope specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeSpecificationUpdated")]
pub struct EventScopeSpecificationUpdated {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// updated.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationDeleted is an event message indicating a scope specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventScopeSpecificationDeleted")]
pub struct EventScopeSpecificationDeleted {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// deleted.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationCreated is an event message indicating a contract specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventContractSpecificationCreated")]
pub struct EventContractSpecificationCreated {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was created.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationUpdated is an event message indicating a contract specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventContractSpecificationUpdated")]
pub struct EventContractSpecificationUpdated {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was updated.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationDeleted is an event message indicating a contract specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventContractSpecificationDeleted")]
pub struct EventContractSpecificationDeleted {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was deleted.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationCreated is an event message indicating a record specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordSpecificationCreated")]
pub struct EventRecordSpecificationCreated {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// created.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationUpdated is an event message indicating a record specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordSpecificationUpdated")]
pub struct EventRecordSpecificationUpdated {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// updated.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationDeleted is an event message indicating a record specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventRecordSpecificationDeleted")]
pub struct EventRecordSpecificationDeleted {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// deleted.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventOSLocatorCreated is an event message indicating an object store locator has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventOSLocatorCreated")]
pub struct EventOsLocatorCreated {
    /// owner is the owner in the object store locator that was created.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// EventOSLocatorUpdated is an event message indicating an object store locator has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventOSLocatorUpdated")]
pub struct EventOsLocatorUpdated {
    /// owner is the owner in the object store locator that was updated.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// EventOSLocatorDeleted is an event message indicating an object store locator has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventOSLocatorDeleted")]
pub struct EventOsLocatorDeleted {
    /// owner is the owner in the object store locator that was deleted.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// EventSetNetAssetValue event emitted when Net Asset Value for a scope is update or added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.EventSetNetAssetValue")]
pub struct EventSetNetAssetValue {
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
}
/// Params defines the set of params for the metadata module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Params")]
pub struct Params {}
/// ScopeIdInfo contains various info regarding a scope id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeIdInfo")]
pub struct ScopeIdInfo {
    /// scope_id is the raw bytes of the scope address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// scope_id_prefix is the prefix portion of the scope_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// scope_id_scope_uuid is the scope_uuid portion of the scope_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// scope_addr is the bech32 string version of the scope_id.
    #[prost(string, tag = "4")]
    pub scope_addr: ::prost::alloc::string::String,
    /// scope_uuid is the uuid hex string of the scope_id_scope_uuid.
    #[prost(string, tag = "5")]
    pub scope_uuid: ::prost::alloc::string::String,
}
/// SessionIdInfo contains various info regarding a session id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionIdInfo")]
pub struct SessionIdInfo {
    /// session_id is the raw bytes of the session address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// session_id_prefix is the prefix portion of the session_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// session_id_scope_uuid is the scope_uuid portion of the session_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// session_id_session_uuid is the session_uuid portion of the session_id.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub session_id_session_uuid: ::prost::alloc::vec::Vec<u8>,
    /// session_addr is the bech32 string version of the session_id.
    #[prost(string, tag = "5")]
    pub session_addr: ::prost::alloc::string::String,
    /// session_uuid is the uuid hex string of the session_id_session_uuid.
    #[prost(string, tag = "6")]
    pub session_uuid: ::prost::alloc::string::String,
    /// scope_id_info is information about the scope id referenced in the session_id.
    #[prost(message, optional, tag = "7")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// RecordIdInfo contains various info regarding a record id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordIdInfo")]
pub struct RecordIdInfo {
    /// record_id is the raw bytes of the record address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    /// record_id_prefix is the prefix portion of the record_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// record_id_scope_uuid is the scope_uuid portion of the record_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// record_id_hashed_name is the hashed name portion of the record_id.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    /// record_addr is the bech32 string version of the record_id.
    #[prost(string, tag = "5")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_id_info is information about the scope id referenced in the record_id.
    #[prost(message, optional, tag = "6")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// ScopeSpecIdInfo contains various info regarding a scope specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecIdInfo")]
pub struct ScopeSpecIdInfo {
    /// scope_spec_id is the raw bytes of the scope specification address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_id_prefix is the prefix portion of the scope_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_id_scope_spec_uuid is the scope_spec_uuid portion of the scope_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id_scope_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_addr is the bech32 string version of the scope_spec_id.
    #[prost(string, tag = "4")]
    pub scope_spec_addr: ::prost::alloc::string::String,
    /// scope_spec_uuid is the uuid hex string of the scope_spec_id_scope_spec_uuid.
    #[prost(string, tag = "5")]
    pub scope_spec_uuid: ::prost::alloc::string::String,
}
/// ContractSpecIdInfo contains various info regarding a contract specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecIdInfo")]
pub struct ContractSpecIdInfo {
    /// contract_spec_id is the raw bytes of the contract specification address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_id_prefix is the prefix portion of the contract_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_id_contract_spec_uuid is the contract_spec_uuid portion of the contract_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_addr is the bech32 string version of the contract_spec_id.
    #[prost(string, tag = "4")]
    pub contract_spec_addr: ::prost::alloc::string::String,
    /// contract_spec_uuid is the uuid hex string of the contract_spec_id_contract_spec_uuid.
    #[prost(string, tag = "5")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
/// RecordSpecIdInfo contains various info regarding a record specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecIdInfo")]
pub struct RecordSpecIdInfo {
    /// record_spec_id is the raw bytes of the record specification address.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_prefix is the prefix portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_contract_spec_uuid is the contract_spec_uuid portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_hashed_name is the hashed name portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_addr is the bech32 string version of the record_spec_id.
    #[prost(string, tag = "5")]
    pub record_spec_addr: ::prost::alloc::string::String,
    /// contract_spec_id_info is information about the contract spec id referenced in the record_spec_id.
    #[prost(message, optional, tag = "6")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// ScopeSpecification defines the required parties, resources, conditions, and consideration outputs for a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecification")]
pub struct ScopeSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// General information about this scope specification.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    /// Addresses of the owners of this scope specification.
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of parties that must be present on a scope (and their associated roles)
    #[prost(enumeration = "PartyType", repeated, tag = "4")]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    /// A list of contract specification ids allowed for a scope based on this specification.
    #[prost(bytes = "vec", repeated, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes_vec::serialize",
        deserialize_with = "crate::serde::as_str_bytes_vec::deserialize"
    )]
    pub contract_spec_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ContractSpecification defines the required parties, resources, conditions, and consideration outputs for a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecification")]
pub struct ContractSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// Description information for this contract specification
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    /// Address of the account that owns this specificaiton
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a list of party roles that must be fullfilled when signing a transaction for this contract specification
    #[prost(enumeration = "PartyType", repeated, tag = "4")]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    /// name of the class/type of this contract executable
    #[prost(string, tag = "7")]
    pub class_name: ::prost::alloc::string::String,
    /// Reference to a metadata record with a hash and type information for the instance of code that will process this
    /// contract
    #[prost(oneof = "contract_specification::Source", tags = "5, 6")]
    pub source: ::core::option::Option<contract_specification::Source>,
}
/// Nested message and enum types in `ContractSpecification`.
pub mod contract_specification {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// Reference to a metadata record with a hash and type information for the instance of code that will process this
    /// contract
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Source {
        /// the address of a record on chain that represents this contract
        #[prost(bytes, tag = "5")]
        ResourceId(::prost::alloc::vec::Vec<u8>),
        /// the hash of contract binary (off-chain instance)
        #[prost(string, tag = "6")]
        Hash(::prost::alloc::string::String),
    }
}
/// RecordSpecification defines the specification for a Record including allowed/required inputs/outputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecification")]
pub struct RecordSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// Name of Record that will be created when this specification is used
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// A set of inputs that must be satisified to apply this RecordSpecification and create a Record
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<InputSpecification>,
    /// A type name for data associated with this record (typically a class or proto name)
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    /// Type of result for this record specification (must be RECORD or RECORD_LIST)
    #[prost(enumeration = "DefinitionType", tag = "5")]
    #[serde(
        serialize_with = "DefinitionType::serialize",
        deserialize_with = "DefinitionType::deserialize"
    )]
    pub result_type: i32,
    /// Type of party responsible for this record
    #[prost(enumeration = "PartyType", repeated, tag = "6")]
    pub responsible_parties: ::prost::alloc::vec::Vec<i32>,
}
/// InputSpecification defines a name, type_name, and source reference (either on or off chain) to define an input
/// parameter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.InputSpecification")]
pub struct InputSpecification {
    /// name for this input
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// a type_name (typically a proto name or class_name)
    #[prost(string, tag = "2")]
    pub type_name: ::prost::alloc::string::String,
    /// source is either on chain (record_id) or off-chain (hash)
    #[prost(oneof = "input_specification::Source", tags = "3, 4")]
    pub source: ::core::option::Option<input_specification::Source>,
}
/// Nested message and enum types in `InputSpecification`.
pub mod input_specification {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// source is either on chain (record_id) or off-chain (hash)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Source {
        /// the address of a record on chain (For Established Records)
        #[prost(bytes, tag = "3")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        /// the hash of an off-chain piece of information (For Proposed Records)
        #[prost(string, tag = "4")]
        Hash(::prost::alloc::string::String),
    }
}
/// Description holds general information that is handy to associate with a structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Description")]
pub struct Description {
    /// A Name for this thing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of this thing.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// URL to find even more info.
    #[prost(string, tag = "4")]
    pub website_url: ::prost::alloc::string::String,
    /// URL of an icon.
    #[prost(string, tag = "5")]
    pub icon_url: ::prost::alloc::string::String,
}
/// DefinitionType indicates the required definition type for this value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum DefinitionType {
    /// DEFINITION_TYPE_UNSPECIFIED indicates an unknown/invalid value
    Unspecified = 0,
    /// DEFINITION_TYPE_PROPOSED indicates a proposed value is used here (a record that is not on-chain)
    Proposed = 1,
    /// DEFINITION_TYPE_RECORD indicates the value must be a reference to a record on chain
    Record = 2,
    /// DEFINITION_TYPE_RECORD_LIST indicates the value maybe a reference to a collection of values on chain having
    /// the same name
    RecordList = 3,
}
impl DefinitionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DefinitionType::Unspecified => "DEFINITION_TYPE_UNSPECIFIED",
            DefinitionType::Proposed => "DEFINITION_TYPE_PROPOSED",
            DefinitionType::Record => "DEFINITION_TYPE_RECORD",
            DefinitionType::RecordList => "DEFINITION_TYPE_RECORD_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFINITION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DEFINITION_TYPE_PROPOSED" => Some(Self::Proposed),
            "DEFINITION_TYPE_RECORD" => Some(Self::Record),
            "DEFINITION_TYPE_RECORD_LIST" => Some(Self::RecordList),
            _ => None,
        }
    }
}
/// PartyType are the different roles parties on a contract may use
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum PartyType {
    /// PARTY_TYPE_UNSPECIFIED is an error condition
    Unspecified = 0,
    /// PARTY_TYPE_ORIGINATOR is an asset originator
    Originator = 1,
    /// PARTY_TYPE_SERVICER provides debt servicing functions
    Servicer = 2,
    /// PARTY_TYPE_INVESTOR is a generic investor
    Investor = 3,
    /// PARTY_TYPE_CUSTODIAN is an entity that provides custodian services for assets
    Custodian = 4,
    /// PARTY_TYPE_OWNER indicates this party is an owner of the item
    Owner = 5,
    /// PARTY_TYPE_AFFILIATE is a party with an affiliate agreement
    Affiliate = 6,
    /// PARTY_TYPE_OMNIBUS is a special type of party that controls an omnibus bank account
    Omnibus = 7,
    /// PARTY_TYPE_PROVENANCE is used to indicate this party represents the blockchain or a smart contract action
    Provenance = 8,
    /// PARTY_TYPE_CONTROLLER is an entity which controls a specific asset on chain (ie enote)
    Controller = 10,
    /// PARTY_TYPE_VALIDATOR is an entity which validates given assets on chain
    Validator = 11,
}
impl PartyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PartyType::Unspecified => "PARTY_TYPE_UNSPECIFIED",
            PartyType::Originator => "PARTY_TYPE_ORIGINATOR",
            PartyType::Servicer => "PARTY_TYPE_SERVICER",
            PartyType::Investor => "PARTY_TYPE_INVESTOR",
            PartyType::Custodian => "PARTY_TYPE_CUSTODIAN",
            PartyType::Owner => "PARTY_TYPE_OWNER",
            PartyType::Affiliate => "PARTY_TYPE_AFFILIATE",
            PartyType::Omnibus => "PARTY_TYPE_OMNIBUS",
            PartyType::Provenance => "PARTY_TYPE_PROVENANCE",
            PartyType::Controller => "PARTY_TYPE_CONTROLLER",
            PartyType::Validator => "PARTY_TYPE_VALIDATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PARTY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PARTY_TYPE_ORIGINATOR" => Some(Self::Originator),
            "PARTY_TYPE_SERVICER" => Some(Self::Servicer),
            "PARTY_TYPE_INVESTOR" => Some(Self::Investor),
            "PARTY_TYPE_CUSTODIAN" => Some(Self::Custodian),
            "PARTY_TYPE_OWNER" => Some(Self::Owner),
            "PARTY_TYPE_AFFILIATE" => Some(Self::Affiliate),
            "PARTY_TYPE_OMNIBUS" => Some(Self::Omnibus),
            "PARTY_TYPE_PROVENANCE" => Some(Self::Provenance),
            "PARTY_TYPE_CONTROLLER" => Some(Self::Controller),
            "PARTY_TYPE_VALIDATOR" => Some(Self::Validator),
            _ => None,
        }
    }
}
/// Scope defines a root reference for a collection of records owned by one or more parties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Scope")]
pub struct Scope {
    /// Unique ID for this scope.  Implements sdk.Address interface for use where addresses are required in Cosmos
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// the scope specification that contains the specifications for data elements allowed within this scope
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// These parties represent top level owners of the records within.  These parties must sign any requests that modify
    /// the data within the scope.  These addresses are in union with parties listed on the sessions.
    #[prost(message, repeated, tag = "3")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    /// Addresses in this list are authorized to receive off-chain data associated with this scope.
    #[prost(string, repeated, tag = "4")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An address that controls the value associated with this scope.  Standard blockchain accounts and marker accounts
    /// are supported for this value.  This attribute may only be changed by the entity indicated once it is set.
    #[prost(string, tag = "5")]
    pub value_owner_address: ::prost::alloc::string::String,
    /// Whether all parties in this scope and its sessions must be present in this scope's owners field.
    /// This also enables use of optional=true scope owners and session parties.
    #[prost(bool, tag = "6")]
    pub require_party_rollup: bool,
}
/// Session defines an execution context against a specific specification instance.
/// The context will have a specification and set of parties involved.
///
/// NOTE: When there are no more Records within a Scope that reference a Session, the Session is removed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Session")]
pub struct Session {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// unique id of the contract specification that was used to create this session.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// parties is the set of identities that signed this contract
    #[prost(message, repeated, tag = "3")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
    /// name to associate with this session execution context, typically classname
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// context is a field for storing client specific data associated with a session.
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub context: ::prost::alloc::vec::Vec<u8>,
    /// Created by, updated by, timestamps, version number, and related info.
    #[prost(message, optional, tag = "99")]
    pub audit: ::core::option::Option<AuditFields>,
}
/// A record (of fact) is attached to a session or each consideration output from a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Record")]
pub struct Record {
    /// name/identifier for this record.  Value must be unique within the scope.  Also known as a Fact name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// id of the session context that was used to create this record (use with filtered kvprefix iterator)
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// process contain information used to uniquely identify an execution on or off chain that generated this record
    #[prost(message, optional, tag = "3")]
    pub process: ::core::option::Option<Process>,
    /// inputs used with the process to achieve the output on this record
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<RecordInput>,
    /// output(s) is the results of executing the process on the given process indicated in this record
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<RecordOutput>,
    /// specification_id is the id of the record specification that was used to create this record.
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
}
/// Process contains information used to uniquely identify what was used to generate this record
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Process")]
pub struct Process {
    /// a name associated with the process (type_name, classname or smart contract common name)
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// method is a name or reference to a specific operation (method) within a class/contract that was invoked
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    /// unique identifier for this process
    #[prost(oneof = "process::ProcessId", tags = "1, 2")]
    pub process_id: ::core::option::Option<process::ProcessId>,
}
/// Nested message and enum types in `Process`.
pub mod process {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// unique identifier for this process
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum ProcessId {
        /// the address of a smart contract used for this process
        #[prost(string, tag = "1")]
        Address(::prost::alloc::string::String),
        /// the hash of an off-chain process used
        #[prost(string, tag = "2")]
        Hash(::prost::alloc::string::String),
    }
}
/// Tracks the inputs used to establish this record
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordInput")]
pub struct RecordInput {
    /// Name value included to link back to the definition spec.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// from proposed fact structure to unmarshal
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    /// Indicates if this input was a recorded fact on chain or just a given hashed input
    #[prost(enumeration = "RecordInputStatus", tag = "5")]
    #[serde(
        serialize_with = "RecordInputStatus::serialize",
        deserialize_with = "RecordInputStatus::deserialize"
    )]
    pub status: i32,
    /// data source
    #[prost(oneof = "record_input::Source", tags = "2, 3")]
    pub source: ::core::option::Option<record_input::Source>,
}
/// Nested message and enum types in `RecordInput`.
pub mod record_input {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// data source
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Source {
        /// the address of a record on chain (For Established Records)
        #[prost(bytes, tag = "2")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        /// the hash of an off-chain piece of information (For Proposed Records)
        #[prost(string, tag = "3")]
        Hash(::prost::alloc::string::String),
    }
}
/// RecordOutput encapsulates the output of a process recorded on chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordOutput")]
pub struct RecordOutput {
    /// Hash of the data output that was output/generated for this record
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// Status of the process execution associated with this output indicating success,failure, or pending
    #[prost(enumeration = "ResultStatus", tag = "2")]
    #[serde(
        serialize_with = "ResultStatus::serialize",
        deserialize_with = "ResultStatus::deserialize"
    )]
    pub status: i32,
}
/// A Party is an address with/in a given role associated with a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.Party")]
pub struct Party {
    /// address of the account (on chain)
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// a role for this account within the context of the processes used
    #[prost(enumeration = "PartyType", tag = "2")]
    #[serde(
        serialize_with = "PartyType::serialize",
        deserialize_with = "PartyType::deserialize"
    )]
    pub role: i32,
    /// whether this party's signature is optional
    #[prost(bool, tag = "3")]
    pub optional: bool,
}
/// AuditFields capture information about the last account to make modifications and when they were made
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.AuditFields")]
pub struct AuditFields {
    /// the date/time when this entry was created
    #[prost(message, optional, tag = "1")]
    pub created_date: ::core::option::Option<crate::shim::Timestamp>,
    /// the address of the account that created this record
    #[prost(string, tag = "2")]
    pub created_by: ::prost::alloc::string::String,
    /// the date/time when this entry was last updated
    #[prost(message, optional, tag = "3")]
    pub updated_date: ::core::option::Option<crate::shim::Timestamp>,
    /// the address of the account that modified this record
    #[prost(string, tag = "4")]
    pub updated_by: ::prost::alloc::string::String,
    /// an optional version number that is incremented with each update
    #[prost(uint32, tag = "5")]
    pub version: u32,
    /// an optional message associated with the creation/update event
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
}
/// NetAssetValue defines a scope's net asset value
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.NetAssetValue")]
pub struct NetAssetValue {
    /// price is the complete value of the asset's volume
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// updated_block_height is the block height of last update
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub updated_block_height: u64,
}
/// A set of types for inputs on a record (of fact)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum RecordInputStatus {
    /// RECORD_INPUT_STATUS_UNSPECIFIED indicates an invalid/unknown input type
    Unspecified = 0,
    /// RECORD_INPUT_STATUS_PROPOSED indicates this input was an arbitrary piece of data that was hashed
    Proposed = 1,
    /// RECORD_INPUT_STATUS_RECORD indicates this input is a reference to a previously recorded fact on blockchain
    Record = 2,
}
impl RecordInputStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecordInputStatus::Unspecified => "RECORD_INPUT_STATUS_UNSPECIFIED",
            RecordInputStatus::Proposed => "RECORD_INPUT_STATUS_PROPOSED",
            RecordInputStatus::Record => "RECORD_INPUT_STATUS_RECORD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECORD_INPUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "RECORD_INPUT_STATUS_PROPOSED" => Some(Self::Proposed),
            "RECORD_INPUT_STATUS_RECORD" => Some(Self::Record),
            _ => None,
        }
    }
}
/// ResultStatus indicates the various states of execution of a record
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum ResultStatus {
    /// RESULT_STATUS_UNSPECIFIED indicates an unset condition
    Unspecified = 0,
    /// RESULT_STATUS_PASS indicates the execution was successful
    Pass = 1,
    /// RESULT_STATUS_SKIP indicates condition/consideration was skipped due to missing inputs or delayed execution
    Skip = 2,
    /// RESULT_STATUS_FAIL indicates the execution of the condition/consideration failed.
    Fail = 3,
}
impl ResultStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultStatus::Unspecified => "RESULT_STATUS_UNSPECIFIED",
            ResultStatus::Pass => "RESULT_STATUS_PASS",
            ResultStatus::Skip => "RESULT_STATUS_SKIP",
            ResultStatus::Fail => "RESULT_STATUS_FAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESULT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "RESULT_STATUS_PASS" => Some(Self::Pass),
            "RESULT_STATUS_SKIP" => Some(Self::Skip),
            "RESULT_STATUS_FAIL" => Some(Self::Fail),
            _ => None,
        }
    }
}
/// Defines an Locator object stored on chain, which represents a owner( blockchain address) associated with a endpoint
/// uri for it's associated object store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ObjectStoreLocator")]
pub struct ObjectStoreLocator {
    /// account address the endpoint is owned by
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// locator endpoint uri
    #[prost(string, tag = "2")]
    pub locator_uri: ::prost::alloc::string::String,
    /// owners encryption key address
    #[prost(string, tag = "3")]
    pub encryption_key: ::prost::alloc::string::String,
}
/// Params defines the parameters for the metadata-locator module methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorParams")]
pub struct OsLocatorParams {
    #[prost(uint32, tag = "1")]
    pub max_uri_length: u32,
}
/// GenesisState defines the account module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// A collection of metadata scopes and specs to create on start
    #[prost(message, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<Scope>,
    #[prost(message, repeated, tag = "3")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    #[prost(message, repeated, tag = "4")]
    pub records: ::prost::alloc::vec::Vec<Record>,
    #[prost(message, repeated, tag = "5")]
    pub scope_specifications: ::prost::alloc::vec::Vec<ScopeSpecification>,
    #[prost(message, repeated, tag = "6")]
    pub contract_specifications: ::prost::alloc::vec::Vec<ContractSpecification>,
    #[prost(message, repeated, tag = "7")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecification>,
    #[prost(message, optional, tag = "8")]
    pub o_s_locator_params: ::core::option::Option<OsLocatorParams>,
    #[prost(message, repeated, tag = "9")]
    pub object_store_locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// Net asset values assigned to scopes
    #[prost(message, repeated, tag = "10")]
    pub net_asset_values: ::prost::alloc::vec::Vec<MarkerNetAssetValues>,
}
/// MarkerNetAssetValues defines the net asset values for a scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MarkerNetAssetValues")]
pub struct MarkerNetAssetValues {
    /// address defines the scope address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// net_asset_values that are assigned to scope
    #[prost(message, repeated, tag = "2")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<QueryParamsRequest>,
}
/// ScopeRequest is the request type for the Query/Scope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/Scope",
    response_type = ScopeResponse
)]
pub struct ScopeRequest {
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_addr is a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    /// include_sessions is a flag for whether to include the sessions of the scope in the response.
    #[prost(bool, tag = "10")]
    pub include_sessions: bool,
    /// include_records is a flag for whether to include the records of the scope in the response.
    #[prost(bool, tag = "11")]
    pub include_records: bool,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// ScopeResponse is the response type for the Query/Scope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeResponse")]
pub struct ScopeResponse {
    /// scope is the wrapped scope result.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped sessions in this scope (if requested).
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped records in this scope (if requested).
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeRequest>,
}
/// SessionWrapper contains a single scope and its uuid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeWrapper")]
pub struct ScopeWrapper {
    /// scope is the on-chain scope message.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    /// scope_id_info contains information about the id/address of the scope.
    #[prost(message, optional, tag = "2")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
    /// scope_spec_id_info contains information about the id/address of the scope specification.
    #[prost(message, optional, tag = "3")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// ScopesAllRequest is the request type for the Query/ScopesAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopesAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ScopesAll",
    response_type = ScopesAllResponse
)]
pub struct ScopesAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// ScopesAllResponse is the response type for the Query/ScopesAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopesAllResponse")]
pub struct ScopesAllResponse {
    /// scopes are the wrapped scopes.
    #[prost(message, repeated, tag = "1")]
    pub scopes: ::prost::alloc::vec::Vec<ScopeWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopesAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// SessionsRequest is the request type for the Query/Sessions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionsRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/Sessions",
    response_type = SessionsResponse
)]
pub struct SessionsRequest {
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_id can either be a uuid, e.g. 5803f8bc-6067-4eb5-951f-2121671c2ec0 or a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. This can only be a uuid if a scope_id is also
    /// provided.
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    /// record_name is the name of the record to find the session for in the provided scope.
    #[prost(string, tag = "4")]
    pub record_name: ::prost::alloc::string::String,
    /// include_scope is a flag for whether to include the scope containing these sessions in the response.
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    /// include_records is a flag for whether to include the records of these sessions in the response.
    #[prost(bool, tag = "11")]
    pub include_records: bool,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// SessionsResponse is the response type for the Query/Sessions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionsResponse")]
pub struct SessionsResponse {
    /// scope is the wrapped scope that holds these sessions (if requested).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped session results.
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped records contained in these sessions (if requested).
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<SessionsRequest>,
}
/// SessionWrapper contains a single session and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionWrapper")]
pub struct SessionWrapper {
    /// session is the on-chain session message.
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    /// session_id_info contains information about the id/address of the session.
    #[prost(message, optional, tag = "2")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
    /// contract_spec_id_info contains information about the id/address of the contract specification.
    #[prost(message, optional, tag = "3")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// SessionsAllRequest is the request type for the Query/SessionsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionsAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/SessionsAll",
    response_type = SessionsAllResponse
)]
pub struct SessionsAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// SessionsAllResponse is the response type for the Query/SessionsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionsAllResponse")]
pub struct SessionsAllResponse {
    /// sessions are the wrapped sessions.
    #[prost(message, repeated, tag = "1")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<SessionsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// RecordsRequest is the request type for the Query/Records RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordsRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/Records",
    response_type = RecordsResponse
)]
pub struct RecordsRequest {
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "2")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_id can either be a uuid, e.g. 5803f8bc-6067-4eb5-951f-2121671c2ec0 or a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. This can only be a uuid if a scope_id is also
    /// provided.
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    /// name is the name of the record to look for
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// include_scope is a flag for whether to include the the scope containing these records in the response.
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    /// include_sessions is a flag for whether to include the sessions containing these records in the response.
    #[prost(bool, tag = "11")]
    pub include_sessions: bool,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// RecordsResponse is the response type for the Query/Records RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordsResponse")]
pub struct RecordsResponse {
    /// scope is the wrapped scope that holds these records (if requested).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped sessions that hold these records (if requested).
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped record results.
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordsRequest>,
}
/// RecordWrapper contains a single record and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordWrapper")]
pub struct RecordWrapper {
    /// record is the on-chain record message.
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    /// record_id_info contains information about the id/address of the record.
    #[prost(message, optional, tag = "2")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
    /// record_spec_id_info contains information about the id/address of the record specification.
    #[prost(message, optional, tag = "3")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// RecordsAllRequest is the request type for the Query/RecordsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordsAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/RecordsAll",
    response_type = RecordsAllResponse
)]
pub struct RecordsAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// RecordsAllResponse is the response type for the Query/RecordsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordsAllResponse")]
pub struct RecordsAllResponse {
    /// records are the wrapped records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// OwnershipRequest is the request type for the Query/Ownership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OwnershipRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/Ownership",
    response_type = OwnershipResponse
)]
pub struct OwnershipRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// OwnershipResponse is the response type for the Query/Ownership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OwnershipResponse")]
pub struct OwnershipResponse {
    /// A list of scope ids (uuid) associated with the given address.
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OwnershipRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// ValueOwnershipRequest is the request type for the Query/ValueOwnership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ValueOwnershipRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ValueOwnership",
    response_type = ValueOwnershipResponse
)]
pub struct ValueOwnershipRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// ValueOwnershipResponse is the response type for the Query/ValueOwnership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ValueOwnershipResponse")]
pub struct ValueOwnershipResponse {
    /// A list of scope ids (uuid) associated with the given address.
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ValueOwnershipRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// ScopeSpecificationRequest is the request type for the Query/ScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecificationRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ScopeSpecification",
    response_type = ScopeSpecificationResponse
)]
pub struct ScopeSpecificationRequest {
    /// specification_id can either be a uuid, e.g. dc83ea70-eacd-40fe-9adf-1cf6148bf8a2 or a bech32 scope specification
    /// address, e.g. scopespec1qnwg86nsatx5pl56muw0v9ytlz3qu3jx6m.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// include_contract_specs is a flag for whether to include the contract specifications of the scope specification in
    /// the response.
    #[prost(bool, tag = "10")]
    pub include_contract_specs: bool,
    /// include_record_specs is a flag for whether to include the record specifications of the scope specification in the
    /// response.
    #[prost(bool, tag = "11")]
    pub include_record_specs: bool,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// ScopeSpecificationResponse is the response type for the Query/ScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecificationResponse")]
pub struct ScopeSpecificationResponse {
    /// scope_specification is the wrapped scope specification.
    #[prost(message, optional, tag = "1")]
    pub scope_specification: ::core::option::Option<ScopeSpecificationWrapper>,
    /// contract_specs is any number of wrapped contract specifications in this scope specification (if requested).
    #[prost(message, repeated, tag = "2")]
    pub contract_specs: ::prost::alloc::vec::Vec<ContractSpecificationWrapper>,
    /// record_specs is any number of wrapped record specifications in this scope specification (if requested).
    #[prost(message, repeated, tag = "3")]
    pub record_specs: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeSpecificationRequest>,
}
/// ScopeSpecificationWrapper contains a single scope specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecificationWrapper")]
pub struct ScopeSpecificationWrapper {
    /// specification is the on-chain scope specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    /// scope_spec_id_info contains information about the id/address of the scope specification.
    #[prost(message, optional, tag = "2")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// ScopeSpecificationsAllRequest is the request type for the Query/ScopeSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecificationsAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ScopeSpecificationsAll",
    response_type = ScopeSpecificationsAllResponse
)]
pub struct ScopeSpecificationsAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// ScopeSpecificationsAllResponse is the response type for the Query/ScopeSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ScopeSpecificationsAllResponse")]
pub struct ScopeSpecificationsAllResponse {
    /// scope_specifications are the wrapped scope specifications.
    #[prost(message, repeated, tag = "1")]
    pub scope_specifications: ::prost::alloc::vec::Vec<ScopeSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// ContractSpecificationRequest is the request type for the Query/ContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecificationRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ContractSpecification",
    response_type = ContractSpecificationResponse
)]
pub struct ContractSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// include_record_specs is a flag for whether to include the the record specifications of this contract specification
    /// in the response.
    #[prost(bool, tag = "10")]
    pub include_record_specs: bool,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// ContractSpecificationResponse is the response type for the Query/ContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecificationResponse")]
pub struct ContractSpecificationResponse {
    /// contract_specification is the wrapped contract specification.
    #[prost(message, optional, tag = "1")]
    pub contract_specification: ::core::option::Option<ContractSpecificationWrapper>,
    /// record_specifications is any number or wrapped record specifications associated with this contract_specification
    /// (if requested).
    #[prost(message, repeated, tag = "3")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ContractSpecificationRequest>,
}
/// ContractSpecificationWrapper contains a single contract specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecificationWrapper")]
pub struct ContractSpecificationWrapper {
    /// specification is the on-chain contract specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    /// contract_spec_id_info contains information about the id/address of the contract specification.
    #[prost(message, optional, tag = "2")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// ContractSpecificationsAllRequest is the request type for the Query/ContractSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecificationsAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ContractSpecificationsAll",
    response_type = ContractSpecificationsAllResponse
)]
pub struct ContractSpecificationsAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// ContractSpecificationsAllResponse is the response type for the Query/ContractSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.ContractSpecificationsAllResponse")]
pub struct ContractSpecificationsAllResponse {
    /// contract_specifications are the wrapped contract specifications.
    #[prost(message, repeated, tag = "1")]
    pub contract_specifications: ::prost::alloc::vec::Vec<ContractSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ContractSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// RecordSpecificationsForContractSpecificationRequest is the request type for the
/// Query/RecordSpecificationsForContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/provenance.metadata.v1.RecordSpecificationsForContractSpecificationRequest"
)]
#[proto_query(
    path = "/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification",
    response_type = RecordSpecificationsForContractSpecificationResponse
)]
pub struct RecordSpecificationsForContractSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// RecordSpecificationsForContractSpecificationResponse is the response type for the
/// Query/RecordSpecificationsForContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(
    type_url = "/provenance.metadata.v1.RecordSpecificationsForContractSpecificationResponse"
)]
pub struct RecordSpecificationsForContractSpecificationResponse {
    /// record_specifications is any number of wrapped record specifications associated with this contract_specification.
    #[prost(message, repeated, tag = "1")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// contract_specification_uuid is the uuid of this contract specification.
    #[prost(string, tag = "2")]
    pub contract_specification_uuid: ::prost::alloc::string::String,
    /// contract_specification_addr is the contract specification address as a bech32 encoded string.
    #[prost(string, tag = "3")]
    pub contract_specification_addr: ::prost::alloc::string::String,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationsForContractSpecificationRequest>,
}
/// RecordSpecificationRequest is the request type for the Query/RecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecificationRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/RecordSpecification",
    response_type = RecordSpecificationResponse
)]
pub struct RecordSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// name is the name of the record to look up.
    /// It is required if the specification_id is a uuid or contract specification address.
    /// It is ignored if the specification_id is a record specification address.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// RecordSpecificationResponse is the response type for the Query/RecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecificationResponse")]
pub struct RecordSpecificationResponse {
    /// record_specification is the wrapped record specification.
    #[prost(message, optional, tag = "1")]
    pub record_specification: ::core::option::Option<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationRequest>,
}
/// RecordSpecificationWrapper contains a single record specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecificationWrapper")]
pub struct RecordSpecificationWrapper {
    /// specification is the on-chain record specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    /// record_spec_id_info contains information about the id/address of the record specification.
    #[prost(message, optional, tag = "2")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// RecordSpecificationsAllRequest is the request type for the Query/RecordSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecificationsAllRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/RecordSpecificationsAll",
    response_type = RecordSpecificationsAllResponse
)]
pub struct RecordSpecificationsAllRequest {
    /// exclude_id_info is a flag for whether to exclude the id info from the response.
    #[prost(bool, tag = "12")]
    pub exclude_id_info: bool,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// RecordSpecificationsAllResponse is the response type for the Query/RecordSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.RecordSpecificationsAllResponse")]
pub struct RecordSpecificationsAllResponse {
    /// record_specifications are the wrapped record specifications.
    #[prost(message, repeated, tag = "1")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// GetByAddrRequest is the request type for the Query/GetByAddr RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.GetByAddrRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/GetByAddr",
    response_type = GetByAddrResponse
)]
pub struct GetByAddrRequest {
    /// ids are the metadata addresses of the things to look up.
    #[prost(string, repeated, tag = "1")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetByAddrResponse is the response type for the Query/GetByAddr RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.GetByAddrResponse")]
pub struct GetByAddrResponse {
    /// scopes contains any scopes that were requested and found.
    #[prost(message, repeated, tag = "1")]
    pub scopes: ::prost::alloc::vec::Vec<Scope>,
    /// sessions contains any sessions that were requested and found.
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    /// records contains any records that were requested and found.
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<Record>,
    /// scope_specs contains any scope specifications that were requested and found.
    #[prost(message, repeated, tag = "4")]
    pub scope_specs: ::prost::alloc::vec::Vec<ScopeSpecification>,
    /// contract_specs contains any contract specifications that were requested and found.
    #[prost(message, repeated, tag = "5")]
    pub contract_specs: ::prost::alloc::vec::Vec<ContractSpecification>,
    /// record_specs contains any record specifications that were requested and found.
    #[prost(message, repeated, tag = "6")]
    pub record_specs: ::prost::alloc::vec::Vec<RecordSpecification>,
    /// not_found contains any addrs requested but not found.
    #[prost(string, repeated, tag = "7")]
    pub not_found: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// OSLocatorParamsRequest is the request type for the Query/OSLocatorParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorParamsRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/OSLocatorParams",
    response_type = OsLocatorParamsResponse
)]
pub struct OsLocatorParamsRequest {
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// OSLocatorParamsResponse is the response type for the Query/OSLocatorParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorParamsResponse")]
pub struct OsLocatorParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<OsLocatorParams>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorParamsRequest>,
}
/// OSLocatorRequest is the request type for the Query/OSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/OSLocator",
    response_type = OsLocatorResponse
)]
pub struct OsLocatorRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// OSLocatorResponse is the response type for the Query/OSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorResponse")]
pub struct OsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorRequest>,
}
/// OSLocatorsByURIRequest is the request type for the Query/OSLocatorsByURI RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorsByURIRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/OSLocatorsByURI",
    response_type = OsLocatorsByUriResponse
)]
pub struct OsLocatorsByUriRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// OSLocatorsByURIResponse is the response type for the Query/OSLocatorsByURI RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorsByURIResponse")]
pub struct OsLocatorsByUriResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByUriRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// OSLocatorsByScopeRequest is the request type for the Query/OSLocatorsByScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorsByScopeRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/OSLocatorsByScope",
    response_type = OsLocatorsByScopeResponse
)]
pub struct OsLocatorsByScopeRequest {
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
/// OSLocatorsByScopeResponse is the response type for the Query/OSLocatorsByScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSLocatorsByScopeResponse")]
pub struct OsLocatorsByScopeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByScopeRequest>,
}
/// OSAllLocatorsRequest is the request type for the Query/OSAllLocators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSAllLocatorsRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/OSAllLocators",
    response_type = OsAllLocatorsResponse
)]
pub struct OsAllLocatorsRequest {
    /// include_request is a flag for whether to include this request in your result.
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// OSAllLocatorsResponse is the response type for the Query/OSAllLocators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.OSAllLocatorsResponse")]
pub struct OsAllLocatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsAllLocatorsRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// AccountDataRequest is the request type for the Query/AccountData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.AccountDataRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/AccountData",
    response_type = AccountDataResponse
)]
pub struct AccountDataRequest {
    /// The metadata address to look up.
    /// Currently, only scope ids are supported.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub metadata_addr: ::prost::alloc::vec::Vec<u8>,
}
/// AccountDataResponse is the response type for the Query/AccountData RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.AccountDataResponse")]
pub struct AccountDataResponse {
    /// The accountdata for the requested metadata address.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// QueryNetAssetValuesRequest is the request type for the Query/NetAssetValues method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.QueryScopeNetAssetValuesRequest")]
#[proto_query(
    path = "/provenance.metadata.v1.Query/ScopeNetAssetValues",
    response_type = QueryScopeNetAssetValuesResponse
)]
pub struct QueryScopeNetAssetValuesRequest {
    /// scopeid metadata address
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryNetAssetValuesRequest is the response type for the Query/NetAssetValues method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.QueryScopeNetAssetValuesResponse")]
pub struct QueryScopeNetAssetValuesResponse {
    /// net asset values for scope
    #[prost(message, repeated, tag = "1")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// MsgWriteScopeRequest is the request type for the Msg/WriteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteScopeRequest")]
pub struct MsgWriteScopeRequest {
    /// scope is the Scope you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// scope_uuid is an optional uuid string, e.g. "91978ba2-5f35-459a-86a7-feca1b0512e0"
    /// If provided, it will be used to generate the MetadataAddress for the scope which will override the scope_id in the
    /// provided scope. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in scope.scope_id that is different from the one created from this uuid, an error is returned.
    #[prost(string, tag = "3")]
    pub scope_uuid: ::prost::alloc::string::String,
    /// spec_uuid is an optional scope specification uuid string, e.g. "dc83ea70-eacd-40fe-9adf-1cf6148bf8a2"
    /// If provided, it will be used to generate the MetadataAddress for the scope specification which will override the
    /// specification_id in the provided scope. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in scope.specification_id that is different from the one created from this uuid, an error is
    /// returned.
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
    /// usd_mills value of scope in usd mills (1234 = $1.234) used for net asset value
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_mills: u64,
}
/// MsgWriteScopeResponse is the response type for the Msg/WriteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteScopeResponse")]
pub struct MsgWriteScopeResponse {
    /// scope_id_info contains information about the id/address of the scope that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// MsgDeleteScopeRequest is the request type for the Msg/DeleteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeRequest")]
pub struct MsgDeleteScopeRequest {
    /// Unique ID for the scope to delete
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeResponse is the response type for the Msg/DeleteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeResponse")]
pub struct MsgDeleteScopeResponse {}
/// MsgAddScopeDataAccessRequest is the request to add data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddScopeDataAccessRequest")]
pub struct MsgAddScopeDataAccessRequest {
    /// scope MetadataAddress for updating data access
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress addresses to be added to scope
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddScopeDataAccessResponse is the response for adding data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddScopeDataAccessResponse")]
pub struct MsgAddScopeDataAccessResponse {}
/// MsgDeleteScopeDataAccessRequest is the request to remove data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeDataAccessRequest")]
pub struct MsgDeleteScopeDataAccessRequest {
    /// scope MetadataAddress for removing data access
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress address to be removed from scope
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeDataAccessResponse is the response from removing data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeDataAccessResponse")]
pub struct MsgDeleteScopeDataAccessResponse {}
/// MsgAddScopeOwnerRequest is the request to add owner AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddScopeOwnerRequest")]
pub struct MsgAddScopeOwnerRequest {
    /// scope MetadataAddress for updating data access
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// owner parties to add to the scope
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddScopeOwnerResponse is the response for adding owner AccAddresses to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddScopeOwnerResponse")]
pub struct MsgAddScopeOwnerResponse {}
/// MsgDeleteScopeOwnerRequest is the request to remove owner AccAddresses to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeOwnerRequest")]
pub struct MsgDeleteScopeOwnerRequest {
    /// scope MetadataAddress for removing data access
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress owner addresses to be removed from scope
    #[prost(string, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeOwnerResponse is the response from removing owner AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeOwnerResponse")]
pub struct MsgDeleteScopeOwnerResponse {}
/// MsgUpdateValueOwnersRequest is the request to update the value owner addresses in one or more scopes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgUpdateValueOwnersRequest")]
pub struct MsgUpdateValueOwnersRequest {
    /// scope_ids are the scope metadata addresses of all scopes to be updated.
    #[prost(bytes = "vec", repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes_vec::serialize",
        deserialize_with = "crate::serde::as_str_bytes_vec::deserialize"
    )]
    pub scope_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// value_owner_address is the address of the new value owner for the provided scopes.
    #[prost(string, tag = "2")]
    pub value_owner_address: ::prost::alloc::string::String,
    /// signers is the list of addresses of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgUpdateValueOwnersResponse is the response from updating value owner addresses in one or more scopes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgUpdateValueOwnersResponse")]
pub struct MsgUpdateValueOwnersResponse {}
/// MsgMigrateValueOwnerRequest is the request to migrate all scopes with one value owner to another value owner.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgMigrateValueOwnerRequest")]
pub struct MsgMigrateValueOwnerRequest {
    /// existing is the value owner address that is being migrated.
    #[prost(string, tag = "1")]
    pub existing: ::prost::alloc::string::String,
    /// proposed is the new value owner address for all of existing's scopes.
    #[prost(string, tag = "2")]
    pub proposed: ::prost::alloc::string::String,
    /// signers is the list of addresses of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgMigrateValueOwnerResponse is the response from migrating a value owner address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgMigrateValueOwnerResponse")]
pub struct MsgMigrateValueOwnerResponse {}
/// MsgWriteSessionRequest is the request type for the Msg/WriteSession RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteSessionRequest")]
pub struct MsgWriteSessionRequest {
    /// session is the Session you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SessionIDComponents is an optional (alternate) way of defining what the session_id should be in the provided
    /// session. If provided, it must have both a scope and session_uuid. Those components will be used to create the
    /// MetadataAddress for the session which will override the session_id in the provided session. If not provided (or
    /// all empty), nothing special happens.
    /// If there is a value in session.session_id that is different from the one created from these components, an error is
    /// returned.
    #[prost(message, optional, tag = "3")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    /// spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be used to generate the MetadataAddress for the contract specification which will override the
    /// specification_id in the provided session. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in session.specification_id that is different from the one created from this uuid, an error is
    /// returned.
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// SessionIDComponents contains fields for the components that make up a session id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.SessionIdComponents")]
pub struct SessionIdComponents {
    /// session_uuid is a uuid string for identifying this session, e.g. "5803f8bc-6067-4eb5-951f-2121671c2ec0"
    #[prost(string, tag = "3")]
    pub session_uuid: ::prost::alloc::string::String,
    /// scope is used to define the scope this session belongs to.
    #[prost(oneof = "session_id_components::ScopeIdentifier", tags = "1, 2")]
    pub scope_identifier: ::core::option::Option<session_id_components::ScopeIdentifier>,
}
/// Nested message and enum types in `SessionIdComponents`.
pub mod session_id_components {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// scope is used to define the scope this session belongs to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum ScopeIdentifier {
        /// scope_uuid is the uuid string for the scope, e.g. "91978ba2-5f35-459a-86a7-feca1b0512e0"
        #[prost(string, tag = "1")]
        ScopeUuid(::prost::alloc::string::String),
        /// scope_addr is the bech32 address string for the scope, g.g. "scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel"
        #[prost(string, tag = "2")]
        ScopeAddr(::prost::alloc::string::String),
    }
}
/// MsgWriteSessionResponse is the response type for the Msg/WriteSession RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteSessionResponse")]
pub struct MsgWriteSessionResponse {
    /// session_id_info contains information about the id/address of the session that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
}
/// MsgWriteRecordRequest is the request type for the Msg/WriteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteRecordRequest")]
pub struct MsgWriteRecordRequest {
    /// record is the Record you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SessionIDComponents is an optional (alternate) way of defining what the session_id should be in the provided
    /// record. If provided, it must have both a scope and session_uuid. Those components will be used to create the
    /// MetadataAddress for the session which will override the session_id in the provided record. If not provided (or
    /// all empty), nothing special happens.
    /// If there is a value in record.session_id that is different from the one created from these components, an error is
    /// returned.
    #[prost(message, optional, tag = "3")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    /// contract_spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be combined with the record name to generate the MetadataAddress for the record specification
    /// which will override the specification_id in the provided record. If not provided (or it is an empty string),
    /// nothing special happens.
    /// If there is a value in record.specification_id that is different from the one created from this uuid and
    /// record.name, an error is returned.
    #[prost(string, tag = "4")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
    /// parties is the list of parties involved with this record.
    /// Deprecated: This field is ignored. The parties are identified in the session and as signers.
    #[prost(message, repeated, tag = "5")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
}
/// MsgWriteRecordResponse is the response type for the Msg/WriteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteRecordResponse")]
pub struct MsgWriteRecordResponse {
    /// record_id_info contains information about the id/address of the record that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
}
/// MsgDeleteRecordRequest is the request type for the Msg/DeleteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteRecordRequest")]
pub struct MsgDeleteRecordRequest {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteRecordResponse is the response type for the Msg/DeleteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteRecordResponse")]
pub struct MsgDeleteRecordResponse {}
/// MsgWriteScopeSpecificationRequest is the request type for the Msg/WriteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteScopeSpecificationRequest")]
pub struct MsgWriteScopeSpecificationRequest {
    /// specification is the ScopeSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spec_uuid is an optional scope specification uuid string, e.g. "dc83ea70-eacd-40fe-9adf-1cf6148bf8a2"
    /// If provided, it will be used to generate the MetadataAddress for the scope specification which will override the
    /// specification_id in the provided specification. If not provided (or it is an empty string), nothing special
    /// happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid, an
    /// error is returned.
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteScopeSpecificationResponse is the response type for the Msg/WriteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteScopeSpecificationResponse")]
pub struct MsgWriteScopeSpecificationResponse {
    /// scope_spec_id_info contains information about the id/address of the scope specification that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// MsgDeleteScopeSpecificationRequest is the request type for the Msg/DeleteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeSpecificationRequest")]
pub struct MsgDeleteScopeSpecificationRequest {
    /// MetadataAddress for the scope specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeSpecificationResponse is the response type for the Msg/DeleteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteScopeSpecificationResponse")]
pub struct MsgDeleteScopeSpecificationResponse {}
/// MsgWriteContractSpecificationRequest is the request type for the Msg/WriteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteContractSpecificationRequest")]
pub struct MsgWriteContractSpecificationRequest {
    /// specification is the ContractSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be used to generate the MetadataAddress for the contract specification which will override the
    /// specification_id in the provided specification. If not provided (or it is an empty string), nothing special
    /// happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid, an
    /// error is returned.
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteContractSpecificationResponse is the response type for the Msg/WriteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteContractSpecificationResponse")]
pub struct MsgWriteContractSpecificationResponse {
    /// contract_spec_id_info contains information about the id/address of the contract specification that was added or
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// MsgAddContractSpecToScopeSpecRequest is the request type for the Msg/AddContractSpecToScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddContractSpecToScopeSpecRequest")]
pub struct MsgAddContractSpecToScopeSpecRequest {
    /// MetadataAddress for the contract specification to add.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    /// MetadataAddress for the scope specification to add contract specification to.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddContractSpecToScopeSpecResponse is the response type for the Msg/AddContractSpecToScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddContractSpecToScopeSpecResponse")]
pub struct MsgAddContractSpecToScopeSpecResponse {}
/// MsgDeleteContractSpecFromScopeSpecRequest is the request type for the Msg/DeleteContractSpecFromScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteContractSpecFromScopeSpecRequest")]
pub struct MsgDeleteContractSpecFromScopeSpecRequest {
    /// MetadataAddress for the contract specification to add.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    /// MetadataAddress for the scope specification to add contract specification to.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteContractSpecFromScopeSpecResponse is the response type for the Msg/DeleteContractSpecFromScopeSpec RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteContractSpecFromScopeSpecResponse")]
pub struct MsgDeleteContractSpecFromScopeSpecResponse {}
/// MsgDeleteContractSpecificationRequest is the request type for the Msg/DeleteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteContractSpecificationRequest")]
pub struct MsgDeleteContractSpecificationRequest {
    /// MetadataAddress for the contract specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteContractSpecificationResponse is the response type for the Msg/DeleteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteContractSpecificationResponse")]
pub struct MsgDeleteContractSpecificationResponse {}
/// MsgWriteRecordSpecificationRequest is the request type for the Msg/WriteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteRecordSpecificationRequest")]
pub struct MsgWriteRecordSpecificationRequest {
    /// specification is the RecordSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract_spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be combined with the record specification name to generate the MetadataAddress for the record
    /// specification which will override the specification_id in the provided specification. If not provided (or it is an
    /// empty string), nothing special happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid and
    /// specification.name, an error is returned.
    #[prost(string, tag = "3")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteRecordSpecificationResponse is the response type for the Msg/WriteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteRecordSpecificationResponse")]
pub struct MsgWriteRecordSpecificationResponse {
    /// record_spec_id_info contains information about the id/address of the record specification that was added or
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// MsgDeleteRecordSpecificationRequest is the request type for the Msg/DeleteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteRecordSpecificationRequest")]
pub struct MsgDeleteRecordSpecificationRequest {
    /// MetadataAddress for the record specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteRecordSpecificationResponse is the response type for the Msg/DeleteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteRecordSpecificationResponse")]
pub struct MsgDeleteRecordSpecificationResponse {}
/// MsgBindOSLocatorRequest is the request type for the Msg/BindOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgBindOSLocatorRequest")]
pub struct MsgBindOsLocatorRequest {
    /// The object locator to bind the address to bind to the URI.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgBindOSLocatorResponse is the response type for the Msg/BindOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgBindOSLocatorResponse")]
pub struct MsgBindOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgDeleteOSLocatorRequest is the request type for the Msg/DeleteOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteOSLocatorRequest")]
pub struct MsgDeleteOsLocatorRequest {
    /// The record being removed
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgDeleteOSLocatorResponse is the response type for the Msg/DeleteOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgDeleteOSLocatorResponse")]
pub struct MsgDeleteOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgModifyOSLocatorRequest is the request type for the Msg/ModifyOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgModifyOSLocatorRequest")]
pub struct MsgModifyOsLocatorRequest {
    /// The object locator to bind the address to bind to the URI.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgModifyOSLocatorResponse is the response type for the Msg/ModifyOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgModifyOSLocatorResponse")]
pub struct MsgModifyOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgSetAccountDataRequest is the request to set/update/delete a scope's account data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgSetAccountDataRequest")]
pub struct MsgSetAccountDataRequest {
    /// The identifier to associate the data with.
    /// Currently, only scope ids are supported.
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub metadata_addr: ::prost::alloc::vec::Vec<u8>,
    /// The desired accountdata value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// The signers of this message. Must fulfill owner requirements of the scope.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSetAccountDataResponse is the response from setting/updating/deleting a scope's account data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgSetAccountDataResponse")]
pub struct MsgSetAccountDataResponse {}
/// MsgWriteP8eContractSpecRequest has been deprecated and is no longer usable.
/// Deprecated: This message is no longer part of any endpoint and cannot be used for anything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteP8eContractSpecRequest")]
#[deprecated]
pub struct MsgWriteP8eContractSpecRequest {
    #[prost(message, optional, tag = "1")]
    pub contractspec: ::core::option::Option<p8e::ContractSpec>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgWriteP8eContractSpecResponse  has been deprecated and is no longer usable.
/// Deprecated: This message is no longer part of any endpoint and cannot be used for anything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgWriteP8eContractSpecResponse")]
#[deprecated]
pub struct MsgWriteP8eContractSpecResponse {
    #[prost(message, optional, tag = "1")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
    #[prost(message, repeated, tag = "2")]
    pub record_spec_id_infos: ::prost::alloc::vec::Vec<RecordSpecIdInfo>,
}
/// MsgP8eMemorializeContractRequest  has been deprecated and is no longer usable.
/// Deprecated: This message is no longer part of any endpoint and cannot be used for anything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgP8eMemorializeContractRequest")]
#[deprecated]
pub struct MsgP8eMemorializeContractRequest {
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scope_specification_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub recitals: ::core::option::Option<p8e::Recitals>,
    #[prost(message, optional, tag = "5")]
    pub contract: ::core::option::Option<p8e::Contract>,
    #[prost(message, optional, tag = "6")]
    pub signatures: ::core::option::Option<p8e::SignatureSet>,
    #[prost(string, tag = "7")]
    pub invoker: ::prost::alloc::string::String,
}
/// MsgP8eMemorializeContractResponse  has been deprecated and is no longer usable.
/// Deprecated: This message is no longer part of any endpoint and cannot be used for anything.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgP8eMemorializeContractResponse")]
#[deprecated]
pub struct MsgP8eMemorializeContractResponse {
    #[prost(message, optional, tag = "1")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
    #[prost(message, optional, tag = "2")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
    #[prost(message, repeated, tag = "3")]
    pub record_id_infos: ::prost::alloc::vec::Vec<RecordIdInfo>,
}
/// MsgAddNetAssetValuesRequest defines the Msg/AddNetAssetValues request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddNetAssetValuesRequest")]
pub struct MsgAddNetAssetValuesRequest {
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub net_asset_values: ::prost::alloc::vec::Vec<NetAssetValue>,
}
/// MsgAddNetAssetValuesResponse defines the Msg/AddNetAssetValue response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.MsgAddNetAssetValuesResponse")]
pub struct MsgAddNetAssetValuesResponse {}
pub struct MetadataQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> MetadataQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(
        &self,
        include_request: bool,
    ) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest { include_request }.query(self.querier)
    }
    pub fn scope(
        &self,
        scope_id: ::prost::alloc::string::String,
        session_addr: ::prost::alloc::string::String,
        record_addr: ::prost::alloc::string::String,
        include_sessions: bool,
        include_records: bool,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<ScopeResponse, cosmwasm_std::StdError> {
        ScopeRequest {
            scope_id,
            session_addr,
            record_addr,
            include_sessions,
            include_records,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn scopes_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<ScopesAllResponse, cosmwasm_std::StdError> {
        ScopesAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn sessions(
        &self,
        scope_id: ::prost::alloc::string::String,
        session_id: ::prost::alloc::string::String,
        record_addr: ::prost::alloc::string::String,
        record_name: ::prost::alloc::string::String,
        include_scope: bool,
        include_records: bool,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<SessionsResponse, cosmwasm_std::StdError> {
        SessionsRequest {
            scope_id,
            session_id,
            record_addr,
            record_name,
            include_scope,
            include_records,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn sessions_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<SessionsAllResponse, cosmwasm_std::StdError> {
        SessionsAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn records(
        &self,
        record_addr: ::prost::alloc::string::String,
        scope_id: ::prost::alloc::string::String,
        session_id: ::prost::alloc::string::String,
        name: ::prost::alloc::string::String,
        include_scope: bool,
        include_sessions: bool,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<RecordsResponse, cosmwasm_std::StdError> {
        RecordsRequest {
            record_addr,
            scope_id,
            session_id,
            name,
            include_scope,
            include_sessions,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn records_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<RecordsAllResponse, cosmwasm_std::StdError> {
        RecordsAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn ownership(
        &self,
        address: ::prost::alloc::string::String,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<OwnershipResponse, cosmwasm_std::StdError> {
        OwnershipRequest {
            address,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn value_ownership(
        &self,
        address: ::prost::alloc::string::String,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<ValueOwnershipResponse, cosmwasm_std::StdError> {
        ValueOwnershipRequest {
            address,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn scope_specification(
        &self,
        specification_id: ::prost::alloc::string::String,
        include_contract_specs: bool,
        include_record_specs: bool,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<ScopeSpecificationResponse, cosmwasm_std::StdError> {
        ScopeSpecificationRequest {
            specification_id,
            include_contract_specs,
            include_record_specs,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn scope_specifications_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<ScopeSpecificationsAllResponse, cosmwasm_std::StdError> {
        ScopeSpecificationsAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn contract_specification(
        &self,
        specification_id: ::prost::alloc::string::String,
        include_record_specs: bool,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<ContractSpecificationResponse, cosmwasm_std::StdError> {
        ContractSpecificationRequest {
            specification_id,
            include_record_specs,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn contract_specifications_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<ContractSpecificationsAllResponse, cosmwasm_std::StdError> {
        ContractSpecificationsAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn record_specifications_for_contract_specification(
        &self,
        specification_id: ::prost::alloc::string::String,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<RecordSpecificationsForContractSpecificationResponse, cosmwasm_std::StdError> {
        RecordSpecificationsForContractSpecificationRequest {
            specification_id,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn record_specification(
        &self,
        specification_id: ::prost::alloc::string::String,
        name: ::prost::alloc::string::String,
        exclude_id_info: bool,
        include_request: bool,
    ) -> Result<RecordSpecificationResponse, cosmwasm_std::StdError> {
        RecordSpecificationRequest {
            specification_id,
            name,
            exclude_id_info,
            include_request,
        }
        .query(self.querier)
    }
    pub fn record_specifications_all(
        &self,
        exclude_id_info: bool,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<RecordSpecificationsAllResponse, cosmwasm_std::StdError> {
        RecordSpecificationsAllRequest {
            exclude_id_info,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_by_addr(
        &self,
        addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ) -> Result<GetByAddrResponse, cosmwasm_std::StdError> {
        GetByAddrRequest { addrs }.query(self.querier)
    }
    pub fn os_locator_params(
        &self,
        include_request: bool,
    ) -> Result<OsLocatorParamsResponse, cosmwasm_std::StdError> {
        OsLocatorParamsRequest { include_request }.query(self.querier)
    }
    pub fn os_locator(
        &self,
        owner: ::prost::alloc::string::String,
        include_request: bool,
    ) -> Result<OsLocatorResponse, cosmwasm_std::StdError> {
        OsLocatorRequest {
            owner,
            include_request,
        }
        .query(self.querier)
    }
    pub fn os_locators_by_uri(
        &self,
        uri: ::prost::alloc::string::String,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<OsLocatorsByUriResponse, cosmwasm_std::StdError> {
        OsLocatorsByUriRequest {
            uri,
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn os_locators_by_scope(
        &self,
        scope_id: ::prost::alloc::string::String,
        include_request: bool,
    ) -> Result<OsLocatorsByScopeResponse, cosmwasm_std::StdError> {
        OsLocatorsByScopeRequest {
            scope_id,
            include_request,
        }
        .query(self.querier)
    }
    pub fn os_all_locators(
        &self,
        include_request: bool,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<OsAllLocatorsResponse, cosmwasm_std::StdError> {
        OsAllLocatorsRequest {
            include_request,
            pagination,
        }
        .query(self.querier)
    }
    pub fn account_data(
        &self,
        metadata_addr: ::prost::alloc::vec::Vec<u8>,
    ) -> Result<AccountDataResponse, cosmwasm_std::StdError> {
        AccountDataRequest { metadata_addr }.query(self.querier)
    }
    pub fn scope_net_asset_values(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryScopeNetAssetValuesResponse, cosmwasm_std::StdError> {
        QueryScopeNetAssetValuesRequest { id }.query(self.querier)
    }
}
