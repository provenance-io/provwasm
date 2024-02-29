use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError, msg::Security},
    events::update_security_types::UpdateSecurityTypesEvent,
    storage::{self, asset},
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the RemoveSecurityTypes variant of ExecuteMsg.
///
/// Removes specified security types from the contract's store.
/// The sender must be the owner and each specified security type
/// must not be in use to succeed.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `security_types` - The security types to be removed.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.security_types)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, security_types: &[Security]) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for security in security_types {
        if asset::has_security(deps.storage, security) {
            return Err(ContractError::SecurityInUse(security.to_string()));
        }

        storage::security::remove_type(deps.storage, security);
    }

    Ok(Response::default()
        .set_action(ActionType::RemoveSecurityTypes {})
        .add_event(UpdateSecurityTypesEvent::new().into()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{error::ContractError, msg::Security},
        events::update_security_types::UpdateSecurityTypesEvent,
        execute::remove_security_types::handle,
        storage,
        testing::{
            constants::{OWNER, TAG1, TAG2},
            setup,
        },
        util::action::ActionType,
    };

    #[test]
    fn test_must_be_owner() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked("invalid");
        let security_types = vec![Security::new(TAG1), Security::new(TAG2)];

        setup::mock_contract(deps.as_mut());

        let error =
            handle(deps.as_mut(), sender, &security_types).expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_security_type_in_use() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("addr");
        let security_types = vec![Security::new(TAG1), Security::new(TAG2)];

        setup::mock_contract(deps.as_mut());
        storage::asset::set_security(deps.as_mut().storage, &asset_addr, &Security::new(TAG1))
            .expect("should set security");
        let error =
            handle(deps.as_mut(), sender, &security_types).expect_err("should return an error");

        assert_eq!(
            ContractError::SecurityInUse(Security::new(TAG1).to_string()).to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_single_remove() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let security_types = vec![Security::new(TAG1)];
        let expected_attributes: Vec<Attribute> = vec![ActionType::RemoveSecurityTypes {}.into()];
        let expected_events: Vec<Event> = vec![UpdateSecurityTypesEvent {}.into()];

        setup::mock_contract(deps.as_mut());
        let res =
            handle(deps.as_mut(), sender, &security_types).expect("should not return an error");

        let found = storage::security::has_type(&deps.storage, &Security::new(TAG1));
        assert_eq!(false, found);
        let found = storage::security::has_type(&deps.storage, &Security::new(TAG2));
        assert_eq!(true, found);
        assert_eq!(expected_attributes, res.attributes);
        assert_eq!(expected_events, res.events);
    }

    #[test]
    fn test_multi_remove() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let security_types = vec![Security::new(TAG1), Security::new(TAG2)];
        let expected_attributes: Vec<Attribute> = vec![ActionType::RemoveSecurityTypes {}.into()];
        let expected_events: Vec<Event> = vec![UpdateSecurityTypesEvent {}.into()];

        setup::mock_contract(deps.as_mut());
        let res =
            handle(deps.as_mut(), sender, &security_types).expect("should not return an error");

        for security in &security_types {
            let found = storage::security::has_type(&deps.storage, security);
            assert_eq!(false, found);
        }
        assert_eq!(expected_attributes, res.attributes);
        assert_eq!(expected_events, res.events);
    }
}
