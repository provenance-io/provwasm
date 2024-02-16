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

impl From<String> for Security {
    fn from(security: String) -> Self {
        let parts: Vec<&str> = security.split(".").collect();
        let category = parts[0].into();
        let mut name: Option<String> = None;
        if parts.len() > 1 {
            name = Some(parts[1].into());
        }
        Security { category, name }
    }
}

impl From<&Security> for String {
    fn from(security: &Security) -> Self {
        if security.name.is_none() {
            return format!("{}", security.category);
        }
        return format!("{}.{}", security.category, security.name.as_ref().unwrap());
    }
}

impl From<(String, String)> for Security {
    fn from(security: (String, String)) -> Self {
        let category = security.0;
        let mut name: Option<String> = None;
        if !security.1.is_empty() {
            name = Some(security.1);
        }
        Security { category, name }
    }
}

impl Security {
    pub fn new(category: &str) -> Self {
        Self {
            category: category.to_string(),
            name: None,
        }
    }

    pub fn new_with_name(category: &str, name: &str) -> Self {
        Self {
            category: category.to_string(),
            name: Some(name.to_string()),
        }
    }
}

impl ToString for Security {
    fn to_string(&self) -> String {
        if self.name.is_some() {
            return format!("{}.{}", self.category, self.name.as_ref().unwrap());
        }
        format!("{}", self.category)
    }
}
