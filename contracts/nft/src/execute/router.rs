use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::core::msg::ExecuteMsg;

use super::{approve, approve_all, burn, mint, revoke, revoke_all, send, transfer};

/// Routes the execute message to the appropriate handler based on the message's variant.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `info` - Contains the message signer and any sent funds.
/// * `msg` - The execute variant being ran by the user.
///
/// # Examples
/// ```
/// use cosmwasm_std::Addr;
/// use scope::core::msg::ExecuteMsg;
/// use scope::execute::router::route;
/// let msg = ExecuteMsg::ChangeOwner {new_owner: Addr::unchecked("new_owner_addr")};
/// let res = route(deps, env, info, msg)?;
/// ```
pub fn route(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ProvTxResponse {
    match msg {
        ExecuteMsg::Mint {
            token_id,
            owner,
            token_uri,
        } => mint::handle(deps, info, token_id, owner, token_uri),
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => approve::handle(deps, env, info, spender, token_id, expires),
        ExecuteMsg::Revoke { spender, token_id } => {
            revoke::handle(deps, env, info, spender, token_id)
        }
        ExecuteMsg::ApproveAll { operator, expires } => {
            approve_all::handle(deps, env, info, operator, expires)
        }
        ExecuteMsg::RevokeAll { operator } => revoke_all::handle(deps, env, info, operator),
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => transfer::handle(deps, env, info, recipient, token_id),
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => send::handle(deps, env, info, contract, token_id, msg),
        ExecuteMsg::Burn { token_id } => burn::handle(deps, env, info, token_id),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Attribute};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        testing::{
            constants::OWNER,
            msg::mock_change_owner_msg,
            setup::{self, mock_info},
        },
        util::action::ActionType,
    };

    use super::route;

    #[test]
    fn test_change_owner_route() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info(false, OWNER);
        let msg = mock_change_owner_msg();

        setup::mock_contract(deps.as_mut());

        let res = route(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            vec![Attribute::from(ActionType::ChangeOwner {})],
            res.attributes
        );
    }
}
