use cosmwasm_std::{Env, Reply, Response, SubMsgResult};

use crate::core::{
    aliases::{ProvDepsMut, ProvTxResponse},
    error::ContractError,
};

pub fn handle(_deps: ProvDepsMut, _env: Env, reply: Reply) -> ProvTxResponse {
    match reply.result {
        SubMsgResult::Ok(_) => Ok(Response::default()),
        SubMsgResult::Err(err) => Err(ContractError::ReplyFailure(reply.id, err.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Reply, Response, SubMsgResponse, SubMsgResult};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::{constants::DEFAULT_REPLY, error::ContractError},
        reply::default::handle,
    };

    #[test]
    fn test_reply_success() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let res = handle(
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
    fn test_reply_error() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let err = handle(
            deps.as_mut(),
            env,
            Reply {
                id: DEFAULT_REPLY,
                result: SubMsgResult::Err("Internal error".to_string()),
            },
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            ContractError::ReplyFailure(DEFAULT_REPLY, "Internal error".to_string()).to_string()
        );
    }
}
