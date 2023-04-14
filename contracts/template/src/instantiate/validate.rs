use cosmwasm_std::Coin;

use crate::{
    core::{aliases::ProvDeps, error::ContractError, msg::InstantiateMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    fn validate(&self, _deps: ProvDeps) -> ValidateResult {
        Ok(())
    }

    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult {
        if funds.is_empty() {
            return Err(ContractError::MissingFunds {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::error::ContractError,
        testing::{
            constants::{TEST_AMOUNT, TEST_DENOM},
            msg::mock_instantiate_msg,
        },
        util::validate::Validate,
    };

    #[test]
    fn test_validate_funds_has_funds() {
        let funds = vec![Coin::new(TEST_AMOUNT, TEST_DENOM)];
        let message = mock_instantiate_msg(true);
        assert_eq!((), message.validate_funds(&funds).unwrap());
    }

    #[test]
    fn test_validate_funds_missing_funds() {
        let funds = vec![];
        let message = mock_instantiate_msg(true);
        let error = message.validate_funds(&funds).unwrap_err();
        assert_eq!(
            ContractError::MissingFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_validate_succeeds() {
        let deps = mock_dependencies(&[]);
        let message = mock_instantiate_msg(true);
        let response = message.validate(deps.as_ref()).unwrap();
        assert_eq!((), response);
    }
}
