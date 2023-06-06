use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;

pub trait Validate {
    fn validate(&self, deps: Deps) -> ValidateResult;
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult;
}
