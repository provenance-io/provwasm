use cosmwasm_std::{DepsMut, Env, Reply};

use crate::core::{aliases::ProvTxResponse, constants::DEFAULT_REPLY, error::ContractError};

use super::default;

/// Routes the reply to the appropriate handler based on the reply's id.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `reply` - The response sent from the other contract.
///
/// # Examples
/// ```
/// let res = route(deps, env, reply)?;
/// ```
pub fn route(deps: DepsMut, env: Env, reply: Reply) -> ProvTxResponse {
    match reply.id {
        DEFAULT_REPLY => default::handle(deps, env, reply),
        _ => Err(ContractError::UnexpectedReplyId(reply.id)),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Reply, Response, SubMsgResponse, SubMsgResult};
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::core::{constants::DEFAULT_REPLY, error::ContractError};

    use super::route;

    #[test]
    fn test_default_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let res = route(
            deps.as_mut(),
            env,
            Reply {
                id: DEFAULT_REPLY,
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: None,
                }),
            },
        )
        .unwrap();
        assert_eq!(res, Response::default());
    }

    #[test]
    fn test_invalid_route() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let err = route(
            deps.as_mut(),
            env,
            Reply {
                id: 1,
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![],
                    data: None,
                }),
            },
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            ContractError::UnexpectedReplyId(1).to_string()
        );
    }
}
