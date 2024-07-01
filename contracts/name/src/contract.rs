use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};
use provwasm_std::types::provenance::name::v1::{
    MsgBindNameRequest, MsgDeleteNameRequest, NameQuerier, NameRecord,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InitMsg, QueryMsg};
use crate::state::{State, CONFIG};

/// Initialize the smart contract config state and bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create contract config state.
    let state = State {
        contract_owner: info.sender.clone(),
        contract_name: msg.name.clone(),
    };

    // Save contract config state.
    CONFIG.save(deps.storage, &state)?;

    let split: Vec<&str> = msg.name.splitn(2, '.').collect();
    let record = split.first();
    let parent = split.last();

    match (parent, record) {
        (Some(parent), Some(record)) => {
            // Create a bind name message
            let bind_name_msg = MsgBindNameRequest {
                parent: Some(NameRecord {
                    name: parent.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
                record: Some(NameRecord {
                    name: record.to_string(),
                    address: env.contract.address.to_string(),
                    restricted: true,
                }),
            };

            // Dispatch bind name message and add event attributes.
            let res = Response::new()
                .add_message(bind_name_msg)
                .add_attribute("integration_test", "v2")
                .add_attribute("action", "provwasm.contracts.name.init")
                .add_attribute("contract_name", msg.name)
                .add_attribute("contract_owner", info.sender);
            Ok(res)
        }
        (_, _) => Err(ContractError::Std(StdError::generic_err(
            "Invalid contract name",
        ))),
    }
}

/// Handle messages that bind names under the contract root name.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BindPrefix { prefix } => try_bind_prefix(deps, env, info, prefix),
        ExecuteMsg::UnbindPrefix { prefix } => try_unbind_prefix(deps, env, info, prefix),
    }
}

// Bind a name under the contract root name.
pub fn try_bind_prefix(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    prefix: String,
) -> Result<Response, ContractError> {
    // Load contract state
    let state = CONFIG.load(deps.storage)?;

    // Validate the message sender is the contact owner.
    if info.sender != state.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    // Create a name using the prefix
    let name = format!("{}.{}", prefix, state.contract_name);

    // Create a message that will bind a subdomain of the contract.
    let bind_name_msg = MsgBindNameRequest {
        parent: Some(NameRecord {
            name: state.contract_name,
            address: env.contract.address.to_string(),
            restricted: true,
        }),
        record: Some(NameRecord {
            name: prefix,
            address: env.contract.address.to_string(),
            restricted: true,
        }),
    };

    // Dispatch message to handler and emit events
    let res = Response::new()
        .add_message(bind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.name.try_bind_prefix")
        .add_attribute("name", name)
        .add_attribute("address", env.contract.address);
    Ok(res)
}

// Unbind a name from the contract.
pub fn try_unbind_prefix(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    prefix: String,
) -> Result<Response, ContractError> {
    // Load contract state
    let state = CONFIG.load(deps.storage)?;

    // Validate the message sender is the contact owner.
    if info.sender != state.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    // Create a name using the prefix
    let name = format!("{}.{}", prefix, state.contract_name);

    // Create a message that will set the marker pointer.
    let unbind_name_msg = MsgDeleteNameRequest {
        record: Some(NameRecord {
            name: name.clone(),
            address: env.contract.address.to_string(),
            restricted: true,
        }),
    };

    // Dispatch message to handler and emit events
    let res = Response::new()
        .add_message(unbind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.name.try_unbind_prefix")
        .add_attribute("name", name);
    Ok(res)
}

/// Handle query requests for the provenance name module. The queries handled here are not bound to
/// the contract name or address.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::Resolve { name } => try_resolve(deps, name),
        QueryMsg::Lookup { address } => try_lookup(deps, address),
        QueryMsg::Params {} => try_params(deps),
    }
}

// Use a ProvenanceQuerier to resolve the address for a name.
fn try_resolve(deps: Deps, name: String) -> Result<QueryResponse, StdError> {
    let querier = NameQuerier::new(&deps.querier);
    let name = querier.resolve(name)?;
    to_json_binary(&name)
}

// Use a ProvenanceQuerier to resolve the address for a name.
fn try_params(deps: Deps) -> Result<QueryResponse, StdError> {
    let querier = NameQuerier::new(&deps.querier);
    let params = querier.params()?;
    to_json_binary(&params)
}

