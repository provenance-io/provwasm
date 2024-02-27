use cosmwasm_std::{DepsMut, Env, Response};

use crate::{
    core::{aliases::ProvTxResponse, msg::MigrateMsg},
    util::action::{Action, ActionType},
};

/// Performs the migration logic for the Default variant of MigrateMsg.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `msg` - The Default variant of MigrateMsg provided by the user.
///
/// # Examples
/// ```
/// let msg = MigrateMsg::Default {};
/// let res = handle(deps, env, msg)?;
/// ```
pub fn handle(_deps: &DepsMut, _env: Env, _msg: MigrateMsg) -> ProvTxResponse {
    Ok(Response::new().set_action(ActionType::Migrate {}))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{core::msg::MigrateMsg, util::action::ActionType};

    use super::handle;

    #[test]
    fn test_handle() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let msg = MigrateMsg::Default {};
        let res = handle(&deps.as_mut(), env, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::Migrate {})],
            res.attributes
        );
        assert_eq!(0, res.events.len());
        assert_eq!(0, res.messages.len());
    }
}
