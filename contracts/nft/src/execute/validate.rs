use cosmwasm_std::{Coin, Deps};

use crate::{
    core::msg::ExecuteMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    fn validate(&self, _deps: Deps) -> ValidateResult {
        Ok(())
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        match self {
            ExecuteMsg::Mint { .. } => Ok(()),
            ExecuteMsg::Burn { .. } => Ok(()),
        }
    }
}
