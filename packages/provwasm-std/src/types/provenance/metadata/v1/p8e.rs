use provwasm_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Contract")]
pub struct Contract {
    #[prost(message, optional, tag = "1")]
    pub definition: ::core::option::Option<DefinitionSpec>,
    /// Points to the proto for the contractSpec
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<Fact>,
    /// Invoker of this contract
    #[prost(message, optional, tag = "3")]
    pub invoker: ::core::option::Option<SigningAndEncryptionPublicKeys>,
    /// Constructor arguments.
    /// These are always the output of a previously recorded consideration.
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<Fact>,
    /// conditions is a deprecated field that is not used at all anymore.
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    #[prost(message, repeated, tag = "6")]
    pub considerations: ::prost::alloc::vec::Vec<Consideration>,
    #[prost(message, repeated, tag = "7")]
    pub recitals: ::prost::alloc::vec::Vec<Recital>,
    #[prost(int32, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub times_executed: i32,
    /// This is only set once when the contract is initially executed
    #[prost(message, optional, tag = "9")]
    pub start_time: ::core::option::Option<Timestamp>,
    #[prost(bytes = "vec", tag = "10")]
    pub context: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.DefinitionSpec")]
pub struct DefinitionSpec {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource_location: ::core::option::Option<Location>,
    #[prost(message, optional, tag = "3")]
    pub signature: ::core::option::Option<Signature>,
    #[prost(enumeration = "DefinitionSpecType", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Fact")]
pub struct Fact {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub data_location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Condition")]
pub struct Condition {
    #[prost(string, tag = "1")]
    pub condition_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<ExecutionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Consideration")]
pub struct Consideration {
    #[prost(string, tag = "1")]
    pub consideration_name: ::prost::alloc::string::String,
    /// Data pushed to a consideration that will ultimately match the output_spec of the consideration
    #[prost(message, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<ProposedFact>,
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<ExecutionResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ProposedFact")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ExecutionResult")]
pub struct ExecutionResult {
    #[prost(message, optional, tag = "1")]
    pub output: ::core::option::Option<ProposedFact>,
    #[prost(enumeration = "ExecutionResultType", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
    #[prost(message, optional, tag = "3")]
    pub recorded_at: ::core::option::Option<Timestamp>,
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Recitals")]
pub struct Recitals {
    #[prost(message, repeated, tag = "1")]
    pub parties: ::prost::alloc::vec::Vec<Recital>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Recital")]
pub struct Recital {
    #[prost(enumeration = "PartyType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub signer_role: i32,
    #[prost(message, optional, tag = "2")]
    pub signer: ::core::option::Option<SigningAndEncryptionPublicKeys>,
    #[prost(bytes = "vec", tag = "3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Location")]
pub struct Location {
    #[prost(message, optional, tag = "1")]
    pub r#ref: ::core::option::Option<ProvenanceReference>,
    #[prost(string, tag = "2")]
    pub classname: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ProvenanceReference")]
pub struct ProvenanceReference {
    /// \[Req\] \[Scope.uuid\]
    /// Scope ID
    #[prost(message, optional, tag = "1")]
    pub scope_uuid: ::core::option::Option<Uuid>,
    /// \[Opt\] \[RecordGroup.group_uuid\]
    /// require record to be within a specific group
    #[prost(message, optional, tag = "2")]
    pub group_uuid: ::core::option::Option<Uuid>,
    /// \[Opt\] \[Record.result_hash\]
    /// specify a specific record inside a scope (and group) by result-hash
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    /// \[Opt\] \[Record.result_name\]
    /// specify a result-name of a record within a scope
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.SignatureSet")]
pub struct SignatureSet {
    #[prost(message, repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Signature")]
pub struct Signature {
    /// Signature Detail
    #[prost(string, tag = "1")]
    pub algo: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub provider: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
    /// Identity of signer
    #[prost(message, optional, tag = "4")]
    pub signer: ::core::option::Option<SigningAndEncryptionPublicKeys>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.SigningAndEncryptionPublicKeys")]
pub struct SigningAndEncryptionPublicKeys {
    #[prost(message, optional, tag = "1")]
    pub signing_public_key: ::core::option::Option<PublicKey>,
    #[prost(message, optional, tag = "2")]
    pub encryption_public_key: ::core::option::Option<PublicKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.PublicKey")]
pub struct PublicKey {
    #[prost(bytes = "vec", tag = "1")]
    pub public_key_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "PublicKeyType", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    #[prost(enumeration = "PublicKeyCurve", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub curve: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.UUID")]
pub struct Uuid {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// A Timestamp represents a point in time using values relative to the epoch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.Timestamp")]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution.
    #[prost(int32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub nanos: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ContractSpec")]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ConditionSpec")]
pub struct ConditionSpec {
    #[prost(string, tag = "1")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub input_specs: ::prost::alloc::vec::Vec<DefinitionSpec>,
    #[prost(message, optional, tag = "3")]
    pub output_spec: ::core::option::Option<OutputSpec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.ConsiderationSpec")]
pub struct ConsiderationSpec {
    #[prost(string, tag = "1")]
    pub func_name: ::prost::alloc::string::String,
    /// Invoking party
    #[prost(enumeration = "PartyType", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub responsible_party: i32,
    #[prost(message, repeated, tag = "3")]
    pub input_specs: ::prost::alloc::vec::Vec<DefinitionSpec>,
    #[prost(message, optional, tag = "4")]
    pub output_spec: ::core::option::Option<OutputSpec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.metadata.v1.p8e.OutputSpec")]
pub struct OutputSpec {
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<DefinitionSpec>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DefinitionSpecType {
    Unknown = 0,
    Proposed = 1,
    Fact = 2,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyCurve {
    Secp256k1 = 0,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyType {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionResultType {
    ResultTypeUnknown = 0,
    ResultTypePass = 1,
    /// Couldn't process the condition/consideration due to missing facts being generated by other considerations.
    ResultTypeSkip = 2,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartyType {
    Unknown = 0,
    Originator = 1,
    Servicer = 2,
    Investor = 3,
    Custodian = 4,
    Owner = 5,
    Affiliate = 6,
    Omnibus = 7,
    Provenance = 8,
    Marker = 9,
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
