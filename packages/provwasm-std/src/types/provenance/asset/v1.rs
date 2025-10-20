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
/// AssetData defines the data type for Assets and Asset Classes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.AssetData")]
pub struct AssetData {
    /// value is the string stored in the data field of an asset or asset class.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// EventAssetBurned is emitted when an asset is burned.
/// This event is triggered by the MsgBurnAsset message handler when an
/// asset is successfully burned and removed from circulation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventAssetBurned")]
pub struct EventAssetBurned {
    /// class_id is the asset class identifier of the burned asset.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is the identifier of the burned asset.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// owner is the address of the account that owned the asset before it was burned.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventAssetClassCreated is emitted when a new asset class is created.
/// This event is triggered by the MsgCreateAssetClass message handler when
/// an asset class is successfully created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventAssetClassCreated")]
pub struct EventAssetClassCreated {
    /// class_id is the unique identifier of the created asset class.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// class_name is the human-readable name of the asset class.
    #[prost(string, tag = "2")]
    pub class_name: ::prost::alloc::string::String,
    /// class_symbol is the symbol or ticker for the asset class.
    #[prost(string, tag = "3")]
    pub class_symbol: ::prost::alloc::string::String,
}
/// EventAssetCreated is emitted when a new asset is created.
/// This event is triggered by the MsgCreateAsset message handler when
/// an asset is successfully created and minted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventAssetCreated")]
pub struct EventAssetCreated {
    /// class_id is the asset class identifier of the created asset.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is the identifier of the created asset.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// owner is the address of the account that owns the newly created asset.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventPoolCreated is emitted when a new pool is created.
/// This event is triggered by the MsgCreatePool message handler when
/// a pool is successfully created with assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventPoolCreated")]
pub struct EventPoolCreated {
    /// pool is the coin representation of the created pool.
    #[prost(string, tag = "1")]
    pub pool: ::prost::alloc::string::String,
    /// asset_count is the number of assets added to the pool.
    #[prost(uint32, tag = "2")]
    pub asset_count: u32,
    /// owner is the address of the account that created the pool.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventTokenizationCreated is emitted when a tokenization marker is created.
/// This event is triggered by the MsgCreateTokenization message handler when
/// a tokenization is successfully created for an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventTokenizationCreated")]
pub struct EventTokenizationCreated {
    /// tokenization is the coin representation of the tokenization marker.
    #[prost(string, tag = "1")]
    pub tokenization: ::prost::alloc::string::String,
    /// class_id is the asset class identifier of the tokenized asset.
    #[prost(string, tag = "2")]
    pub class_id: ::prost::alloc::string::String,
    /// id is the identifier of the tokenized asset.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// owner is the address of the account that created the tokenization.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
/// EventSecuritizationCreated is emitted when a securitization is created.
/// This event is triggered by the MsgCreateSecuritization message handler when
/// a securitization is successfully created with tranches and pools.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.EventSecuritizationCreated")]
pub struct EventSecuritizationCreated {
    /// securitization_id is the unique identifier of the created securitization.
    #[prost(string, tag = "1")]
    pub securitization_id: ::prost::alloc::string::String,
    /// tranche_count is the number of tranches in the securitization.
    #[prost(uint32, tag = "2")]
    pub tranche_count: u32,
    /// pool_count is the number of pools in the securitization.
    #[prost(uint32, tag = "3")]
    pub pool_count: u32,
    /// owner is the address of the account that created the securitization.
    #[prost(string, tag = "4")]
    pub owner: ::prost::alloc::string::String,
}
/// QueryAssetRequest is the request type for the Query/Asset RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetRequest")]
#[proto_query(
    path = "/provenance.asset.v1.Query/Asset",
    response_type = QueryAssetResponse
)]
pub struct QueryAssetRequest {
    /// class_id associated with the asset.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the asset.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryAssetResponse is the response type for the Query/Asset RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetResponse")]
pub struct QueryAssetResponse {
    /// asset is the asset returned from the query.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<Asset>,
}
/// QueryAssetsRequest is the request type for the Query/Assets RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetsRequest")]
#[proto_query(
    path = "/provenance.asset.v1.Query/Assets",
    response_type = QueryAssetsResponse
)]
pub struct QueryAssetsRequest {
    /// class_id associated with the asset.
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the asset.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAssetsResponse is the response type for the Query/Assets RPC methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetsResponse")]
pub struct QueryAssetsResponse {
    /// assets are the assets returned from the query.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryAssetClassRequest is the request type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetClassRequest")]
#[proto_query(
    path = "/provenance.asset.v1.Query/AssetClass",
    response_type = QueryAssetClassResponse
)]
pub struct QueryAssetClassRequest {
    /// class_id associated with the asset.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryAssetClassResponse is the response type for the Query/Class RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetClassResponse")]
pub struct QueryAssetClassResponse {
    /// asset_class is the asset class returned from the query.
    #[prost(message, optional, tag = "1")]
    pub asset_class: ::core::option::Option<AssetClass>,
}
/// QueryAssetClassesRequest is the request type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetClassesRequest")]
#[proto_query(
    path = "/provenance.asset.v1.Query/AssetClasses",
    response_type = QueryAssetClassesResponse
)]
pub struct QueryAssetClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAssetClassesResponse is the response type for the Query/Classes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.QueryAssetClassesResponse")]
pub struct QueryAssetClassesResponse {
    /// asset_classes are the asset classes returned from the query.
    #[prost(message, repeated, tag = "1")]
    pub asset_classes: ::prost::alloc::vec::Vec<AssetClass>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgBurnAsset is the message type for burning an existing asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgBurnAsset")]
pub struct MsgBurnAsset {
    /// asset defines the asset by key to be burned.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<AssetKey>,
    /// signer defines the address of asset owner.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgBurnAssetResponse is the response type for the BurnAsset RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgBurnAssetResponse")]
pub struct MsgBurnAssetResponse {}
/// MsgCreateAsset is the message type for creating a new asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.asset.v1.MsgCreateAsset")]
pub struct MsgCreateAsset {
    /// asset defines the asset to be created.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<Asset>,
    /// owner defines the owner address for the created asset.
    /// if owner is not provided, signer will be used.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// signer defines the address of the message sender.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
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
    /// signer defines the address of the message sender.
    #[prost(string, tag = "2")]
    pub signer: ::prost::alloc::string::String,
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
    /// signer defines the address of the message sender.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
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
    /// token defines the new token supply and symbol.
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// asset defines the asset by key to be tokenized.
    #[prost(message, optional, tag = "2")]
    pub asset: ::core::option::Option<AssetKey>,
    /// signer defines the address of the message sender.
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
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
    /// signer defines the address of the message sender.
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
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
    pub fn asset(
        &self,
        class_id: ::prost::alloc::string::String,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryAssetResponse, cosmwasm_std::StdError> {
        QueryAssetRequest { class_id, id }.query(self.querier)
    }
    pub fn assets(
        &self,
        class_id: ::prost::alloc::string::String,
        owner: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAssetsResponse, cosmwasm_std::StdError> {
        QueryAssetsRequest {
            class_id,
            owner,
            pagination,
        }
        .query(self.querier)
    }
    pub fn asset_class(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryAssetClassResponse, cosmwasm_std::StdError> {
        QueryAssetClassRequest { id }.query(self.querier)
    }
    pub fn asset_classes(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAssetClassesResponse, cosmwasm_std::StdError> {
        QueryAssetClassesRequest { pagination }.query(self.querier)
    }
}
