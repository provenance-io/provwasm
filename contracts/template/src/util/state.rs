use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

use super::fee::Fee;

/// A general purpose object that the contract uses to track store its state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub fee: Fee,
}

impl State {
    /// Creates a new instance of State
    ///
    /// # Arguments
    ///
    /// * `owner` - The address of the contract's owner.
    /// * `fee` - The custom fee to apply to the contract.
    ///
    /// # Examples
    /// ```
    /// let owner = Addr::unchecked(OWNER);
    /// let fee = Fee {recipient: None, amount: Coin::new(100, "nhash")};
    /// let state = State::new(owner, fee);
    /// ```
    pub fn new(owner: Addr, fee: Fee) -> Self {
        State { owner, fee }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};

    use crate::{
        testing::constants::{OWNER, TEST_AMOUNT, TEST_DENOM},
        util::fee::Fee,
    };

    use super::State;

    #[test]
    fn test_new() {
        let owner = Addr::unchecked(OWNER);
        let fee = Fee {
            recipient: None,
            amount: Coin::new(TEST_AMOUNT, TEST_DENOM),
        };
        let state = State::new(owner.clone(), fee.clone());
        assert_eq!(owner, state.owner);
        assert_eq!(fee, state.fee);
    }
}
