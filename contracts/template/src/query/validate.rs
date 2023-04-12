use cosmwasm_std::Coin;

use crate::{
    core::msg::QueryMsg,
    util::validate::{Validate, ValidateResult},
};

impl Validate for QueryMsg {
    fn validate(&self) -> ValidateResult {
        match self {
            QueryMsg::QueryOwner {} => Ok(()),
            QueryMsg::QueryVersion {} => Ok(()),
        }
    }

    fn validate_funds(&self, _funds: &[Coin]) -> ValidateResult {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::msg::QueryMsg, util::validate::Validate};

    #[test]
    fn test_query_owner_validate_succeeds() {
        let query = QueryMsg::QueryOwner {};
        assert_eq!((), query.validate().unwrap())
    }

    #[test]
    fn test_query_version_validate_succeeds() {
        let query = QueryMsg::QueryVersion {};
        assert_eq!((), query.validate().unwrap())
    }

    #[test]
    fn test_query_funds_should_always_return_true() {
        let query1 = QueryMsg::QueryVersion {};
        let query2 = QueryMsg::QueryOwner {};
        assert_eq!((), query1.validate_funds(&[]).unwrap());
        assert_eq!((), query2.validate_funds(&[]).unwrap());
    }
}
