use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;

pub trait Validate {
    fn validate(&self, deps: Deps) -> Result<(), ContractError>;
    fn validate_funds(&self, funds: &[Coin]) -> Result<(), ContractError>;
}
