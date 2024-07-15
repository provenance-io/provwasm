use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Contract")]
#[deprecated]
pub struct Contract {
    #[prost(message, optional, tag = "1")]
    pub definition: ::core::option::Option<DefinitionSpec>,
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<Fact>,
    #[prost(message, optional, tag = "3")]
    pub invoker: ::core::option::Option<SigningAndEncryptionPublicKeys>,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<Fact>,
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    #[prost(message, repeated, tag = "6")]
    pub considerations: ::prost::alloc::vec::Vec<Consideration>,
    #[prost(message, repeated, tag = "7")]
    pub recitals: ::prost::alloc::vec::Vec<Recital>,
    #[prost(int32, tag = "8")]
    pub times_executed: i32,
    #[prost(message, optional, tag = "9")]
    pub start_time: ::core::option::Option<Timestamp>,
    #[prost(bytes = "vec", tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str_bytes::serialize",
        deserialize_with = "crate::serde::as_str_bytes::deserialize"
    )]
    pub context: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.DefinitionSpec")]
#[deprecated]
pub struct DefinitionSpec {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource_location: ::core::option::Option<Location>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
    #[prost(enumeration = "DefinitionSpecType", tag = "4")]
    #[serde(
        serialize_with = "DefinitionSpecType::serialize",
        deserialize_with = "DefinitionSpecType::deserialize"
    )]
    pub r#type: i32,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Fact")]
