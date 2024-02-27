use cosmwasm_std::{Coin, Deps};
use semver::Version;

use crate::{
    core::{
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
        error::ContractError,
        msg::MigrateMsg,
    },
    util::validate::{Validate, ValidateResult},
};

impl Validate for MigrateMsg {
    /// Performs basic error checking on the MigrateMsg
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// let msg = MigrateMsg::Default {};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, deps: Deps) -> ValidateResult {
        let storage = deps.storage;
        let version: Version = CONTRACT_VERSION.parse()?;
        let storage_version: Version = cw2::get_contract_version(storage)?.version.parse().unwrap();
        let ver = cw2::get_contract_version(storage)?;

        if ver.contract != CONTRACT_NAME {
            return Err(ContractError::ContractNameMismatch(
                ver.contract,
                CONTRACT_NAME.to_string(),
            ));
        }

        if storage_version >= version {
            return Err(ContractError::InvalidVersion(
                version.to_string(),
                storage_version.to_string(),
            ));
        }

        Ok(())
    }

    /// Performs basic error checking on MigrateMsg.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// let msg = MigrateMsg::Default {};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cw2::set_contract_version;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            constants::{CONTRACT_NAME, CONTRACT_VERSION},
            error::ContractError,
            msg::MigrateMsg,
        },
        util::validate::Validate,
    };

    #[test]
    fn test_validate_funds() {
        let msg = MigrateMsg::Default {};
        assert_eq!((), msg.validate_funds(&[]).unwrap());
    }

    #[test]
    fn test_validate_names_must_match() {
        let mut deps = mock_provenance_dependencies();
        let msg = MigrateMsg::Default {};
        let name = "TEST_NAME";
        let version = CONTRACT_VERSION;
        set_contract_version(deps.as_mut().storage, name, version).unwrap();

        let err = msg.validate(deps.as_ref()).unwrap_err();
        assert_eq!(
            ContractError::ContractNameMismatch(name.to_string(), CONTRACT_NAME.to_string())
                .to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_validate_version_must_be_greater() {
        let mut deps = mock_provenance_dependencies();
        let msg = MigrateMsg::Default {};
        let name = CONTRACT_NAME;
        let version = CONTRACT_VERSION;
        set_contract_version(deps.as_mut().storage, name, version).unwrap();

        let err = msg.validate(deps.as_ref()).unwrap_err();
        assert_eq!(
            ContractError::InvalidVersion(version.to_string(), CONTRACT_VERSION.to_string())
                .to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_validate_success() {
        let mut deps = mock_provenance_dependencies();
        let msg = MigrateMsg::Default {};
        let name = CONTRACT_NAME;
        let version = "0.0.1";
        set_contract_version(deps.as_mut().storage, name, version).unwrap();
        msg.validate(deps.as_ref()).unwrap();
    }
}
