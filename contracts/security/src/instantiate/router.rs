use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::core::{aliases::ProvTxResponse, msg::InstantiateMsg};

use super::default;

/// Routes the instantiate message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `info` - Contains the message signer and any sent funds.
/// * `msg` - The instantiate variant being ran by the user.
///
/// # Examples
/// ```
/// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), tag_types: vec![Security::new("tag1")]};
/// let res = route(deps, env, info, msg)?;
/// ```
pub fn route(deps: DepsMut, env: Env, _info: MessageInfo, msg: InstantiateMsg) -> ProvTxResponse {
    match msg {
        InstantiateMsg::Default {
            owner,
            security_types,
        } => default::handle(deps, env, owner, security_types.as_slice()),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        instantiate::router::route,
        testing::{constants::CREATOR, msg::mock_instantiate_msg, setup::mock_info},
        util::action::ActionType,
    };

    #[test]
    fn test_route() {
        let mut deps = mock_provenance_dependencies();
        Addr::unchecked("blah");
        let env = mock_env();
        let msg = mock_instantiate_msg();
        let info = mock_info(true, CREATOR);

        let res = route(deps.as_mut(), env.clone(), info, msg).unwrap();

        assert_eq!(
            vec![Attribute::from(ActionType::Initialize {})],
            res.attributes
        );
    }
}
