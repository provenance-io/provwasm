use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};
use cw2::ContractVersion;

use super::aliases::AssetTag;

#[cw_serde]
pub enum InstantiateMsg {
    Default {
        owner: Addr,
        tag_types: Vec<AssetTag>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner { new_owner: Addr },
    SetTag { asset_addr: Addr, tag: AssetTag },
    AddTagTypes { tag_types: Vec<AssetTag> },
    RemoveTagTypes { tag_types: Vec<AssetTag> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryVersionResponse)]
    QueryVersion {},

    #[returns(QueryOwnerResponse)]
    QueryOwner {},

    #[returns(QueryAddressResponse)]
    QueryAddress { asset_addr: Addr },

    #[returns(QueryTagResponse)]
    QueryTag {
        tag: AssetTag,
        paginate: Paginate<Addr>,
    },

    #[returns(QueryTagTypesResponse)]
    QueryTagTypes { paginate: Paginate<AssetTag> },
}

#[cw_serde]
pub struct QueryVersionResponse {
    pub contract_version: ContractVersion,
}

#[cw_serde]
pub struct QueryOwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct QueryAddressResponse {
    pub tag: AssetTag,
}

#[cw_serde]
pub struct QueryTagResponse {
    pub assets: Vec<Addr>,
}

#[cw_serde]
pub struct QueryTagTypesResponse {
    pub tags: Vec<AssetTag>,
}

#[cw_serde]
pub enum MigrateMsg {
    Default {},
}

#[cw_serde]
pub struct Paginate<T> {
    pub limit: Option<Uint64>,
    pub start_after: Option<T>,
}
