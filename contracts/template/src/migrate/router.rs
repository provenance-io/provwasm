use cosmwasm_std::{DepsMut, Env};

use crate::core::{aliases::ProvTxResponse, msg::MigrateMsg};

use super::default;

/// Routes the migrate message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The migration being ran by the user.
///
/// # Examples
/// ```
/// let msg = MigrateMsg::Default {};
/// let res = route(deps, env, msg)?;
/// ```
pub fn route(deps: &DepsMut, env: Env, msg: MigrateMsg) -> ProvTxResponse {
    match msg {
        MigrateMsg::Default {} => default::handle(deps, env, msg),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{core::msg::MigrateMsg, util::action::ActionType};

    use super::route;

    #[test]
    fn test_default_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let msg = MigrateMsg::Default {};
        let res = route(&deps.as_mut(), env, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::Migrate {})],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }
}
