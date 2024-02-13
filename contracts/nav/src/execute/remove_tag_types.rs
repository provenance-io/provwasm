use cosmwasm_std::{Addr, DepsMut, Response};

use crate::{
    core::{
        aliases::{AssetTag, ProvTxResponse},
        error::ContractError,
    },
    events::update_tag_types::UpdateTagTypesEvent,
    storage::{self, asset},
    util::action::{Action, ActionType},
};

/// Performs the execute logic for the RemoveTagTypes variant of ExecuteMsg.
///
/// Removes specified tag types from the contract's store.
/// The sender must be the owner and each specified tag type
/// must not be in use to succeed.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `sender` - The address of the message signer.
/// * `tag_types` - The tags to be removed.
///
/// # Examples
/// ```
/// let res = handle(deps, env, info.sender, msg.tags)?;
/// ```
pub fn handle(deps: DepsMut, sender: Addr, tag_types: &[AssetTag]) -> ProvTxResponse {
    if !storage::state::is_owner(deps.storage, &sender)? {
        return Err(ContractError::Unauthorized {});
    }

    for tag in tag_types {
        if asset::has_tag(deps.storage, tag) {
            return Err(ContractError::TagInUse(tag.clone()));
        }

        storage::tag::remove_type(deps.storage, tag);
    }

    Ok(Response::default()
        .set_action(ActionType::RemoveTagTypes {})
        .add_event(UpdateTagTypesEvent::new().into()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::error::ContractError,
        events::update_tag_types::UpdateTagTypesEvent,
        execute::remove_tag_types::handle,
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
    fn test_tag_in_use() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let asset_addr = Addr::unchecked("addr");
        let tag_types = vec![TAG1.to_string(), TAG2.to_string()];

        setup::mock_contract(deps.as_mut());
        storage::asset::set_tag(deps.as_mut().storage, &asset_addr, TAG1).expect("should set tag");
        let error = handle(deps.as_mut(), sender, &tag_types).expect_err("should return an error");

        assert_eq!(
            ContractError::TagInUse(TAG1.to_string()).to_string(),
            error.to_string()
        );
    }

    #[test]
    fn test_single_remove() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let tag_types = vec![TAG1.to_string()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::RemoveTagTypes {}.into()];
        let expected_events: Vec<Event> = vec![UpdateTagTypesEvent {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &tag_types).expect("should not return an error");

        let found = storage::tag::has_type(&deps.storage, TAG1);
        assert_eq!(false, found);
        let found = storage::tag::has_type(&deps.storage, TAG2);
        assert_eq!(true, found);
        assert_eq!(expected_attributes, res.attributes);
        assert_eq!(expected_events, res.events);
    }

    #[test]
    fn test_multi_remove() {
        let mut deps = mock_provenance_dependencies();
        let sender = Addr::unchecked(OWNER);
        let tag_types = vec![TAG1.to_string(), TAG2.to_string()];
        let expected_attributes: Vec<Attribute> = vec![ActionType::RemoveTagTypes {}.into()];
        let expected_events: Vec<Event> = vec![UpdateTagTypesEvent {}.into()];

        setup::mock_contract(deps.as_mut());
        let res = handle(deps.as_mut(), sender, &tag_types).expect("should not return an error");

        for tag in tag_types {
            let found = storage::tag::has_type(&deps.storage, tag.as_str());
            assert_eq!(false, found);
        }
        assert_eq!(expected_attributes, res.attributes);
        assert_eq!(expected_events, res.events);
    }
}
