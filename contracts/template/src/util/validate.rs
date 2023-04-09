use cosmwasm_std::Coin;

use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;

pub trait Validate {
        fn validate(&self) -> ValidateResult;
        fn validate_funds(&self, funds: &[Coin]) -> ValidateResult;
}