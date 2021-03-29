use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
        to: HumanAddr,
    },
    Withdraw {
        amount: Uint128,
        denom: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetByAddress { address: HumanAddr },
    GetByDenom { denom: String },
}
