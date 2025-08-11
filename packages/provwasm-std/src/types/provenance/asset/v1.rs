use provwasm_proc_macro::CosmwasmExt;
/// AssetClass defines the class type data for an Asset. Similar to NftClass.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.AssetClass")]
pub struct AssetClass {
    /// id defines the unique identifier of the asset classification, similar to the contract address of ERC721.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the asset classification.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for asset classification.
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    /// description is a brief description of asset classification.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// uri for the class metadata stored off chain. It can define schema for Class and asset `Data` attributes.
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri.
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is the app specific json schema of the asset class.
    #[prost(string, tag = "7")]
    pub data: ::prost::alloc::string::String,
}
/// Asset defines the asset. Similar to Nft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.Asset")]
pub struct Asset {
    /// class_id associated with the asset, similar to the contract address of ERC721.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the asset.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// uri for the asset metadata stored off chain.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri.
    #[prost(string, tag = "4")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is an app specific json data of the asset.
    #[prost(string, tag = "5")]
    pub data: ::prost::alloc::string::String,
}
/// AssetKey defines the lookup key for an Asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.AssetKey")]
pub struct AssetKey {
    /// class_id associated with the asset, similar to the contract address of ERC721.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the asset.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// GenesisState defines the asset module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.GenesisState")]
pub struct GenesisState {
    /// asset defines the list of assets in the genesis state.
    #[prost(message, repeated, tag = "1")]
    pub asset: ::prost::alloc::vec::Vec<Asset>,
    /// asset_classes defines the list of asset classes in the genesis state.
    #[prost(message, repeated, tag = "2")]
    pub asset_classes: ::prost::alloc::vec::Vec<AssetClass>,
}
/// QueryListAssets is the request type for the ListAssets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryListAssets")]
#[proto_query(
    path = "/provenance.asset.v1.Query/ListAssets",
    response_type = QueryListAssetsResponse
)]
pub struct QueryListAssets {
    /// address defines the address to query assets for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryListAssetsResponse is the response type for the ListAssets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryListAssetsResponse")]
pub struct QueryListAssetsResponse {
    /// assets defines the list of assets.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryListAssetClasses is the request type for the ListAssetClasses RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryListAssetClasses")]
#[proto_query(
    path = "/provenance.asset.v1.Query/ListAssetClasses",
    response_type = QueryListAssetClassesResponse
)]
pub struct QueryListAssetClasses {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryListAssetClassesResponse is the response type for the ListAssetClasses RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryListAssetClassesResponse")]
pub struct QueryListAssetClassesResponse {
    /// asset_classes defines the list of asset classes.
    #[prost(message, repeated, tag = "1")]
    pub asset_classes: ::prost::alloc::vec::Vec<AssetClass>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetClass is the request type for the GetClass RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryGetClass")]
#[proto_query(
    path = "/provenance.asset.v1.Query/GetClass",
    response_type = QueryGetClassResponse
)]
pub struct QueryGetClass {
    /// id defines the unique identifier of the asset class to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryGetClassResponse is the response type for the GetClass RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryGetClassResponse")]
pub struct QueryGetClassResponse {
    /// asset_class defines the requested asset class.
    #[prost(message, optional, tag = "1")]
    pub asset_class: ::core::option::Option<AssetClass>,
}
/// MsgCreateAsset is the message type for creating a new asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateAsset")]
pub struct MsgCreateAsset {
    /// asset defines the asset to be created.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<Asset>,
    /// from_address defines the address of the message sender.
    #[prost(string, tag = "2")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgCreateAssetResponse is the response type for the CreateAsset RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateAssetResponse")]
pub struct MsgCreateAssetResponse {}
/// MsgCreateAssetClass is the message type for creating a new asset class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateAssetClass")]
pub struct MsgCreateAssetClass {
    /// asset_class defines the asset class to be created.
    #[prost(message, optional, tag = "1")]
    pub asset_class: ::core::option::Option<AssetClass>,
    /// from_address defines the address of the message sender.
    #[prost(string, tag = "2")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgCreateAssetClassResponse is the response type for the CreateAssetClass RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateAssetClassResponse")]
pub struct MsgCreateAssetClassResponse {}
/// MsgCreatePool is the message type for creating a new pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreatePool")]
pub struct MsgCreatePool {
    /// pool defines the pool to be created.
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// assets defines the list of assets by keys to be included in the pool.
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<AssetKey>,
    /// from_address defines the address of the message sender.
    #[prost(string, tag = "3")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgCreatePoolResponse is the response type for the CreatePool RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreatePoolResponse")]
pub struct MsgCreatePoolResponse {}
/// MsgCreateTokenization is the message type for creating a new tokenization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateTokenization")]
pub struct MsgCreateTokenization {
    /// denom defines the denomination for the tokenization.
    #[prost(message, optional, tag = "1")]
    pub denom: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// asset defines the asset by key to be tokenized.
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<AssetKey>,
    /// from_address defines the address of the message sender.
    #[prost(string, tag = "3")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgCreateTokenizationResponse is the response type for the CreateTokenization RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateTokenizationResponse")]
pub struct MsgCreateTokenizationResponse {}
/// MsgCreateSecuritization is the message type for creating a new securitization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateSecuritization")]
pub struct MsgCreateSecuritization {
    /// id defines the unique identifier for the securitization.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pools defines the list of pool identifiers to be included in the securitization.
    #[prost(string, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// tranches defines the list of tranches for the securitization.
    #[prost(message, repeated, tag = "3")]
    pub tranches: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// from_address defines the address of the message sender.
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgCreateSecuritizationResponse is the response type for the CreateSecuritization RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateSecuritizationResponse")]
pub struct MsgCreateSecuritizationResponse {}
pub struct AssetQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AssetQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn list_assets(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryListAssetsResponse, cosmwasm_std::StdError> {
        QueryListAssets {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn list_asset_classes(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryListAssetClassesResponse, cosmwasm_std::StdError> {
        QueryListAssetClasses { pagination }.query(self.querier)
    }
    pub fn get_class(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryGetClassResponse, cosmwasm_std::StdError> {
        QueryGetClass { id }.query(self.querier)
    }
}
