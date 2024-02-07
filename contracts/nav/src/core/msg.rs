use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw2::ContractVersion;

#[cw_serde]
pub enum InstantiateMsg {
    Default { owner: Addr },
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner { new_owner: Addr },
    SetTag { asset_addr: Addr, tag: String },
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
    QueryTag { tag: String },

    #[returns(QueryTagTypesResponse)]
    QueryTagTypes {},
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
    pub tag: String,
}

#[cw_serde]
pub struct QueryTagResponse {
    pub assets: Vec<Addr>,
}

#[cw_serde]
pub struct QueryTagTypesResponse {
    pub tags: Vec<String>,
}

#[cw_serde]
pub enum MigrateMsg {
    Default {},
}
