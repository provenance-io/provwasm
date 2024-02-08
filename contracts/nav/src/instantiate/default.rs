use cosmwasm_std::{Addr, DepsMut, Env, Response};
use cw2::set_contract_version;

use crate::{
    core::{
        aliases::ProvTxResponse,
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
    },
    storage,
    util::{
        action::{Action, ActionType},
        state::State,
    },
};

/// Performs the instantiation logic for the Default variant of InstantiateMsg.
///
/// The contract first stores the owner and tag types into the contract's storage. It then sets the contract version,
/// and returns a response with the correct action.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `owner` - The address of the contract's owner.
/// * `tag_types` - The initial tag types that this contract supports.
///
///
/// # Examples
/// ```
/// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), tag_types: vec!["tag1".as_string(), "tag2".as_string()]};
/// let res = handle(deps, env, msg.owner, msg.tags.as_slice())?;
/// ```
pub fn handle(deps: DepsMut, _: Env, owner: Addr, tag_types: &[String]) -> ProvTxResponse {
    storage::state::set(deps.storage, &State::new(owner))?;
    for tag in tag_types {
        storage::tag::add_type(deps.storage, tag)?;
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default().set_action(ActionType::Initialize {}))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute};
    use cw2::get_contract_version;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
        storage,
        testing::constants::OWNER,
        util::{action::ActionType, state::State},
    };

    use super::handle;

    #[test]
    fn test_handle_with_tag() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let owner = Addr::unchecked(OWNER);

        let expected_tags = vec!["tag1".to_string()];
        let expected_state = State::new(owner.clone());

        let res = handle(deps.as_mut(), env.clone(), owner, expected_tags.as_slice()).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        let tags = storage::tag::get_types(&deps.storage).unwrap();
        let contract_version = get_contract_version(&deps.storage).unwrap();

        assert_eq!(expected_state, state);
        assert_eq!(expected_tags, tags);
        assert_eq!(CONTRACT_NAME, contract_version.contract);
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(
            vec![Attribute::from(ActionType::Initialize {})],
            res.attributes
        );
    }

    #[test]
    fn test_handle_with_multiple_tags() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let owner = Addr::unchecked(OWNER);

        let expected_tags = vec!["tag1".to_string(), "tag2".to_string()];
        let expected_state = State::new(owner.clone());

        let res = handle(deps.as_mut(), env.clone(), owner, expected_tags.as_slice()).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        let tags = storage::tag::get_types(&deps.storage).unwrap();
        let contract_version = get_contract_version(&deps.storage).unwrap();

        assert_eq!(expected_state, state);
        assert_eq!(expected_tags, tags);
        assert_eq!(CONTRACT_NAME, contract_version.contract);
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(
            vec![Attribute::from(ActionType::Initialize {})],
            res.attributes
        );
    }

    #[test]
    fn test_handle_with_no_tags() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let owner = Addr::unchecked(OWNER);

        let expected_tags = vec![];
        let expected_state = State::new(owner.clone());

        let res = handle(deps.as_mut(), env.clone(), owner, expected_tags.as_slice()).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        let tags = storage::tag::get_types(&deps.storage).unwrap();
        let contract_version = get_contract_version(&deps.storage).unwrap();

        assert_eq!(expected_state, state);
        assert_eq!(expected_tags, tags);
        assert_eq!(CONTRACT_NAME, contract_version.contract);
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(
            vec![Attribute::from(ActionType::Initialize {})],
            res.attributes
        );
    }
}
