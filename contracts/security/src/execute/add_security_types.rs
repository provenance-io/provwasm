use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError, msg::Security},
    events::update_security_types::UpdateSecurityTypesEvent,
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the AddSecurityTypes variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the contract's
/// accepted security types.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `security_types` - The security types to be added.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.securities)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, security_types: &[Security]) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for security in security_types {
        storage::security::add_type(deps.storage, security)?;
    }

    Ok(Response::default()
        .set_action(ActionType::AddSecurityTypes {})
        .add_event(UpdateSecurityTypesEvent::new().into()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::{error::ContractError, msg::Security},
        events::update_security_types::UpdateSecurityTypesEvent,
        execute::add_security_types::handle,
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
    fn test_adds_one_security_type() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let security_types = vec![Security::new("newtag")];
        let expected_events: Vec<Event> = vec![UpdateSecurityTypesEvent::new().into()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::AddSecurityTypes {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &security_types).expect("should return an error");
        let found = storage::security::has_type(&deps.storage, &Security::new("newtag"));

        assert_eq!(true, found);
        assert_eq!(expected_events, res.events);
        assert_eq!(expected_attributes, res.attributes);
    }

    #[test]
    fn test_add_handles_duplicates() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let security_types = vec![
            Security::new("tag3"),
            Security::new("tag4"),
            Security::new("tag3"),
        ];
        let expected_events: Vec<Event> = vec![UpdateSecurityTypesEvent::new().into()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::AddSecurityTypes {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &security_types).expect("should return an error");

        for security in &security_types {
            let found = storage::security::has_type(&deps.storage, security);
            assert_eq!(true, found);
        }
        assert_eq!(expected_events, res.events);
        assert_eq!(expected_attributes, res.attributes);
    }
}
