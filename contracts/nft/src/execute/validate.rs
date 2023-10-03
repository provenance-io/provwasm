use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::util::parse_uuid;
use crate::{core::msg::ExecuteMsg, util::validate::Validate};

impl Validate for ExecuteMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::Approve { token_id, .. }
            | ExecuteMsg::Burn { token_id }
            | ExecuteMsg::Revoke { token_id, .. } => {
                parse_uuid(token_id)?;
            }
            ExecuteMsg::Mint {
                session_uuid,
                token_id,
                ..
            }
            | ExecuteMsg::SendNft {
                session_uuid,
                token_id,
                ..
            }
            | ExecuteMsg::TransferNft {
                session_uuid,
                token_id,
                ..
            } => {
                parse_uuid(session_uuid)?;
                parse_uuid(token_id)?;
            }
            _ => {}
        }

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
