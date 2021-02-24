use cosmwasm_std::{
    attr, to_binary, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse, MessageInfo,
    QueryResponse, StdError,
};

use provwasm_std::{bind_name, unbind_name, Name, Names, ProvenanceMsg, ProvenanceQuerier};

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

/// Initialize the smart contract config state and bind a name to the contract address.
pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse<ProvenanceMsg>, ContractError> {
    // Create contract config state.
    let state = State {
        contract_owner: info.sender.clone(),
        contract_name: msg.name.clone(),
    };

    // Save contract config state.
    config(deps.storage).save(&state)?;

    // Create a bind name message
    let bind_name_msg = bind_name(&msg.name, &env.contract.address);

    // Dispatch message to handler and emit events
    Ok(InitResponse {
        messages: vec![bind_name_msg], // Routed to the name module handler
        attributes: vec![
            attr("integration_test", "v2"),
            attr("action", "provwasm.contracts.name.init"),
            attr("contract_name", msg.name),
            attr("contract_owner", info.sender),
        ],
    })
}

/// Handle messages that bind names under the contract root name.
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    match msg {
        HandleMsg::BindPrefix { prefix } => try_bind_prefix(deps, env, info, prefix),
        HandleMsg::UnbindPrefix { prefix } => try_unbind_prefix(deps, info, prefix),
    }
}

// Bind a name under the contract root name.
pub fn try_bind_prefix(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    prefix: String,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    // Load contract state
    let state = config_read(deps.storage).load()?;

    // Validate the message sender is the contact owner.
    if info.sender != state.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    // Create a name using the prefix
    let name = format!("{}.{}", prefix, state.contract_name);

    // Create a message that will set the marker pointer.
    let bind_name_msg = bind_name(&name, &env.contract.address);

    // Dispatch message to handler and emit events
    Ok(HandleResponse {
        messages: vec![bind_name_msg],
        attributes: vec![
            attr("integration_test", "v2"),
            attr("action", "provwasm.contracts.name.handle.bind_prefix"),
            attr("name", name),
            attr("address", env.contract.address),
        ],
        data: None,
    })
}

// Unbind a name from the contract.
pub fn try_unbind_prefix(
    deps: DepsMut,
    info: MessageInfo,
    prefix: String,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    // Load contract state
    let state = config_read(deps.storage).load()?;

    // Validate the message sender is the contact owner.
    if info.sender != state.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    // Create a name using the prefix
    let name = format!("{}.{}", prefix, state.contract_name);

    // Create a message that will set the marker pointer.
    let unbind_name_msg = unbind_name(&name);

    // Dispatch message to handler and emit events
    Ok(HandleResponse {
        messages: vec![unbind_name_msg],
        attributes: vec![
            attr("integration_test", "v2"),
            attr("action", "provwasm.contracts.name.handle.unbind_prefix"),
            attr("name", name),
        ],
        data: None,
    })
}

/// Handle query requests for the provenance name module. The queries handled here are not bound to
/// the contract name or address.
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::Resolve { name } => try_resolve(deps, name),
        QueryMsg::Lookup { address } => try_lookup(deps, address),
    }
}

// Use a ProvenanceQuerier to resolve the address for a name.
fn try_resolve(deps: Deps, name: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let name: Name = querier.resolve_name(&name)?;
    to_binary(&name)
}

// Use a ProvenanceQuerier to lookup all names bound to the contract address.
fn try_lookup(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let names: Names = querier.lookup_names(&address)?;
    to_binary(&names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{from_binary, CosmosMsg, StdError};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{NameMsgParams, Names, ProvenanceMsgParams};

    #[test]
    fn init_test() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);

        // Give the contract a name
        let msg = InitMsg {
            name: "contract.pb".into(),
        };

        // Ensure a message was created to bind the name to the contract address.
        let res = init(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
        match &res.messages[0] {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Name(p) => match &p {
                    NameMsgParams::BindName { name, .. } => assert_eq!(name, "contract.pb"),
                    _ => panic!("unexpected name params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_success() {
        // Init state
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = init(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = HandleMsg::BindPrefix {
            prefix: "test".into(),
        };
        let res = handle(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(1, res.messages.len());
        match &res.messages[0] {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Name(p) => match &p {
                    NameMsgParams::BindName { name, .. } => assert_eq!(name, "test.contract.pb"),
                    _ => panic!("unexpected name params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }
    #[test]
    fn unbind_name_success() {
        // Init state
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = init(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = HandleMsg::UnbindPrefix {
            prefix: "test".into(),
        };
        let res = handle(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(1, res.messages.len());
        match &res.messages[0] {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Name(p) => match &p {
                    NameMsgParams::DeleteName { name, .. } => assert_eq!(name, "test.contract.pb"),
                    _ => panic!("unexpected name params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_unauthorized() {
        // Init state
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = init(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = mock_info("other", &[]); // error: not 'sender'
        let msg = HandleMsg::BindPrefix {
            prefix: "test".into(),
        };
        let err = handle(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn unbind_name_unauthorized() {
        // Init state
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = init(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = mock_info("other", &[]); // error: not 'sender'
        let msg = HandleMsg::UnbindPrefix {
            prefix: "test".into(),
        };
        let err = handle(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn query_resolve() {
        // Create provenance mock deps with a single bound name.
        let mut deps = mock_dependencies(&[]);
        deps.querier
            .with_names(&[("a.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false)]);

        // Call the smart contract query function to resolve the address for our test name.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Resolve {
                name: "a.pb".into(),
            },
        )
        .unwrap();

        // Ensure that we got the expected address.
        let rep: Name = from_binary(&bin).unwrap();
        assert_eq!(
            rep.address.as_str(),
            "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync"
        )
    }

    #[test]
    fn query_resolve_name_not_bound() {
        // Create provenance mock deps with a single bound name.
        let mut deps = mock_dependencies(&[]);
        deps.querier
            .with_names(&[("b.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false)]);

        // Call the smart contract query function to resolve an address that is not bound.
        let err = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Resolve {
                name: "a.pb".into(),
            },
        )
        .unwrap_err();

        // Ensure the expected error was returned.
        match err {
            StdError::GenericErr { msg, .. } => {
                assert_eq!(true, msg.contains("no address bound to name"))
            }
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn query_lookup() {
        // Create provenance mock deps with two bound names.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_names(&[
            ("b.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
            ("a.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false),
        ]);

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync".into(),
            },
        )
        .unwrap();

        // Ensure that we got the expected number of records.
        let rep: Names = from_binary(&bin).unwrap();
        assert_eq!(rep.records.len(), 2);

        // Ensure that we got the expected names.
        let names: Vec<&str> = rep.records.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(true, names.contains(&"a.pb"));
        assert_eq!(true, names.contains(&"b.pb"))
    }

    #[test]
    fn query_lookup_empty() {
        // Create provenance mock deps with a bound name.
        let mut deps = mock_dependencies(&[]);
        deps.querier
            .with_names(&[("a.pb", "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync", false)]);

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: "tp1qtwv8qd8jnezaquap4pt64wvluyf59h7w9fn4m".into(),
            },
        )
        .unwrap();

        // Ensure that we got zero records.
        let rep: Names = from_binary(&bin).unwrap();
        assert_eq!(rep.records.len(), 0);
    }
}
