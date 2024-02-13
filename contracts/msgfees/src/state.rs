use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<State> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub(crate) fee_amount: Option<Coin>,
    pub(crate) fee_recipient: Option<Addr>,
}
