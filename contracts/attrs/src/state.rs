use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<State> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub contract_owner: Addr, // Ensures only sender from contract init can call handle.
    pub contract_name: String, // The root name of the contract.
}
