use cosmwasm_std::Coin;

use crate::{
    core::{error::ContractError, msg::ExecuteMsg},
    types::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            _ => Ok(()),
        }
    }

    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult {
        match self {
            ExecuteMsg::ChangeOwner { new_owner: _ } => {
                if !funds.is_empty() {
                    return Err(ContractError::UnexpectedFunds {});
                }
                Ok(())
            }
        }
    }
}
