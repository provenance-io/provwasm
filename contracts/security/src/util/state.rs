use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

/// A general purpose object that the contract uses to track store its state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub owner: Addr,
}

impl State {
    /// Creates a new instance of State
    ///
    /// # Arguments
    ///
    /// * `owner` - The address of the contract's owner.
    ///
    /// # Examples
    /// ```
    /// let owner = Addr::unchecked(OWNER);
    /// let state = State::new(owner, fee);
    /// ```
    pub fn new(owner: Addr) -> Self {
        State { owner }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;

    use crate::testing::constants::OWNER;

    use super::State;

    #[test]
    fn test_new() {
        let owner = Addr::unchecked(OWNER);
        let state = State::new(owner.clone());
        assert_eq!(owner, state.owner);
    }
}
