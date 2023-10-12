use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::util::parse_uuid;
use crate::{core::msg::QueryMsg, util::validate::Validate};

impl Validate for QueryMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        match self {
            QueryMsg::AllNftInfo { token_id, .. }
            | QueryMsg::Approvals { token_id, .. }
            | QueryMsg::NftInfo { token_id }
            | QueryMsg::OwnerOf { token_id, .. } => {
                parse_uuid(token_id)?;
            }
            QueryMsg::AllOperators { .. } => Ok(()),
            QueryMsg::AllTokens { .. } => Ok(()),
            QueryMsg::Approval { .. } => Ok(()),
            QueryMsg::ContractInfo {} => Ok(()),
            QueryMsg::ContractVersion {} => Ok(()),
            QueryMsg::Minter { .. } => Ok(()),
            QueryMsg::NumTokens {} => Ok(()),
            QueryMsg::Operator { .. } => Ok(()),
            QueryMsg::Ownership { .. } => Ok(()),
            QueryMsg::Tokens { .. } => Ok(()),
        }

        Ok(())
    }
    fn validate_funds(&self, _funds: &[Coin]) -> Result<(), ContractError> {
        Ok(())
    }
}
