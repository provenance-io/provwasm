use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    // General Errors
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Asset does not exist for address {0}")]
    AssetDoesNotExist(String),

    #[error("Security type {0} is still in use and cannot be removed")]
    SecurityInUse(String),

    #[error("Security type {0} is not a valid security type")]
    InvalidSecurityType(String),

    #[error("Security type {0} has an empty string for category or name")]
    InvalidSecurityTypeFormat(String),

    #[error("No assets were included in the message")]
    NoAssetsSupplied {},

    // Funds
    #[error("Unexpected funds included in transaction")]
    UnexpectedFunds {},

    // Migration Errors
    #[error("Mismatch in the migration contract name. Expected contract with name {0}, but received {1}")]
    ContractNameMismatch(String, String),

    #[error("Invalid migration version. Version {0} is not greater than {1}")]
    InvalidVersion(String, String),

    #[error("Semver parsing error: {0}")]
    SemVer(String),
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
