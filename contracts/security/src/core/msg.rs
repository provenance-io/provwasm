use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};
use cw2::ContractVersion;

#[cw_serde]
pub enum InstantiateMsg {
    Default {
        owner: Addr,
        security_types: Vec<Security>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner {
        new_owner: Addr,
    },
    SetSecurity {
        asset_addr: Addr,
        security: Security,
    },
    SetSecurityMultiple {
        assets: Vec<Addr>,
        security: Security,
    },
    RemoveSecurity {
        asset_addr: Addr,
    },
    AddSecurityTypes {
        security_types: Vec<Security>,
    },
    RemoveSecurityTypes {
        security_types: Vec<Security>,
    },
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

    #[returns(QuerySecurityResponse)]
    QuerySecurity {
        security: Security,
        paginate: Paginate<Addr>,
    },

    #[returns(QuerySecurityCategoryResponse)]
    QuerySecurityCategory {
        category: String,
        paginate: Paginate<CategorizedSecurity>,
    },

    #[returns(QuerySecurityTypesResponse)]
    QuerySecurityTypes { paginate: Paginate<Security> },
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
    pub security: Security,
}

#[cw_serde]
pub struct QuerySecurityResponse {
    pub assets: Vec<Addr>,
}

#[cw_serde]
pub struct QuerySecurityCategoryResponse {
    pub assets: Vec<CategorizedSecurity>,
}

#[cw_serde]
pub struct QuerySecurityTypesResponse {
    pub securities: Vec<Security>,
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

#[cw_serde]
pub struct Security {
    pub category: String,
    pub name: Option<String>,
}

#[cw_serde]
pub struct CategorizedSecurity {
    pub name: String,
    pub asset: Addr,
}
