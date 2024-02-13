use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<State> = Item::new("config");

/// Fields that comprise the smart contract state
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    // The required purchase denomination
    pub purchase_denom: String,
    // The merchant account
    pub merchant_address: Addr,
    // The fee collection account
    pub fee_collection_address: Addr,
    // The percentage to collect on transfers
    pub fee_percent: Decimal,
}
