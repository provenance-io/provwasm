use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::types;

#[cw_serde]
pub struct InitMsg {
    pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        supply: Uint128,
        denom: String,
        allow_forced_transfer: bool,
    },
    GrantAccess {
        denom: String,
    },
    Finalize {
        denom: String,
    },
    Activate {
        denom: String,
    },
    Mint {
        amount: Uint128,
        denom: String,
    },
    Burn {
        amount: Uint128,
        denom: String,
    },
    Cancel {
        denom: String,
    },
    Destroy {
        denom: String,
    },
    Transfer {
        amount: Uint128,
        denom: String,
        to: String,
    },
    Withdraw {
        amount: Uint128,
        denom: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(types::Marker)]
    GetByAddress { address: String },
    #[returns(types::Marker)]
    GetByDenom { denom: String },
}
