use provwasm_proc_macro::CosmwasmExt;
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
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "scopeID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "scopeID_scope_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub scope_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub scope_uuid: ::prost::alloc::string::String,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "sessionID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "sessionID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "sessionID_scope_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(alias = "sessionID_session_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id_session_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    #[serde(alias = "scopeID_info")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "recordID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "recordID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "recordID_scope_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(alias = "recordID_hashed_name")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    #[serde(alias = "scopeID_info")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scope_specID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "scope_specID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "scope_specID_scope_spec_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_spec_id_scope_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub scope_spec_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub scope_spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "contract_specID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "contract_specID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "contract_specID_contract_spec_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub contract_spec_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "record_specID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub record_spec_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "record_specID_prefix")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(alias = "record_specID_contract_spec_uuid")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(alias = "record_specID_hashed_name")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_spec_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub record_spec_addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    #[serde(alias = "contract_specID_info")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "4")]
    #[serde(
        serialize_with = "PartyType::serialize_vec",
        deserialize_with = "PartyType::deserialize_vec"
    )]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    #[serde(alias = "contract_specIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes_vec::serialize",
        deserialize_with = "crate::serde::as_str_bytes_vec::deserialize"
    )]
    pub contract_spec_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "4")]
    #[serde(
        serialize_with = "PartyType::serialize_vec",
        deserialize_with = "PartyType::deserialize_vec"
    )]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "7")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(oneof = "contract_specification::Source", tags = "5, 6")]
    pub source: ::core::option::Option<contract_specification::Source>,
}
/// Nested message and enum types in `ContractSpecification`.
pub mod contract_specification {
    use provwasm_proc_macro::CosmwasmExt;
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
        #[prost(bytes, tag = "5")]
        ResourceId(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "6")]
        Hash(::prost::alloc::string::String),
    }
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<InputSpecification>,
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    #[prost(enumeration = "DefinitionType", tag = "5")]
    #[serde(
        serialize_with = "DefinitionType::serialize",
        deserialize_with = "DefinitionType::deserialize"
    )]
    pub result_type: i32,
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "6")]
    #[serde(
        serialize_with = "PartyType::serialize_vec",
        deserialize_with = "PartyType::deserialize_vec"
    )]
    pub responsible_parties: ::prost::alloc::vec::Vec<i32>,
}
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
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub type_name: ::prost::alloc::string::String,
    #[prost(oneof = "input_specification::Source", tags = "3, 4")]
    pub source: ::core::option::Option<input_specification::Source>,
}
/// Nested message and enum types in `InputSpecification`.
pub mod input_specification {
    use provwasm_proc_macro::CosmwasmExt;
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
        #[prost(bytes, tag = "3")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "4")]
        Hash(::prost::alloc::string::String),
    }
}
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
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub website_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub icon_url: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum DefinitionType {
    Unspecified = 0,
    Proposed = 1,
    Record = 2,
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

    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::try_from(*v);
        match enum_value {
            Ok(v) => serializer.serialize_str(v.as_str_name()),
            Err(e) => Err(serde::ser::Error::custom(e)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum PartyType {
    Unspecified = 0,
    Originator = 1,
    Servicer = 2,
    Investor = 3,
    Custodian = 4,
    Owner = 5,
    Affiliate = 6,
    Omnibus = 7,
    Provenance = 8,
    Controller = 10,
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

    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::try_from(*v);
        match enum_value {
            Ok(v) => serializer.serialize_str(v.as_str_name()),
            Err(e) => Err(serde::ser::Error::custom(e)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }

    pub fn serialize_vec<S>(v: &Vec<i32>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut enum_strs: Vec<&str> = Vec::new();
        for ord in v {
            // let enum_value = Self::try_from(*ord);
            let enum_value = Self::try_from(*ord);
            match enum_value {
                Ok(v) => {
                    enum_strs.push(v.as_str_name());
                }
                Err(e) => return Err(serde::ser::Error::custom(e)),
            }
        }
        let mut seq = serializer.serialize_tuple(enum_strs.len())?;
        for item in enum_strs {
            seq.serialize_element(item)?;
        }
        seq.end()
    }

    fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Deserialize, Error};

        let strs: Vec<String> = Vec::deserialize(deserializer)?;
        let mut ords: Vec<i32> = Vec::new();
        for str_name in strs {
            let enum_value = Self::from_str_name(&str_name)
                .ok_or_else(|| Error::custom(format!("unknown enum string: {}", str_name)))?;
            ords.push(enum_value as i32);
        }
        Ok(ords)
    }
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    #[prost(string, repeated, tag = "4")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub value_owner_address: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub require_party_rollup: bool,
}
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
    #[serde(alias = "sessionID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub context: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "99")]
    pub audit: ::core::option::Option<AuditFields>,
}
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
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "sessionID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub process: ::core::option::Option<Process>,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<RecordInput>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<RecordOutput>,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
}
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
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    #[prost(oneof = "process::ProcessId", tags = "1, 2")]
    #[serde(alias = "processID")]
    pub process_id: ::core::option::Option<process::ProcessId>,
}
/// Nested message and enum types in `Process`.
pub mod process {
    use provwasm_proc_macro::CosmwasmExt;
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
        #[prost(string, tag = "1")]
        Address(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Hash(::prost::alloc::string::String),
    }
}
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
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    #[prost(enumeration = "RecordInputStatus", tag = "5")]
    #[serde(
        serialize_with = "RecordInputStatus::serialize",
        deserialize_with = "RecordInputStatus::deserialize"
    )]
    pub status: i32,
    #[prost(oneof = "record_input::Source", tags = "2, 3")]
    pub source: ::core::option::Option<record_input::Source>,
}
/// Nested message and enum types in `RecordInput`.
pub mod record_input {
    use provwasm_proc_macro::CosmwasmExt;
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
        #[prost(bytes, tag = "2")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "3")]
        Hash(::prost::alloc::string::String),
    }
}
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
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(enumeration = "ResultStatus", tag = "2")]
    #[serde(
        serialize_with = "ResultStatus::serialize",
        deserialize_with = "ResultStatus::deserialize"
    )]
    pub status: i32,
}
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
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "PartyType", tag = "2")]
    #[serde(
        serialize_with = "PartyType::serialize",
        deserialize_with = "PartyType::deserialize"
    )]
    pub role: i32,
    #[prost(bool, tag = "3")]
    pub optional: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub created_date: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "2")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub updated_date: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "4")]
    pub updated_by: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u32,
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum RecordInputStatus {
    Unspecified = 0,
    Proposed = 1,
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

    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::try_from(*v);
        match enum_value {
            Ok(v) => serializer.serialize_str(v.as_str_name()),
            Err(e) => Err(serde::ser::Error::custom(e)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ResultStatus {
    Unspecified = 0,
    Pass = 1,
    Skip = 2,
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

    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::try_from(*v);
        match enum_value {
            Ok(v) => serializer.serialize_str(v.as_str_name()),
            Err(e) => Err(serde::ser::Error::custom(e)),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
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
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub locator_uri: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encryption_key: ::prost::alloc::string::String,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<QueryParamsRequest>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "scopeID")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub include_sessions: bool,
    #[prost(bool, tag = "11")]
    pub include_records: bool,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "scopeID_info")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "scope_specID_info")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "scopeID")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "sessionID")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub record_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    #[prost(bool, tag = "11")]
    pub include_records: bool,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<SessionsRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "sessionID_info")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "contract_specID_info")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
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
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "scopeID")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "sessionID")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    #[prost(bool, tag = "11")]
    pub include_sessions: bool,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordsRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "recordID_info")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "record_specID_info")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OwnershipRequest>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ValueOwnershipRequest>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "specificationID")]
    pub specification_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub include_contract_specs: bool,
    #[prost(bool, tag = "11")]
    pub include_record_specs: bool,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope_specification: ::core::option::Option<ScopeSpecificationWrapper>,
    #[prost(message, repeated, tag = "2")]
    pub contract_specs: ::prost::alloc::vec::Vec<ContractSpecificationWrapper>,
    #[prost(message, repeated, tag = "3")]
    pub record_specs: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeSpecificationRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "scope_specID_info")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "specificationID")]
    pub specification_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "10")]
    pub include_record_specs: bool,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub contract_specification: ::core::option::Option<ContractSpecificationWrapper>,
    #[prost(message, repeated, tag = "3")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ContractSpecificationRequest>,
}

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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "contract_specID_info")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "specificationID")]
    pub specification_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, repeated, tag = "1")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    #[prost(string, tag = "2")]
    pub contract_specification_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_specification_addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationsForContractSpecificationRequest>,
}
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
    #[prost(string, tag = "1")]
    #[serde(alias = "specificationID")]
    pub specification_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "12")]
    #[serde(alias = "excludeID_info")]
    pub exclude_id_info: bool,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub record_specification: ::core::option::Option<RecordSpecificationWrapper>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "record_specID_info")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<OsLocatorParams>,
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorParamsRequest>,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorRequest>,
}
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
    #[prost(bool, tag = "98")]
    pub include_request: bool,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByUriRequest>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
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
    #[serde(alias = "scopeID")]
    pub scope_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "98")]
    pub include_request: bool,
}
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
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByScopeRequest>,
}
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
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub scope_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub usd_mills: u64,
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "scopeID_info")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "scopeID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", repeated, tag = "1")]
    #[serde(alias = "scopeIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes_vec::serialize",
        deserialize_with = "crate::serde::as_str_bytes_vec::deserialize"
    )]
    pub scope_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "2")]
    pub value_owner_address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(string, tag = "1")]
    pub existing: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub proposed: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "sessionID_components")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(string, tag = "3")]
    pub session_uuid: ::prost::alloc::string::String,
    #[prost(oneof = "session_id_components::ScopeIdentifier", tags = "1, 2")]
    #[serde(alias = "scopeIDentifier")]
    pub scope_identifier: ::core::option::Option<session_id_components::ScopeIdentifier>,
}
/// Nested message and enum types in `SessionIdComponents`.
pub mod session_id_components {
    use provwasm_proc_macro::CosmwasmExt;
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
        #[prost(string, tag = "1")]
        ScopeUuid(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        ScopeAddr(::prost::alloc::string::String),
    }
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "sessionID_info")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
}
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
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "sessionID_components")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    #[prost(string, tag = "4")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "recordID_info")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
}
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
    #[serde(alias = "recordID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "scope_specID_info")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "contract_specID_info")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "contract_specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "scope_specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "contract_specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(alias = "scope_specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "record_specID_info")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
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
    #[prost(bytes = "vec", tag = "1")]
    #[serde(alias = "specificationID")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
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
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
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
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
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
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
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
}
