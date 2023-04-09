use cosmwasm_std::Coin;

use crate::{
    core::msg::InstantiateMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
