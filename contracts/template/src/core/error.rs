use cosmwasm_std::{Coin, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    // General Errors
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // Funds
    #[error("Missing required funds")]
    MissingFunds {},

    #[error("Unexpected funds included in transaction")]
    UnexpectedFunds {},

    #[error("Mismatch in the expected funds and the actual funds. Expected {0}, but received {1}")]
    MismatchFunds(Coin, Coin),

    // Migration Errors
    #[error("Mismatch in the migration contract name. Expected contract with name {0}, but received {1}")]
    ContractNameMismatch(String, String),

    #[error("Invalid migration version. Version {0} is not greater than {1}")]
    InvalidVersion(String, String),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    // Reply Errors
    #[error("Unexpected reply id: {0}")]
    UnexpectedReplyId(u64),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
