use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::{core::msg::QueryMsg, util::validate::Validate};

impl Validate for QueryMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        match self {
            // QueryMsg::Approval { .. } => Ok(()),
            // QueryMsg::Approvals { .. } => Ok(()),
            // QueryMsg::Operator { .. } => Ok(()),
            // QueryMsg::AllOperators { .. } => Ok(()),
            // QueryMsg::NumTokens { .. } => Ok(()),
            // QueryMsg::ContractInfo { .. } => Ok(()),
            QueryMsg::NftInfo { .. } => Ok(()),
            // QueryMsg::AllNftInfo { .. } => Ok(()),
            // QueryMsg::Tokens { .. } => Ok(()),
            // QueryMsg::AllTokens { .. } => Ok(()),
            // QueryMsg::Minter { .. } => Ok(()),
        }
    }
    fn validate_funds(&self, _funds: &[Coin]) -> Result<(), ContractError> {
        Ok(())
    }
}
