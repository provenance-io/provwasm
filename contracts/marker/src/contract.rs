use cosmwasm_std::{
    attr, to_binary, Coin, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse,
    MessageInfo, QueryResponse, StdError,
};
use provwasm_std::{
    activate_marker, bind_name, create_marker, finalize_marker, grant_marker_access_all,
    withdraw_marker_coins, ProvenanceMsg, ProvenanceQuerier,
};

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, State};

/// Initialize the smart contract config state, then bind a name to the contract address.
pub fn init(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse<ProvenanceMsg>, ContractError> {
    // Create and save state
    let name = &msg.name;
    config(deps.storage).save(&State {
        contract_name: String::from(name),
    })?;

    // Create a name for the contract
    let msg = bind_name(name.clone(), env.contract.address);

    // Dispatch messages to the name module handler and emit an event.
    Ok(InitResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.init"),
            attr("integration_test", "v2"),
            attr("contract_name", name),
        ],
    })
}

/// Handle messages that create and interact with with native provenance markers.
pub fn handle(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    match msg {
        HandleMsg::CreateMarker { coin } => try_create_marker(coin),
        HandleMsg::GrantAccess { denom, address } => try_grant_marker_access(denom, address),
        HandleMsg::Finalize { denom } => try_finalize_marker(denom),
        HandleMsg::Activate { denom } => try_activate_marker(denom),
        HandleMsg::Withdraw { coin, recipient } => try_withdraw_marker_coins(coin, recipient),
    }
}

// Create and dispatch a message that will create a new proposed marker.
fn try_create_marker(coin: Coin) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = create_marker(coin.clone());
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.create"),
            attr("integration_test", "v2"),
            attr("marker_denom", coin.denom),
            attr("marker_supply", coin.amount.to_string()),
        ],
        data: None,
    })
}

// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_marker_access(
    denom: String,
    address: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = grant_marker_access_all(denom.clone(), address.clone());
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.grant_access"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
            attr("marker_addr", address.to_string()),
        ],
        data: None,
    })
}

// Create and dispatch a message that will finalize a proposed marker.
fn try_finalize_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = finalize_marker(denom.clone());
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.finalize"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
        ],
        data: None,
    })
}

// Create and dispatch a message that will activate a finalized marker.
fn try_activate_marker(denom: String) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = activate_marker(denom.clone());
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.activate"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
        ],
        data: None,
    })
}

// Create and dispatch a message that will withdraw coins from a marker.
fn try_withdraw_marker_coins(
    coin: Coin,
    recipient: HumanAddr,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let msg = withdraw_marker_coins(coin.clone(), recipient.clone());
    Ok(HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.withdraw"),
            attr("integration_test", "v2"),
            attr("marker_denom", coin.denom),
            attr("marker_amount", coin.amount.to_string()),
            attr("marker_recipient", recipient.to_string()),
        ],
        data: None,
    })
}

/// Handle query requests for the provenance marker module.
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetByAddress { address } => try_get_marker_by_address(deps, address),
        QueryMsg::GetByDenom { denom } => try_get_marker_by_denom(deps, denom),
    }
}

// Query a marker by address.
fn try_get_marker_by_address(deps: Deps, address: HumanAddr) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_address(address)?;
    to_binary(&marker)
}

// Query a marker by denom.
fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let querier = ProvenanceQuerier::new(&deps.querier);
    let marker = querier.get_marker_by_denom(denom)?;
    to_binary(&marker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coin, from_binary, Binary};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::Marker;
    use std::fs::File;
    use std::io::Read;

    fn read_test_marker_from_file() -> Binary {
        let filename = "testdata/marker.json";
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
    fn valid_init() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        // Call init
        let res = init(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap();
        // Ensure a message was created to bind the name to the contract address.
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn create_marker() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create a marker
        let msg = HandleMsg::CreateMarker {
            coin: coin(420, "budz"),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn grant_access() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create a marker
        let msg = HandleMsg::GrantAccess {
            denom: "budz".into(),
            address: HumanAddr::from("tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn finalize_marker() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create a marker
        let msg = HandleMsg::Finalize {
            denom: "budz".into(),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn activate_marker() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create a marker
        let msg = HandleMsg::Activate {
            denom: "budz".into(),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn withdraw_coins() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create a marker
        let msg = HandleMsg::Withdraw {
            coin: coin(20, "budz"),
            recipient: HumanAddr::from("tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz"),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn query_marker() {
        // Create a mock querier with our expected marker.
        let bin = read_test_marker_from_file();
        let expected_marker: Marker = from_binary(&bin).unwrap();
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_markers(vec![expected_marker.clone()]);
        // Query and ensure we got the expected marker
        let req = QueryMsg::GetByDenom {
            denom: "nugz".into(),
        };
        let bin = query(deps.as_ref(), mock_env(), req).unwrap();
        let marker: Marker = from_binary(&bin).unwrap();
        assert_eq!(marker, expected_marker);
        assert_eq!(marker.address, "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u")
    }
}
