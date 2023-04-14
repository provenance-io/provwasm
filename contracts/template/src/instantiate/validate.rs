use cosmwasm_std::Coin;

use crate::{
    core::{aliases::ProvDeps, error::ContractError, msg::InstantiateMsg},
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
    /// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), fee: Fee {recipient: Some(Addr::unchecked("owner")), amount: Coin::new(0, "nhash"),},};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, _deps: ProvDeps) -> ValidateResult {
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
    /// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), fee: Fee {recipient: Some(Addr::unchecked("owner")), amount: Coin::new(0, "nhash"),},};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
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
