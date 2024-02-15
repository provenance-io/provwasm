use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{aliases::ProvTxResponse, error::ContractError, msg::Security},
    events::update_tag_types::UpdateTagTypesEvent,
    storage,
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the AddTagTypes variant of ExecuteMsg.
///
/// If the sender is the owner of the contract, then the contract will update the contract's
/// accepted tag types.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `tag_types` - The tags to be added.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.tags)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, tag_types: &[Security]) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for tag in tag_types {
        let tag_string: String = tag.into();
        storage::tag::add_type(deps.storage, &tag_string)?;
    }

    Ok(Response::default()
        .set_action(ActionType::AddTagTypes {})
        .add_event(UpdateTagTypesEvent::new().into()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::error::ContractError,
        events::update_tag_types::UpdateTagTypesEvent,
        execute::add_tag_types::handle,
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
        let tag_types = vec![TAG1.to_string(), TAG2.to_string()];

        setup::mock_contract(deps.as_mut());

        let error = handle(deps.as_mut(), sender, &tag_types).expect_err("should return an error");

        assert_eq!(
            ContractError::Unauthorized {}.to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_adds_one_tag() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let tag_types = vec!["new tag".to_string()];
        let expected_events: Vec<Event> = vec![UpdateTagTypesEvent::new().into()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::AddTagTypes {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &tag_types).expect("should return an error");
        let found = storage::tag::has_type(&deps.storage, "new tag");

        assert_eq!(true, found);
        assert_eq!(expected_events, res.events);
        assert_eq!(expected_attributes, res.attributes);
    }

    #[test]
    fn test_adds_tags_handles_duplicates() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let tag_types = vec!["tag3".to_string(), "tag4".to_string(), "tag3".to_string()];
        let expected_events: Vec<Event> = vec![UpdateTagTypesEvent::new().into()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::AddTagTypes {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &tag_types).expect("should return an error");

        for tag in tag_types {
            let found = storage::tag::has_type(&deps.storage, &tag);
            assert_eq!(true, found);
        }
        assert_eq!(expected_events, res.events);
        assert_eq!(expected_attributes, res.attributes);
    }

    #[test]

    fn test_overwrites_tags() {}
}
