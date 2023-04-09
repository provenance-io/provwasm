use cosmwasm_std::{Coin, Storage};
use semver::Version;

use crate::{
    core::{
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        msg::MigrateMsg,
    },
    util::validate::{Validate, ValidateResult},
};

impl Validate for MigrateMsg {
    fn validate(&self) -> ValidateResult {
        Ok(())
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}

pub fn validate_migration(storage: &dyn Storage) -> ValidateResult {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = cw2::get_contract_version(storage)?.version.parse().unwrap();
    let ver = cw2::get_contract_version(storage)?;

    if ver.contract != CONTRACT_NAME {
        return Err(crate::core::error::ContractError::ContractNameMismatch(
            ver.contract,
            CONTRACT_NAME.to_string(),
        ));
    }

    if storage_version >= version {
        return Err(crate::core::error::ContractError::InvalidVersion(
            version.to_string(),
            storage_version.to_string(),
        ));
    }

    Ok(())
}
