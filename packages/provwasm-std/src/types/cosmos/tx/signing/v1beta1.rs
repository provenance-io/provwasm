#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptors")]
#[serde(rename_all = "snake_case")]
pub struct SignatureDescriptors {
    #[prost(message, repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<SignatureDescriptor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor")]
#[serde(rename_all = "snake_case")]
pub struct SignatureDescriptor {
    #[prost(message, optional, tag = "1")]
    pub public_key: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<signature_descriptor::Data>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
/// Nested message and enum types in `SignatureDescriptor`.
pub mod signature_descriptor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        ::prost::Message,
        schemars::JsonSchema,
        serde::Serialize,
        serde::Deserialize,
        provwasm_proc_macro::CosmwasmExt,
    )]
    #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data")]
    #[serde(rename_all = "snake_case")]
    pub struct Data {
        #[prost(oneof = "data::Sum", tags = "1, 2")]
        pub sum: ::core::option::Option<data::Sum>,
    }
    /// Nested message and enum types in `Data`.
    pub mod data {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(
            Clone,
            PartialEq,
            ::prost::Message,
            schemars::JsonSchema,
            serde::Serialize,
            serde::Deserialize,
            provwasm_proc_macro::CosmwasmExt,
        )]
        #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Single")]
        #[serde(rename_all = "snake_case")]
        pub struct Single {
            #[prost(enumeration = "super::super::SignMode", tag = "1")]
            #[serde(
                serialize_with = "super::super::SignMode::serialize",
                deserialize_with = "super::super::SignMode::deserialize"
            )]
            pub mode: i32,
            #[prost(bytes = "vec", tag = "2")]
            #[serde(
                serialize_with = "crate::serde::as_base64::serialize",
                deserialize_with = "crate::serde::as_base64::deserialize"
            )]
            pub signature: ::prost::alloc::vec::Vec<u8>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(
            Clone,
            PartialEq,
            ::prost::Message,
            schemars::JsonSchema,
            serde::Serialize,
            serde::Deserialize,
            provwasm_proc_macro::CosmwasmExt,
        )]
        #[proto_message(type_url = "/cosmos.tx.signing.v1beta1.SignatureDescriptor.Data.Multi")]
        #[serde(rename_all = "snake_case")]
        pub struct Multi {
            #[prost(message, optional, tag = "1")]
            pub bitarray: ::core::option::Option<
                super::super::super::super::super::crypto::multisig::v1beta1::CompactBitArray,
            >,
            #[prost(message, repeated, tag = "2")]
            pub signatures: ::prost::alloc::vec::Vec<super::Data>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(
            Clone,
            PartialEq,
            ::prost::Oneof,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
        )]
        #[serde(rename_all = "snake_case")]
        pub enum Sum {
            #[prost(message, tag = "1")]
            Single(Single),
            #[prost(message, tag = "2")]
            Multi(Multi),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SignMode {
    Unspecified = 0,
    Direct = 1,
    Textual = 2,
    DirectAux = 3,
    LegacyAminoJson = 127,
    Eip191 = 191,
}
impl SignMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignMode::Unspecified => "SIGN_MODE_UNSPECIFIED",
            SignMode::Direct => "SIGN_MODE_DIRECT",
            SignMode::Textual => "SIGN_MODE_TEXTUAL",
            SignMode::DirectAux => "SIGN_MODE_DIRECT_AUX",
            SignMode::LegacyAminoJson => "SIGN_MODE_LEGACY_AMINO_JSON",
            SignMode::Eip191 => "SIGN_MODE_EIP_191",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGN_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGN_MODE_DIRECT" => Some(Self::Direct),
            "SIGN_MODE_TEXTUAL" => Some(Self::Textual),
            "SIGN_MODE_DIRECT_AUX" => Some(Self::DirectAux),
            "SIGN_MODE_LEGACY_AMINO_JSON" => Some(Self::LegacyAminoJson),
            "SIGN_MODE_EIP_191" => Some(Self::Eip191),
            _ => None,
        }
    }
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
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
