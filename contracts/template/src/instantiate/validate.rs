use cosmwasm_std::Coin;

use crate::{
    core::msg::InstantiateMsg,
    types::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            _ => Ok(()),
        }
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
