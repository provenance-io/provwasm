use cosmwasm_std::Coin;

use crate::{
    core::msg::QueryMsg,
    types::validate::{Validate, ValidateResult},
};

impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            QueryMsg::QueryOwner {} => Ok(()),
            QueryMsg::QueryVersion {} => Ok(()),
        }
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}
