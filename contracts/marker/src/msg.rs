use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InitMsg {
    pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Create {
        supply: Uint128,
        denom: String,
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
    #[returns(provwasm_std::Marker)]
    GetByAddress { address: String },
    #[returns(provwasm_std::Marker)]
    GetByDenom { denom: String },
}
