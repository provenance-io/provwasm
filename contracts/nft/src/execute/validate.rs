use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::{core::msg::ExecuteMsg, util::validate::Validate};

impl Validate for ExecuteMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        Ok(())
    }

    // in this nft example, the minter has already received funds OOB since they are the only one authorized to call the mint function
    fn validate_funds(&self, funds: &[Coin]) -> Result<(), ContractError> {
        if !funds.is_empty() {
            return Err(ContractError::FundsPresent {});
        }
        Ok(())
    }
}
