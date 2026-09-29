use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(StdError),
    #[error("Unauthorized")]
    Unauthorized {},
}

/// `StdError` no longer implements `std::error::Error`, so this conversion is manual.
impl From<StdError> for ContractError {
    fn from(err: StdError) -> Self {
        ContractError::Std(err)
    }
}
