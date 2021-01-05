use cosmwasm_std::{
    attr, to_binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo, QueryResponse,
    StdError,
};

use provwasm_std::{bind_name, ProvenanceMsg, ProvenanceQuerier, Scope};

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, State};

/// Initialize the smart contract config state and bind a name to the contract address.
pub fn init(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse<ProvenanceMsg>, ContractError> {
    // Create and save contract config state.
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    // Dispatch a 'bind name' message to the name module handler and emit events
    let bind_name_msg = bind_name(msg.name.clone(), env.contract.address);
    Ok(InitResponse {
        messages: vec![bind_name_msg],
        attributes: vec![
            attr("integration_test", "v2"),
            attr("action", "provwasm.contracts.scope.init"),
            attr("contract_name", msg.name),
        ],
    })
}

/// Handle does nothing
pub fn handle(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    // Do nothing
    Ok(HandleResponse::default())
}

/// Handle scope query requests for the provenance metadata module.
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetScope { id } => try_get_scope(deps, id),
    }
}

// Use a ProvenanceQuerier to get a scope by ID.
fn try_get_scope(deps: Deps, id: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let scope: Scope = querier.get_scope(id)?;
    to_binary(&scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{from_binary, Binary};
    use provwasm_mocks::mock_dependencies;
    use std::fs::File;
    use std::io::Read;

    fn read_test_scope_from_file() -> Binary {
        let filename = "testdata/scope.json";
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                Binary::from(content.as_bytes())
            }
            Err(error) => {
                panic!("Error opening file {}: {}", filename, error);
            }
        }
    }

    #[test]
    fn init_test() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init contract state
        let res = init(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Just check that one message was created (bind name to contract address).
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn query_test() {
        // Read a scope from file
        let bin = read_test_scope_from_file();
        let expected_scope: Scope = from_binary(&bin).unwrap();

        // Create custom deps with the scope.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_scopes(vec![expected_scope.clone()]);

        // Call the contract query function.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetScope {
                id: "fbd81e76-fb4b-44f4-98dd-96f78b654f47".into(),
            },
        )
        .unwrap();

        // Ensure we got the expected scope
        let scope: Scope = from_binary(&bin).unwrap();
        assert_eq!(scope, expected_scope)
    }
}
