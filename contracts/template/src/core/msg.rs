use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw2::ContractVersion;

#[cw_serde]
pub enum InstantiateMsg {
    Instantiate { owner: Addr },
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner { new_owner: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(QueryVersionResponse)]
    QueryVersion {},

    #[returns(QueryOwnerResponse)]
    QueryOwner {},
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
pub enum MigrateMsg {
    MigrateV1ToV2 {},
}
