#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.multisig.v1beta1.MultiSignature")]
#[serde(rename_all = "snake_case")]
pub struct MultiSignature {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_std_derive::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.crypto.multisig.v1beta1.CompactBitArray")]
#[serde(rename_all = "snake_case")]
pub struct CompactBitArray {
    #[prost(uint32, tag = "1")]
    pub extra_bits_stored: u32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub elems: ::prost::alloc::vec::Vec<u8>,
}