// Use a ProvenanceQuerier to lookup all names bound to the contract address.
fn try_lookup(deps: Deps, address: String) -> Result<QueryResponse, StdError> {
    deps.api.addr_validate(&address)?;
    let querier = NameQuerier::new(&deps.querier);
    let names = querier.reverse_lookup(address, None)?;
    to_json_binary(&names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{message_info, mock_env};
    use cosmwasm_std::{from_json, Addr, AnyMsg, Binary, CosmosMsg};
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::types::provenance::name::v1::{
        QueryResolveRequest, QueryResolveResponse, QueryReverseLookupRequest,
        QueryReverseLookupResponse,
    };

    #[test]
    fn init_test() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);

        // Give the contract a name
        let msg = InitMsg {
            name: "contract.pb".into(),
        };

        let contract_address = env.contract.address.to_string();

        // Ensure a message was created to bind the name to the contract address.
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgBindNameRequest {
                    parent: Some(NameRecord {
                        name: "pb".to_string(),
                        address: contract_address.clone(),
                        restricted: true,
                    }),
                    record: Some(NameRecord {
                        name: "contract".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgBindNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_success() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = ExecuteMsg::BindPrefix {
            prefix: "test".into(),
        };

        let contract_address = env.contract.address.to_string();

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgBindNameRequest {
                    parent: Some(NameRecord {
                        name: "contract.pb".to_string(),
                        address: contract_address.clone(),
                        restricted: true,
                    }),
                    record: Some(NameRecord {
                        name: "test".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgBindNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn unbind_name_success() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Bind a name
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = ExecuteMsg::UnbindPrefix {
            prefix: "test".into(),
        };

        let contract_address = env.contract.address.to_string();

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                let expected: Binary = MsgDeleteNameRequest {
                    record: Some(NameRecord {
                        name: "test.contract.pb".to_string(),
                        address: contract_address,
                        restricted: true,
                    }),
                }
                .into();

                assert_eq!(type_url, "/provenance.name.v1.MsgDeleteNameRequest");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn bind_name_unauthorized() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = message_info(&Addr::unchecked("other"), &[]); // error: not 'sender'
        let msg = ExecuteMsg::BindPrefix {
            prefix: "test".into(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn unbind_name_unauthorized() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[]);
        let msg = InitMsg {
            name: "contract.pb".into(),
        };
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap(); // Panics on error

        // Try to bind a name with some other sender address
        let env = mock_env();
        let info = message_info(&Addr::unchecked("other"), &[]); // error: not 'sender'
        let msg = ExecuteMsg::UnbindPrefix {
            prefix: "test".into(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn query_resolve() {
        // Create provenance mock deps with a single bound name.

        let mut deps = mock_provenance_dependencies();

        let mock_response = QueryResolveResponse {
            address: "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync".to_string(),
            restricted: false,
        };

        QueryResolveRequest::mock_response(&mut deps.querier, mock_response);

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
        let rep: QueryResolveResponse = from_json(bin).unwrap();
        assert_eq!(rep.address, "tp1y0txdp3sqmxjvfdaa8hfvwcljl8ugcfv26uync")
    }

    #[test]
    fn query_lookup() {
        // Create provenance mock deps with two bound names.
        let mut deps = mock_provenance_dependencies();

        let mock_response = QueryReverseLookupResponse {
            name: vec!["b.pb".to_string(), "a.pb".to_string()],
            pagination: None,
        };

        QueryReverseLookupRequest::mock_response(&mut deps.querier, mock_response.clone());

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: deps.api.addr_make("address").into(),
            },
        )
        .unwrap();

        // Ensure that we got the expected number of records.
        let rep: QueryReverseLookupResponse = from_json(bin).unwrap();
        assert_eq!(rep, mock_response);
    }

    #[test]
    fn query_lookup_empty() {
        // Create provenance mock deps with a bound name.
        let mut deps = mock_provenance_dependencies();
        let mock_response = QueryReverseLookupResponse {
            name: vec![],
            pagination: None,
        };

        QueryReverseLookupRequest::mock_response(&mut deps.querier, mock_response.clone());

        // Call the smart contract query function to lookup names bound to an address.
        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Lookup {
                address: deps.api.addr_make("address2").into(),
            },
        )
        .unwrap();

        // Ensure that we got zero records.
        let rep: QueryReverseLookupResponse = from_json(bin).unwrap();
        assert_eq!(rep, mock_response);
    }
}
