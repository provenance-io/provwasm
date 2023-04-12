use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

use super::fee::Fee;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub fee: Fee,
}

impl State {
    pub fn new(owner: Addr, fee: Fee) -> Self {
        State { owner, fee }
    }
}
