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
#[proto_message(type_url = "/provenance.attribute.v1.Params")]
pub struct Params {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_value_length: u32,
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
#[proto_message(type_url = "/provenance.attribute.v1.Attribute")]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attribute_type: i32,
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub expiration_date: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeAdd")]
pub struct EventAttributeAdd {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub expiration: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeUpdate")]
pub struct EventAttributeUpdate {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub original_value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub original_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub update_value: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub update_type: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
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
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeDelete")]
pub struct EventAttributeDelete {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeDistinctDelete")]
pub struct EventAttributeDistinctDelete {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub attribute_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AttributeType {
    Unspecified = 0,
    Uuid = 1,
    Json = 2,
    String = 3,
    Uri = 4,
    Int = 5,
    Float = 6,
    Proto = 7,
    Bytes = 8,
}
impl AttributeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttributeType::Unspecified => "ATTRIBUTE_TYPE_UNSPECIFIED",
            AttributeType::Uuid => "ATTRIBUTE_TYPE_UUID",
            AttributeType::Json => "ATTRIBUTE_TYPE_JSON",
            AttributeType::String => "ATTRIBUTE_TYPE_STRING",
            AttributeType::Uri => "ATTRIBUTE_TYPE_URI",
            AttributeType::Int => "ATTRIBUTE_TYPE_INT",
            AttributeType::Float => "ATTRIBUTE_TYPE_FLOAT",
            AttributeType::Proto => "ATTRIBUTE_TYPE_PROTO",
            AttributeType::Bytes => "ATTRIBUTE_TYPE_BYTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ATTRIBUTE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ATTRIBUTE_TYPE_UUID" => Some(Self::Uuid),
            "ATTRIBUTE_TYPE_JSON" => Some(Self::Json),
            "ATTRIBUTE_TYPE_STRING" => Some(Self::String),
            "ATTRIBUTE_TYPE_URI" => Some(Self::Uri),
            "ATTRIBUTE_TYPE_INT" => Some(Self::Int),
            "ATTRIBUTE_TYPE_FLOAT" => Some(Self::Float),
            "ATTRIBUTE_TYPE_PROTO" => Some(Self::Proto),
            "ATTRIBUTE_TYPE_BYTES" => Some(Self::Bytes),
            _ => None,
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Attribute",
    response_type = QueryAttributeResponse
)]
pub struct QueryAttributeRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeResponse")]
pub struct QueryAttributeResponse {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(message, optional, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Attributes",
    response_type = QueryAttributesResponse
)]
pub struct QueryAttributesRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesResponse")]
pub struct QueryAttributesResponse {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(message, optional, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Scan",
    response_type = QueryScanResponse
)]
pub struct QueryScanRequest {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub suffix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanResponse")]
pub struct QueryScanResponse {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(message, optional, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeRequest")]
pub struct MsgAddAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attribute_type: i32,
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub expiration_date: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeResponse")]
pub struct MsgAddAttributeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeRequest")]
pub struct MsgUpdateAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub original_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub update_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub original_attribute_type: i32,
    #[prost(enumeration = "AttributeType", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_attribute_type: i32,
    #[prost(string, tag = "6")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeResponse")]
pub struct MsgUpdateAttributeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeRequest")]
pub struct MsgDeleteAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeResponse")]
pub struct MsgDeleteAttributeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteDistinctAttributeRequest")]
pub struct MsgDeleteDistinctAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteDistinctAttributeResponse")]
pub struct MsgDeleteDistinctAttributeResponse {}
pub struct AttributeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AttributeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn attribute(
        &self,
        account: ::prost::alloc::string::String,
        name: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAttributeResponse, cosmwasm_std::StdError> {
        QueryAttributeRequest {
            account,
            name,
            pagination,
        }
        .query(self.querier)
    }
    pub fn attributes(
        &self,
        account: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAttributesResponse, cosmwasm_std::StdError> {
        QueryAttributesRequest {
            account,
            pagination,
        }
        .query(self.querier)
    }
    pub fn scan(
        &self,
        account: ::prost::alloc::string::String,
        suffix: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryScanResponse, cosmwasm_std::StdError> {
        QueryScanRequest {
            account,
            suffix,
            pagination,
        }
        .query(self.querier)
    }
}
