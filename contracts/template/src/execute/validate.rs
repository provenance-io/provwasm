use cosmwasm_std::Coin;

use crate::{
    core::{aliases::ProvDeps, error::ContractError, msg::ExecuteMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    fn validate(&self, _deps: ProvDeps) -> ValidateResult {
        Ok(())
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
