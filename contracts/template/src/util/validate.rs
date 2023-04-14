use cosmwasm_std::Coin;

use crate::core::{aliases::ProvDeps, error::ContractError};

pub type ValidateResult = Result<(), ContractError>;

pub trait Validate {
    fn validate(&self, deps: ProvDeps) -> ValidateResult;
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult;
}
