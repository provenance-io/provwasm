use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;
use uuid::Error;

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

    #[error("Limit exceeds maximum of: {max_limit}")]
    MaxLimitExceeded { max_limit: u32 },

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Could not parse to UUID: {error}")]
    Uuid { error: Error },
}
