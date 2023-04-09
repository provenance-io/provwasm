use cosmwasm_std::StdError;
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

    #[error("Mismatch in the expected funds and the actual funds")]
    MismatchFunds {},

    // Migration Errors
    #[error("Mismatch in the migration contract name")]
    ContractNameMismatch {},

    #[error("Invalid migration version")]
    InvalidVersion {},

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
