use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::{core::msg::QueryMsg, util::validate::Validate};

impl Validate for QueryMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        match self {
            QueryMsg::AllNftInfo { .. } => Ok(()),
            QueryMsg::AllOperators { .. } => Ok(()),
            QueryMsg::AllTokens { .. } => Ok(()),
            QueryMsg::Approval { .. } => Ok(()),
            QueryMsg::Approvals { .. } => Ok(()),
            QueryMsg::ContractInfo { .. } => Ok(()),
            QueryMsg::ContractVersion { .. } => Ok(()),
            QueryMsg::Minter { .. } => Ok(()),
            QueryMsg::NftInfo { .. } => Ok(()),
            QueryMsg::NumTokens { .. } => Ok(()),
            QueryMsg::Operator { .. } => Ok(()),
            QueryMsg::OwnerOf { .. } => Ok(()),
            QueryMsg::Ownership { .. } => Ok(()),
            QueryMsg::Tokens { .. } => Ok(()),
        }
    }
    fn validate_funds(&self, _funds: &[Coin]) -> Result<(), ContractError> {
        Ok(())
    }
}
