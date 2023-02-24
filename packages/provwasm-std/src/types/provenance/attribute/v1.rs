use provwasm_std_derive::CosmwasmExt;
/// Params defines the set of params for the attribute module.
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
#[proto_message(type_url = "/provenance.attribute.v1.Params")]
pub struct Params {
    /// maximum length of data to allow in an attribute value
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_value_length: u32,
}
/// Attribute holds a typed key/value structure for data associated with an account
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
#[proto_message(type_url = "/provenance.attribute.v1.Attribute")]
pub struct Attribute {
    /// The attribute name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The attribute value.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The attribute value type.
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attribute_type: i32,
    /// The address the attribute is bound to
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
}
/// EventAttributeAdd event emitted when attribute is added
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
}
/// EventAttributeUpdate event emitted when attribute is updated
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
/// EventAttributeDelete event emitted when attribute is deleted
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
#[proto_message(type_url = "/provenance.attribute.v1.EventAttributeDelete")]
pub struct EventAttributeDelete {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventAttributeDistinctDelete event emitted when attribute is deleted with matching value
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
/// AttributeType defines the type of the data stored in the attribute value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttributeType {
    /// ATTRIBUTE_TYPE_UNSPECIFIED defines an unknown/invalid type
    Unspecified = 0,
    /// ATTRIBUTE_TYPE_UUID defines an attribute value that contains a string value representation of a V4 uuid
    Uuid = 1,
    /// ATTRIBUTE_TYPE_JSON defines an attribute value that contains a byte string containing json data
    Json = 2,
    /// ATTRIBUTE_TYPE_STRING defines an attribute value that contains a generic string value
    String = 3,
    /// ATTRIBUTE_TYPE_URI defines an attribute value that contains a URI
    Uri = 4,
    /// ATTRIBUTE_TYPE_INT defines an attribute value that contains an integer (cast as int64)
    Int = 5,
    /// ATTRIBUTE_TYPE_FLOAT defines an attribute value that contains a float
    Float = 6,
    /// ATTRIBUTE_TYPE_PROTO defines an attribute value that contains a serialized proto value in bytes
    Proto = 7,
    /// ATTRIBUTE_TYPE_BYTES defines an attribute value that contains an untyped array of bytes
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
/// GenesisState defines the attribute module's genesis state.
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
#[proto_message(type_url = "/provenance.attribute.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// deposits defines all the deposits present at genesis.
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAttributeRequest is the request type for the Query/Attribute method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Attribute",
    response_type = QueryAttributeResponse
)]
pub struct QueryAttributeRequest {
    /// account defines the address to query for.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// name is the attribute name to query for
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAttributeResponse is the response type for the Query/Attribute method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributeResponse")]
pub struct QueryAttributeResponse {
    /// a string containing the address of the account the attributes are assigned to.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// a list of attribute values
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAttributesRequest is the request type for the Query/Attributes method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Attributes",
    response_type = QueryAttributesResponse
)]
pub struct QueryAttributesRequest {
    /// account defines the address to query for.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAttributesResponse is the response type for the Query/Attribute method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryAttributesResponse")]
pub struct QueryAttributesResponse {
    /// a string containing the address of the account the attributes are assigned to=
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// a list of attribute values
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryScanRequest is the request type for the Query/Scan account attributes method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanRequest")]
#[proto_query(
    path = "/provenance.attribute.v1.Query/Scan",
    response_type = QueryScanResponse
)]
pub struct QueryScanRequest {
    /// account defines the address to query for.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// name defines the partial attribute name to search for base on names being in RDNS format.
    #[prost(string, tag = "2")]
    pub suffix: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryScanResponse is the response type for the Query/Attribute method.
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
#[proto_message(type_url = "/provenance.attribute.v1.QueryScanResponse")]
pub struct QueryScanResponse {
    /// a string containing the address of the account the attributes are assigned to=
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// a list of attribute values
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgAddAttributeRequest defines an sdk.Msg type that is used to add a new attribute to an account
/// Attributes may only be set in an account by the account that the attribute name resolves to.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeRequest")]
pub struct MsgAddAttributeRequest {
    /// The attribute name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The attribute value.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The attribute value type.
    #[prost(enumeration = "AttributeType", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub attribute_type: i32,
    /// The account to add the attribute to.
    #[prost(string, tag = "4")]
    pub account: ::prost::alloc::string::String,
    /// The address that the name must resolve to.
    #[prost(string, tag = "5")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgAddAttributeResponse defines the Msg/Vote response type.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgAddAttributeResponse")]
pub struct MsgAddAttributeResponse {}
/// MsgUpdateAttributeRequest defines an sdk.Msg type that is used to update an existing attribute to an account
/// Attributes may only be set in an account by the account that the attribute name resolves to.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeRequest")]
pub struct MsgUpdateAttributeRequest {
    /// The attribute name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The original attribute value.
    #[prost(bytes = "vec", tag = "2")]
    pub original_value: ::prost::alloc::vec::Vec<u8>,
    /// The update attribute value.
    #[prost(bytes = "vec", tag = "3")]
    pub update_value: ::prost::alloc::vec::Vec<u8>,
    /// The original attribute value type.
    #[prost(enumeration = "AttributeType", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub original_attribute_type: i32,
    /// The update attribute value type.
    #[prost(enumeration = "AttributeType", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_attribute_type: i32,
    /// The account to add the attribute to.
    #[prost(string, tag = "6")]
    pub account: ::prost::alloc::string::String,
    /// The address that the name must resolve to.
    #[prost(string, tag = "7")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgUpdateAttributeResponse defines the Msg/Vote response type.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgUpdateAttributeResponse")]
pub struct MsgUpdateAttributeResponse {}
/// MsgDeleteAttributeRequest defines a message to delete an attribute from an account
/// Attributes may only be remove from an account by the account that the attribute name resolves to.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeRequest")]
pub struct MsgDeleteAttributeRequest {
    /// The attribute name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The account to add the attribute to.
    #[prost(string, tag = "2")]
    pub account: ::prost::alloc::string::String,
    /// The address that the name must resolve to.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgDeleteAttributeResponse defines the Msg/Vote response type.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteAttributeResponse")]
pub struct MsgDeleteAttributeResponse {}
/// MsgDeleteDistinctAttributeRequest defines a message to delete an attribute with matching name, value, and type from
/// an account Attributes may only be remove from an account by the account that the attribute name resolves to.
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
#[proto_message(type_url = "/provenance.attribute.v1.MsgDeleteDistinctAttributeRequest")]
pub struct MsgDeleteDistinctAttributeRequest {
    /// The attribute name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The attribute value.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The account to add the attribute to.
    #[prost(string, tag = "3")]
    pub account: ::prost::alloc::string::String,
    /// The address that the name must resolve to.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
/// MsgDeleteDistinctAttributeResponse defines the Msg/Vote response type.
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
