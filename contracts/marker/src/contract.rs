use cosmwasm_std::{
    attr, to_binary, Coin, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse,
    MessageInfo, QueryResponse, StdError,
};
use provwasm_std::{
    activate_marker, bind_name, create_marker, finalize_marker, grant_marker_access_all,
    send_coins, withdraw_marker_coins, ProvenanceMsg, ProvenanceQuerier,
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
    env: Env,
    _info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    let res = match msg {
        HandleMsg::CreateMarker { coin } => try_create_marker(coin),
        HandleMsg::GrantAccess { denom } => try_grant_marker_access(denom, env.contract.address),
        HandleMsg::Finalize { denom } => try_finalize_marker(denom),
        HandleMsg::Activate { denom } => try_activate_marker(denom),
        HandleMsg::Withdraw { coin } => try_withdraw_marker_coins(coin, env.contract.address),
        HandleMsg::Send { coin, to } => try_send_coins(coin, to, env.contract.address),
    };
    Ok(res)
}

// Create and dispatch a message that will create a new proposed marker.
fn try_create_marker(coin: Coin) -> HandleResponse<ProvenanceMsg> {
    let msg = create_marker(coin.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.create"),
            attr("integration_test", "v2"),
            attr("marker_denom", coin.denom),
            attr("marker_supply", coin.amount),
        ],
        data: None,
    }
}

// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_marker_access(denom: String, address: HumanAddr) -> HandleResponse<ProvenanceMsg> {
    let msg = grant_marker_access_all(denom.clone(), address.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.grant_access"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
            attr("marker_addr", address),
        ],
        data: None,
    }
}

// Create and dispatch a message that will finalize a proposed marker.
fn try_finalize_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
    let msg = finalize_marker(denom.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.finalize"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
        ],
        data: None,
    }
}

// Create and dispatch a message that will activate a finalized marker.
fn try_activate_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
    let msg = activate_marker(denom.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.activate"),
            attr("integration_test", "v2"),
            attr("marker_denom", denom),
        ],
        data: None,
    }
}

// Create and dispatch a message that will withdraw coins from a marker.
fn try_withdraw_marker_coins(coin: Coin, recipient: HumanAddr) -> HandleResponse<ProvenanceMsg> {
    let msg = withdraw_marker_coins(coin.clone(), recipient.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.withdraw"),
            attr("integration_test", "v2"),
            attr("marker_denom", coin.denom),
            attr("marker_amount", coin.amount),
            attr("marker_recipient", recipient),
        ],
        data: None,
    }
}

// Create and dispatch a message that send coins to a recipient.
fn try_send_coins(coin: Coin, to: HumanAddr, from: HumanAddr) -> HandleResponse<ProvenanceMsg> {
    let msg = send_coins(vec![coin.clone()], to.clone(), from.clone());
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.send"),
            attr("integration_test", "v2"),
            attr("send_denom", coin.denom),
            attr("send_amount", coin.amount),
            attr("send_to", to),
            attr("send_from", from),
        ],
        data: None,
    }
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
    use cosmwasm_std::{coin, from_binary};
    use provwasm_mocks::{mock_dependencies, must_read_binary_file};
    use provwasm_std::Marker;

    // TODO: Add assertions for the expected message params dispatched in each test

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
        // Create test handler message
        let msg = HandleMsg::GrantAccess {
            denom: "budz".into(),
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
        // Create test handler message
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
        // Create test handler message
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
        // Create test handler message
        let msg = HandleMsg::Withdraw {
            coin: coin(20, "budz"),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn send_coins() {
        // Create default provenance mocks.
        let balance = coin(100, "budz");
        let mut deps = mock_dependencies(&[balance]);
        let env = mock_env();
        let info = mock_info("sender", &[]);
        // Create test handler message
        let msg = HandleMsg::Send {
            coin: coin(20, "budz"),
            to: HumanAddr::from("recipient"),
        };
        // Ensure message was created
        let res = handle(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());
    }

    #[test]
    fn query_marker() {
        // Create a mock querier with our expected marker.
        let bin = must_read_binary_file("testdata/marker.json");
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
    }
}
