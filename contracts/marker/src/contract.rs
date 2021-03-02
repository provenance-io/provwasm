use cosmwasm_std::{
    attr, to_binary, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse, MessageInfo,
    QueryResponse, StdError, Uint128,
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
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    // Create a name for the contract
    let bind_name_msg = bind_name(&msg.name, env.contract.address);

    // Dispatch messages to the name module handler and emit an event.
    Ok(InitResponse {
        messages: vec![bind_name_msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.init"),
            attr("integration_test", "v2"),
            attr("contract_name", msg.name),
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
        HandleMsg::CreateMarker { supply, denom } => try_create_marker(supply, denom),
        HandleMsg::GrantAccess { denom } => try_grant_marker_access(denom, env.contract.address),
        HandleMsg::Finalize { denom } => try_finalize_marker(denom),
        HandleMsg::Activate { denom } => try_activate_marker(denom),
        HandleMsg::Withdraw { amount, denom } => {
            try_withdraw_marker_coins(amount, denom, env.contract.address)
        }
    };
    Ok(res)
}

// Create and dispatch a message that will create a new proposed marker.
fn try_create_marker(supply: Uint128, denom: String) -> HandleResponse<ProvenanceMsg> {
    let msg = create_marker(supply.u128(), &denom);
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.create"),
            attr("integration_test", "v2"),
            attr("marker_supply", supply),
            attr("marker_denom", denom),
        ],
        data: None,
    }
}

// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_marker_access(denom: String, address: HumanAddr) -> HandleResponse<ProvenanceMsg> {
    let msg = grant_marker_access_all(&denom, &address);
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
    let msg = finalize_marker(&denom);
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
    let msg = activate_marker(&denom);
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
fn try_withdraw_marker_coins(
    amount: Uint128,
    denom: String,
    recipient: HumanAddr,
) -> HandleResponse<ProvenanceMsg> {
    let msg = withdraw_marker_coins(amount.u128(), &denom, &recipient);
    HandleResponse {
        messages: vec![msg],
        attributes: vec![
            attr("action", "provwasm.contracts.marker.withdraw"),
            attr("integration_test", "v2"),
            attr("withdraw_amount", amount),
            attr("withdraw_denom", denom),
            attr("withdraw_recipient", recipient),
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
    let marker = querier.get_marker_by_denom(&denom)?;
    to_binary(&marker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use provwasm_mocks::{mock_dependencies, must_read_binary_file};
    use provwasm_std::Marker;

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
            supply: Uint128(420),
            denom: "budz".into(),
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
            amount: Uint128(20),
            denom: "budz".into(),
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
        assert_eq!(marker.address, "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u")
    }
}