#[deprecated]
pub struct Fact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data_location: ::core::option::Option<Location>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Condition")]
#[deprecated]
pub struct Condition {
    #[prost(string, tag = "1")]
    pub condition_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ExecutionResult>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Consideration")]
#[deprecated]
pub struct Consideration {
    #[prost(string, tag = "1")]
    pub consideration_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<ProposedFact>,
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<ExecutionResult>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ProposedFact")]
#[deprecated]
pub struct ProposedFact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub classname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub ancestor: ::core::option::Option<ProvenanceReference>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ExecutionResult")]
#[deprecated]
pub struct ExecutionResult {
    #[prost(message, optional, tag = "1")]
    pub output: ::core::option::Option<ProposedFact>,
    #[prost(enumeration = "ExecutionResultType", tag = "2")]
    #[serde(
        serialize_with = "ExecutionResultType::serialize",
        deserialize_with = "ExecutionResultType::deserialize"
    )]
    pub result: i32,
    #[prost(message, optional, tag = "3")]
    pub recorded_at: ::core::option::Option<Timestamp>,
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Recitals")]
#[deprecated]
pub struct Recitals {
    #[prost(message, repeated, tag = "1")]
    pub parties: ::prost::alloc::vec::Vec<Recital>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Recital")]
#[deprecated]
pub struct Recital {
    #[prost(enumeration = "PartyType", tag = "1")]
    #[serde(
        serialize_with = "PartyType::serialize",
        deserialize_with = "PartyType::deserialize"
    )]
    pub signer_role: i32,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<SigningAndEncryptionPublicKeys>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Location")]
#[deprecated]
pub struct Location {
    #[prost(message, optional, tag = "1")]
    pub r#ref: ::core::option::Option<ProvenanceReference>,
    #[prost(string, tag = "2")]
    pub classname: ::prost::alloc::string::String,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ProvenanceReference")]
#[deprecated]
pub struct ProvenanceReference {
    #[prost(message, optional, tag = "1")]
    pub scope_uuid: ::core::option::Option<Uuid>,
    #[prost(message, optional, tag = "2")]
    pub group_uuid: ::core::option::Option<Uuid>,
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.SignatureSet")]
#[deprecated]
pub struct SignatureSet {
    #[prost(message, repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Signature")]
#[deprecated]
pub struct Signature {
    #[prost(string, tag = "1")]
    pub algo: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub signer: ::core::option::Option<SigningAndEncryptionPublicKeys>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.SigningAndEncryptionPublicKeys")]
#[deprecated]
pub struct SigningAndEncryptionPublicKeys {
    #[prost(message, optional, tag = "1")]
    pub signing_public_key: ::core::option::Option<PublicKey>,
    #[prost(message, optional, tag = "2")]
    pub encryption_public_key: ::core::option::Option<PublicKey>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.PublicKey")]
#[deprecated]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub public_key_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "PublicKeyType", tag = "2")]
    #[serde(
        serialize_with = "PublicKeyType::serialize",
        deserialize_with = "PublicKeyType::deserialize"
    )]
    pub r#type: i32,
    #[prost(enumeration = "PublicKeyCurve", tag = "3")]
    #[serde(
        serialize_with = "PublicKeyCurve::serialize",
        deserialize_with = "PublicKeyCurve::deserialize"
    )]
    pub curve: i32,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.UUID")]
#[deprecated]
pub struct Uuid {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Timestamp")]
#[deprecated]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ContractSpec")]
#[deprecated]
pub struct ContractSpec {
    #[prost(message, optional, tag = "1")]
    pub definition: ::core::option::Option<DefinitionSpec>,
    #[prost(message, repeated, tag = "2")]
    pub input_specs: ::prost::alloc::vec::Vec<DefinitionSpec>,
    #[prost(enumeration = "PartyType", repeated, tag = "3")]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "4")]
    pub condition_specs: ::prost::alloc::vec::Vec<ConditionSpec>,
    #[prost(message, repeated, tag = "5")]
    pub consideration_specs: ::prost::alloc::vec::Vec<ConsiderationSpec>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ConditionSpec")]
#[deprecated]
pub struct ConditionSpec {
    #[prost(string, tag = "1")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub input_specs: ::prost::alloc::vec::Vec<DefinitionSpec>,
    #[prost(message, optional, tag = "3")]
    pub output_spec: ::core::option::Option<OutputSpec>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ConsiderationSpec")]
#[deprecated]
pub struct ConsiderationSpec {
    #[prost(string, tag = "1")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(enumeration = "PartyType", tag = "2")]
    #[serde(
        serialize_with = "PartyType::serialize",
        deserialize_with = "PartyType::deserialize"
    )]
    pub responsible_party: i32,
    #[prost(message, repeated, tag = "3")]
    pub input_specs: ::prost::alloc::vec::Vec<DefinitionSpec>,
    #[prost(message, optional, tag = "4")]
    pub output_spec: ::core::option::Option<OutputSpec>,
}
/// Deprecated: Do not use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.OutputSpec")]
#[deprecated]
pub struct OutputSpec {
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<DefinitionSpec>,
}
/// Deprecated: Do not use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
#[deprecated]
pub enum DefinitionSpecType {
    /// Deprecated: Do not use.
    Unknown = 0,
    /// Deprecated: Do not use.
    Proposed = 1,
    /// Deprecated: Do not use.
    Fact = 2,
    /// Deprecated: Do not use.
    FactList = 3,
}
impl DefinitionSpecType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DefinitionSpecType::Unknown => "DEFINITION_SPEC_TYPE_UNKNOWN",
            DefinitionSpecType::Proposed => "DEFINITION_SPEC_TYPE_PROPOSED",
            DefinitionSpecType::Fact => "DEFINITION_SPEC_TYPE_FACT",
            DefinitionSpecType::FactList => "DEFINITION_SPEC_TYPE_FACT_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFINITION_SPEC_TYPE_UNKNOWN" => Some(Self::Unknown),
            "DEFINITION_SPEC_TYPE_PROPOSED" => Some(Self::Proposed),
            "DEFINITION_SPEC_TYPE_FACT" => Some(Self::Fact),
            "DEFINITION_SPEC_TYPE_FACT_LIST" => Some(Self::FactList),
            _ => None,
        }
    }
}
/// Deprecated: Do not use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
#[deprecated]
pub enum PublicKeyCurve {
    /// Deprecated: Do not use.
    Secp256k1 = 0,
    /// Deprecated: Do not use.
    P256 = 1,
}
impl PublicKeyCurve {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PublicKeyCurve::Secp256k1 => "SECP256K1",
            PublicKeyCurve::P256 => "P256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECP256K1" => Some(Self::Secp256k1),
            "P256" => Some(Self::P256),
            _ => None,
        }
    }
}
/// Deprecated: Do not use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
#[deprecated]
pub enum PublicKeyType {
    /// Deprecated: Do not use.
    Elliptic = 0,
}
impl PublicKeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PublicKeyType::Elliptic => "ELLIPTIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ELLIPTIC" => Some(Self::Elliptic),
            _ => None,
        }
    }
}
/// Deprecated: Do not use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
#[deprecated]
pub enum ExecutionResultType {
    /// Deprecated: Do not use.
    ResultTypeUnknown = 0,
    /// Deprecated: Do not use.
    ResultTypePass = 1,
    /// Deprecated: Do not use.
    ResultTypeSkip = 2,
    /// Deprecated: Do not use.
    ResultTypeFail = 3,
}
impl ExecutionResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionResultType::ResultTypeUnknown => "RESULT_TYPE_UNKNOWN",
            ExecutionResultType::ResultTypePass => "RESULT_TYPE_PASS",
            ExecutionResultType::ResultTypeSkip => "RESULT_TYPE_SKIP",
            ExecutionResultType::ResultTypeFail => "RESULT_TYPE_FAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESULT_TYPE_UNKNOWN" => Some(Self::ResultTypeUnknown),
            "RESULT_TYPE_PASS" => Some(Self::ResultTypePass),
            "RESULT_TYPE_SKIP" => Some(Self::ResultTypeSkip),
            "RESULT_TYPE_FAIL" => Some(Self::ResultTypeFail),
            _ => None,
        }
    }
}
/// Deprecated: Do not use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
#[deprecated]
pub enum PartyType {
    /// Deprecated: Do not use.
    Unknown = 0,
    /// Deprecated: Do not use.
    Originator = 1,
    /// Deprecated: Do not use.
    Servicer = 2,
    /// Deprecated: Do not use.
    Investor = 3,
    /// Deprecated: Do not use.
    Custodian = 4,
    /// Deprecated: Do not use.
    Owner = 5,
    /// Deprecated: Do not use.
    Affiliate = 6,
    /// Deprecated: Do not use.
    Omnibus = 7,
    /// Deprecated: Do not use.
    Provenance = 8,
    /// Deprecated: Do not use.
    Marker = 9,
    /// Deprecated: Do not use.
    Controller = 10,
    /// Deprecated: Do not use.
    Validator = 11,
}
impl PartyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PartyType::Unknown => "PARTY_TYPE_UNKNOWN",
            PartyType::Originator => "PARTY_TYPE_ORIGINATOR",
            PartyType::Servicer => "PARTY_TYPE_SERVICER",
            PartyType::Investor => "PARTY_TYPE_INVESTOR",
            PartyType::Custodian => "PARTY_TYPE_CUSTODIAN",
            PartyType::Owner => "PARTY_TYPE_OWNER",
            PartyType::Affiliate => "PARTY_TYPE_AFFILIATE",
            PartyType::Omnibus => "PARTY_TYPE_OMNIBUS",
            PartyType::Provenance => "PARTY_TYPE_PROVENANCE",
            PartyType::Marker => "PARTY_TYPE_MARKER",
            PartyType::Controller => "PARTY_TYPE_CONTROLLER",
            PartyType::Validator => "PARTY_TYPE_VALIDATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PARTY_TYPE_UNKNOWN" => Some(Self::Unknown),
            "PARTY_TYPE_ORIGINATOR" => Some(Self::Originator),
            "PARTY_TYPE_SERVICER" => Some(Self::Servicer),
            "PARTY_TYPE_INVESTOR" => Some(Self::Investor),
            "PARTY_TYPE_CUSTODIAN" => Some(Self::Custodian),
            "PARTY_TYPE_OWNER" => Some(Self::Owner),
            "PARTY_TYPE_AFFILIATE" => Some(Self::Affiliate),
            "PARTY_TYPE_OMNIBUS" => Some(Self::Omnibus),
            "PARTY_TYPE_PROVENANCE" => Some(Self::Provenance),
            "PARTY_TYPE_MARKER" => Some(Self::Marker),
            "PARTY_TYPE_CONTROLLER" => Some(Self::Controller),
            "PARTY_TYPE_VALIDATOR" => Some(Self::Validator),
            _ => None,
        }
    }
}
