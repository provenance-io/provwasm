use cosmwasm_std::{Coin, Deps};

use crate::{
    core::{error::ContractError, msg::ExecuteMsg},
    util::validate::{Validate, ValidateResult},
};

impl Validate for ExecuteMsg {
    /// Performs basic error checking on the ExecuteMsg
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// let msg = ExecuteMsg::ChangeOwner {new_owner: Addr::unchecked("new_owner")};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, _deps: Deps) -> ValidateResult {
        Ok(())
    }

    /// Performs basic error checking on ExecuteMsg.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// let msg = ExecuteMsg::ChangeOwner {new_owner: Addr::unchecked("new_owner")};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult {
        match self {
            ExecuteMsg::ChangeOwner { new_owner: _ } => {
                if !funds.is_empty() {
                    return Err(ContractError::UnexpectedFunds {});
                }
                Ok(())
            }
        }
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
            msg::mock_change_owner_msg,
        },
        util::validate::Validate,
    };

    #[test]
    fn test_validate_always_succeeds() {
        let deps = mock_provenance_dependencies();
        let msg = mock_change_owner_msg();

        let res = msg.validate(deps.as_ref()).unwrap();
        assert_eq!((), res);
    }

    #[test]
    fn test_change_owner_should_fail_with_funds() {
        let msg = mock_change_owner_msg();
        let funds = vec![Coin::new(TEST_AMOUNT, TEST_DENOM)];

        let err = msg.validate_funds(&funds).unwrap_err();

        assert_eq!(
            ContractError::UnexpectedFunds {}.to_string(),
            err.to_string()
        );
    }

    #[test]
    fn test_change_owner_should_succeed_with_no_funds() {
        let msg = mock_change_owner_msg();
        let funds = vec![];

        let res = msg.validate_funds(&funds).unwrap();
        assert_eq!((), res);
    }
}
