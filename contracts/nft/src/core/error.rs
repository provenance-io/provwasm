use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    // General Errors
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Token already exists with id: {id}")]
    TokenExists { id: String },

    #[error("Funds present")]
    FundsPresent {},

    #[error("Funds missing")]
    FundsMissing {},

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("Unauthorized")]
    Unauthorized {},
}
