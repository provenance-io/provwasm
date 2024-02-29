use cosmwasm_std::{Coin, Deps};

use crate::{
    core::msg::QueryMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for QueryMsg {
    /// Performs basic error checking on the QueryMsg
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// let msg = QueryMsg::QueryVersion {};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, _deps: Deps) -> ValidateResult {
        Ok(())
    }

    /// Performs basic error checking on QueryMsg.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// let msg = QueryMsg::QueryVersion {};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{core::msg::QueryMsg, util::validate::Validate};

    #[test]
    fn test_query_owner_validate_succeeds() {
        let query = QueryMsg::QueryOwner {};
        let deps = mock_provenance_dependencies();
        assert_eq!((), query.validate(deps.as_ref()).unwrap());
    }

    #[test]
    fn test_query_version_validate_succeeds() {
        let query = QueryMsg::QueryVersion {};
        let deps = mock_provenance_dependencies();
        assert_eq!((), query.validate(deps.as_ref()).unwrap())
    }

    #[test]
    fn test_query_funds_should_always_return_true() {
        let query1 = QueryMsg::QueryVersion {};
        let query2 = QueryMsg::QueryOwner {};
        assert_eq!((), query1.validate_funds(&[]).unwrap());
        assert_eq!((), query2.validate_funds(&[]).unwrap());
    }
}
