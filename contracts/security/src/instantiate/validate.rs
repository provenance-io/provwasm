use cosmwasm_std::{Coin, Deps};

use crate::{
    core::{error::ContractError, msg::InstantiateMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for InstantiateMsg {
    /// Performs basic error checking on the InstantiateMsg
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), tag_types: vec!["tag1".to_string()]};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, _deps: Deps) -> ValidateResult {
        // TODO Check length of tag_types
        // TODO Is this enough for validating an owner address?
        Ok(())
    }

    /// Performs basic error checking on InstantiateMsg.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), tag_types: vec!["tag1".to_string()]};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult {
        if !funds.is_empty() {
            return Err(ContractError::UnexpectedFunds {});
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
    fn test_validate_funds_has_no_funds() {
        let funds = vec![];
        let message = mock_instantiate_msg();
        assert_eq!((), message.validate_funds(&funds).unwrap());
    }

    #[test]
    fn test_validate_funds_unexpected_funds() {
        let funds = vec![Coin::new(TEST_AMOUNT, TEST_DENOM)];
        let message = mock_instantiate_msg();
        let error = message.validate_funds(&funds).unwrap_err();
        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_validate_succeeds() {
        let deps = mock_provenance_dependencies();
        let message = mock_instantiate_msg();
        let response = message.validate(deps.as_ref()).unwrap();
        assert_eq!((), response);
    }
}
