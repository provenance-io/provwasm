use cosmwasm_std::{Coin, Deps};

use crate::{
    core::{error::ContractError, msg::InstantiateMsg},
    util::validate::Validate,
};

impl Validate for InstantiateMsg {
    fn validate(&self, _deps: Deps) -> Result<(), ContractError> {
        Ok(())
    }

    fn validate_funds(&self, funds: &[Coin]) -> Result<(), ContractError> {
        if !funds.is_empty() {
            return Err(ContractError::FundsPresent {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use provwasm_mocks::mock_provenance_dependencies;

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
        let message = mock_instantiate_msg();
        let result = message.validate_funds(&funds);
        assert!(result.is_err());
        assert_eq!(
            ContractError::FundsPresent {}.to_string(),
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn test_validate_funds_empty() {
        let funds = vec![];
        let message = mock_instantiate_msg();
        let result = message.validate_funds(&funds);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_succeeds() {
        let deps = mock_provenance_dependencies();
        let message = mock_instantiate_msg();
        let response = message.validate(deps.as_ref()).unwrap();
        assert_eq!((), response);
    }
}
