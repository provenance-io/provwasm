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
        match self {
            ExecuteMsg::AddSecurityTypes { security_types } => {
                for security_type in security_types {
                    let empty_category = security_type.category.is_empty();
                    let empty_name = security_type
                        .name
                        .as_ref()
                        .is_some_and(|name| name.is_empty());
                    if empty_category || empty_name {
                        return Err(ContractError::InvalidSecurityTypeFormat(
                            security_type.to_string(),
                        ));
                    }
                }
                Ok(())
            }
            ExecuteMsg::SetSecurityMultiple {
                assets,
                security: _,
            } => {
                if assets.is_empty() {
                    return Err(ContractError::NoAssetsSupplied {});
                }
                Ok(())
            }
            _ => Ok(()),
        }
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
        if !funds.is_empty() {
            return Err(ContractError::UnexpectedFunds {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{
            error::ContractError,
            msg::{ExecuteMsg, Security},
        },
        testing::{
            constants::{TEST_AMOUNT, TEST_DENOM},
            msg::{
                mock_add_tag_types_msg, mock_change_owner_msg, mock_invalid_add_tag_types_msg,
                mock_invalid_add_tag_types_with_name_msg, mock_remove_tag_msg,
                mock_remove_tag_types_msg, mock_set_security_multiple_msg, mock_set_tag_msg,
            },
        },
        util::validate::Validate,
    };

    #[test]
    fn test_validate_succeeds_for_messages() {
        let deps = mock_provenance_dependencies();
        let msgs: Vec<ExecuteMsg> = vec![
            mock_add_tag_types_msg(),
            mock_remove_tag_types_msg(),
            mock_change_owner_msg(),
            mock_set_tag_msg(&Addr::unchecked("test")),
        ];

        for msg in msgs {
            let res = msg.validate(deps.as_ref()).unwrap();
            assert_eq!((), res);
        }
    }

    #[test]
    fn test_validate_checks_for_invalid_category_add_security_type() {
        let deps = mock_provenance_dependencies();
        let msgs: Vec<ExecuteMsg> = vec![mock_invalid_add_tag_types_msg()];

        for msg in msgs {
            let res = msg.validate(deps.as_ref()).unwrap_err();
            assert_eq!(
                ContractError::InvalidSecurityTypeFormat(Security::new("").to_string()).to_string(),
                res.to_string()
            );
        }
    }

    #[test]
    fn test_validate_checks_for_missing_assets_in_set_multiple() {
        let deps = mock_provenance_dependencies();
        let msgs: Vec<ExecuteMsg> = vec![mock_set_security_multiple_msg(&vec![])];

        for msg in msgs {
            let res = msg.validate(deps.as_ref()).unwrap_err();
            assert_eq!(
                ContractError::NoAssetsSupplied {}.to_string(),
                res.to_string()
            );
        }
    }

    #[test]
    fn test_validate_checks_for_invalid_name_add_security_type() {
        let deps = mock_provenance_dependencies();
        let msgs: Vec<ExecuteMsg> = vec![mock_invalid_add_tag_types_with_name_msg()];

        for msg in msgs {
            let res = msg.validate(deps.as_ref()).unwrap_err();
            assert_eq!(
                ContractError::InvalidSecurityTypeFormat(
                    Security::new_with_name("category", "").to_string()
                )
                .to_string(),
                res.to_string()
            );
        }
    }

    #[test]
    fn test_messages_should_fail_with_funds() {
        let msgs: Vec<ExecuteMsg> = vec![
            mock_add_tag_types_msg(),
            mock_remove_tag_types_msg(),
            mock_change_owner_msg(),
            mock_set_tag_msg(&Addr::unchecked("test")),
            mock_remove_tag_msg(&Addr::unchecked("test")),
            mock_set_security_multiple_msg(&vec![Addr::unchecked("test")]),
        ];

        let funds = vec![Coin::new(TEST_AMOUNT, TEST_DENOM)];
        for msg in msgs {
            let err = msg.validate_funds(&funds).unwrap_err();
            assert_eq!(
                ContractError::UnexpectedFunds {}.to_string(),
                err.to_string()
            );
        }
    }

    #[test]
    fn test_change_owner_should_succeed_with_no_funds() {
        let msgs: Vec<ExecuteMsg> = vec![
            mock_add_tag_types_msg(),
            mock_remove_tag_types_msg(),
            mock_change_owner_msg(),
            mock_set_tag_msg(&Addr::unchecked("test")),
            mock_remove_tag_msg(&Addr::unchecked("test")),
            mock_set_security_multiple_msg(&vec![Addr::unchecked("test")]),
        ];

        let funds = vec![];
        for msg in msgs {
            let res = msg.validate_funds(&funds).unwrap();
            assert_eq!((), res);
        }
    }
}
