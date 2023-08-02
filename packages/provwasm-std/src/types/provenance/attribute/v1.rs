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
#[proto_message(type_url = "/provenance.attribute.v1.Params")]
#[serde(rename_all = "snake_case")]
pub struct Params {
    #[prost(uint32, tag = "1")]
    pub max_value_length: u32,
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
#[proto_message(type_url = "/provenance.attribute.v1.Attribute")]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "AttributeType::serialize",
        deserialize_with = "AttributeType::deserialize"
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeAdd")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeUpdate")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeDelete")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeDistinctDelete")]
#[serde(rename_all = "snake_case")]
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
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeResponse")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesResponse")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanResponse")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "AttributeType::serialize",
        deserialize_with = "AttributeType::deserialize"
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgAddAttributeResponse {}
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub original_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub update_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "AttributeType", tag = "4")]
    #[serde(
        serialize_with = "AttributeType::serialize",
        deserialize_with = "AttributeType::deserialize"
    )]
    pub original_attribute_type: i32,
    #[prost(enumeration = "AttributeType", tag = "5")]
    #[serde(
        serialize_with = "AttributeType::serialize",
        deserialize_with = "AttributeType::deserialize"
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgUpdateAttributeResponse {}
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeRequest")]
#[serde(rename_all = "snake_case")]
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteAttributeResponse {}
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteDistinctAttributeRequest")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteDistinctAttributeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
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
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteDistinctAttributeResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgDeleteDistinctAttributeResponse {}
pub struct AttributeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AttributeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn attribute(
        &self,
        account: ::prost::alloc::string::String,
        name: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> std::result::Result<QueryAttributeResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryAttributesResponse, cosmwasm_std::StdError> {
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
    ) -> std::result::Result<QueryScanResponse, cosmwasm_std::StdError> {
        QueryScanRequest {
            account,
            suffix,
            pagination,
        }
        .query(self.querier)
    }
}
