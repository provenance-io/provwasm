use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;
use crate::{core::msg::ExecuteMsg, util::validate::Validate};

impl Validate for ExecuteMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        Ok(())
    }

    fn validate_funds(&self, _funds: &[Coin]) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::Mint { .. } => Ok(()),
            ExecuteMsg::Burn { .. } => Ok(()),
            ExecuteMsg::TransferNft { .. } => Ok(()),
            ExecuteMsg::Approve { .. } => Ok(()),
            ExecuteMsg::Revoke { .. } => Ok(()),
            ExecuteMsg::ApproveAll { .. } => Ok(()),
            ExecuteMsg::RevokeAll { .. } => Ok(()),
            ExecuteMsg::SendNft { .. } => Ok(()),
            ExecuteMsg::UpdateOwnership(_) => Ok(()),
        }
    }
}
