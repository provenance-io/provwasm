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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};

    use crate::{
        testing::constants::{OWNER, TEST_DENOM},
        util::fee::Fee,
    };

    use super::State;

    #[test]
    fn test_new() {
        let owner = Addr::unchecked(OWNER);
        let fee = Fee {
            recipient: None,
            amount: Coin::new(100, TEST_DENOM),
        };
        let state = State::new(Addr::unchecked(OWNER), fee.clone());
        assert_eq!(owner, state.owner);
        assert_eq!(fee, state.fee);
    }
}
