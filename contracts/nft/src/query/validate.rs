use cosmwasm_std::{Coin, Deps};

use crate::core::constants::MAX_LIMIT;
use crate::core::error::ContractError;
use crate::util::parse_uuid;
use crate::{core::msg::QueryMsg, util::validate::Validate};

impl Validate for QueryMsg {
    fn validate(&self, deps: Deps) -> Result<(), ContractError> {
        match self {
            QueryMsg::AllNftInfo { token_id, .. }
            | QueryMsg::Approvals { token_id, .. }
            | QueryMsg::NftInfo { token_id }
            | QueryMsg::OwnerOf { token_id, .. } => {
                parse_uuid(token_id)?;
            }
            QueryMsg::Approval {
                token_id, spender, ..
            } => {
                parse_uuid(token_id)?;
                deps.api.addr_validate(spender)?;
            }
            QueryMsg::AllOperators { owner, limit, .. } | QueryMsg::Tokens { owner, limit, .. } => {
                deps.api.addr_validate(owner)?;
                if limit.is_some_and(|limit| limit > MAX_LIMIT) {
                    return Err(ContractError::MaxLimitExceeded {
                        max_limit: MAX_LIMIT,
                    });
                }
            }
            QueryMsg::AllTokens { limit, .. } => {
                if limit.is_some_and(|limit| limit > MAX_LIMIT) {
                    return Err(ContractError::MaxLimitExceeded {
                        max_limit: MAX_LIMIT,
                    });
                }
            }
            QueryMsg::Operator {
                owner, operator, ..
            } => {
                deps.api.addr_validate(owner)?;
                deps.api.addr_validate(operator)?;
            }
            _ => {}
        }

        Ok(())
    }
    fn validate_funds(&self, _funds: &[Coin]) -> Result<(), ContractError> {
        Ok(())
    }
}
