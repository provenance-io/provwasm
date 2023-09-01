use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    // General Errors
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Token already exists with id: {id}")]
    TokenExists { id: String },

    #[error("Funds present")]
    FundsPresent {},

    #[error("Funds missing")]
    FundsMissing {},

    #[error("Unauthorized")]
    Unauthorized {},
}
