use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdError, StdResult, Uint128,
};

use provwasm_std::types::provenance::marker::v1::{MarkerQuerier, MarkerType};

use crate::error::ContractError;
use crate::helpers::{
    activate_marker, all_access, bind_name, burn_marker_supply, cancel_marker, create_marker,
    destroy_marker, finalize_marker, get_marker_by_address, get_marker_by_denom,
    grant_marker_access, mint_marker_supply, transfer_marker_coins, withdraw_coins,
};
use crate::msg::{ExecuteMsg, InitMsg, QueryMsg};
use crate::state::{config, State};

/// Initialize the smart contract config state, then bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create and save state
    config(deps.storage).save(&State {
        contract_name: msg.name.clone(),
    })?;

    // Create a name for the contract
    let bind_name_msg = bind_name(
        &msg.name,
        &env.contract.address,
        &env.contract.address,
        true,
    )?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new()
        .add_message(bind_name_msg)
        .add_attributes(vec![
            attr("action", "provwasm.contracts.marker.init"),
            attr("integration_test", "v2"),
            attr("contract_name", msg.name),
        ]))
}

/// Handle messages that create and interact with with native provenance markers.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Create {
            supply,
            denom,
            allow_forced_transfer,
        } => try_create(supply, denom, allow_forced_transfer, env),
        ExecuteMsg::GrantAccess { denom } => try_grant_access(denom, env.contract.address),
        ExecuteMsg::Finalize { denom } => try_finalize(denom, env.contract.address),
        ExecuteMsg::Activate { denom } => try_activate(denom, env.contract.address),
        ExecuteMsg::Mint { amount, denom } => try_mint(amount, denom, env.contract.address),
        ExecuteMsg::Burn { amount, denom } => try_burn(amount, denom, env.contract.address),
        ExecuteMsg::Cancel { denom } => try_cancel(denom, env.contract.address),
        ExecuteMsg::Destroy { denom } => try_destroy(denom, env.contract.address),
        ExecuteMsg::Withdraw { amount, denom } => try_withdraw(
            amount,
            denom,
            env.contract.address.clone(),
            env.contract.address,
        ),
        ExecuteMsg::Transfer { amount, denom, to } => {
            let to = deps.api.addr_validate(&to)?;
            try_transfer(
                amount,
                denom,
                to,
                env.contract.address.clone(),
                env.contract.address,
            )
        }
    }
}

// Create and dispatch a message that will create a new restricted marker w/ proposed status.
fn try_create(
    supply: Uint128,
    denom: String,
    allow_forced_transfer: bool,
    env: Env,
) -> StdResult<Response> {
    let msg = create_marker(
        supply.u128(),
        &denom,
        MarkerType::Restricted,
        env.contract.address,
        allow_forced_transfer,
    )?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.create")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_supply", supply)
        .add_attribute("marker_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will grant all permissions to a marker for an address.
fn try_grant_access(denom: String, address: Addr) -> StdResult<Response> {
    let msg = grant_marker_access(&denom, address.clone(), all_access(&address))?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.grant_access")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom)
        .add_attribute("marker_addr", address);
    Ok(res)
}

// Create and dispatch a message that will finalize a proposed marker.
fn try_finalize(denom: String, address: Addr) -> StdResult<Response> {
    let msg = finalize_marker(&denom, address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.finalize")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will activate a finalized marker.
fn try_activate(denom: String, address: Addr) -> StdResult<Response> {
    let msg = activate_marker(&denom, address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.activate")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will withdraw coins from a marker.
fn try_withdraw(
    amount: Uint128,
    denom: String,
    recipient: Addr,
    contract_address: Addr,
) -> StdResult<Response> {
    let marker_denom = denom.clone();
    let msg = withdraw_coins(
        &marker_denom,
        amount.u128(),
        &denom,
        recipient.clone(),
        contract_address,
    )?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.withdraw")
        .add_attribute("integration_test", "v2")
        .add_attribute("withdraw_amount", amount)
        .add_attribute("withdraw_denom", denom)
        .add_attribute("withdraw_recipient", recipient);
    Ok(res)
}

// Create and dispatch a message that will mint coins into a marker.
fn try_mint(amount: Uint128, denom: String, contract_address: Addr) -> StdResult<Response> {
    let msg = mint_marker_supply(amount.u128(), &denom, contract_address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.mint")
        .add_attribute("integration_test", "v2")
        .add_attribute("mint_amount", amount)
        .add_attribute("mint_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will burn coins from a marker.
fn try_burn(amount: Uint128, denom: String, contract_address: Addr) -> StdResult<Response> {
    let msg = burn_marker_supply(amount.u128(), &denom, contract_address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.burn")
        .add_attribute("integration_test", "v2")
        .add_attribute("mint_amount", amount)
        .add_attribute("mint_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will cancel a marker.
fn try_cancel(denom: String, contract_address: Addr) -> StdResult<Response> {
    let msg = cancel_marker(&denom, contract_address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.cancel")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will destroy a marker.
fn try_destroy(denom: String, contract_address: Addr) -> StdResult<Response> {
    let msg = destroy_marker(denom.clone(), contract_address)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.destroy")
        .add_attribute("integration_test", "v2")
        .add_attribute("marker_denom", denom);
    Ok(res)
}

// Create and dispatch a message that will transfer coins from one account to another.
fn try_transfer(
    amount: Uint128,
    denom: String,
    to: Addr,
    from: Addr,
    contract_address: Addr,
) -> StdResult<Response> {
    let msg = transfer_marker_coins(
        amount.u128(),
        &denom,
        to.clone(),
        from.clone(),
        contract_address,
    )?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.marker.transfer")
        .add_attribute("integration_test", "v2")
        .add_attribute("funds", format!("{}{}", &amount, &denom))
        .add_attribute("to", to)
        .add_attribute("from", from);
    Ok(res)
}

/// Handle query requests for the provenance marker module.
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    match msg {
        QueryMsg::GetByAddress { address } => try_get_marker_by_address(deps, address),
        QueryMsg::GetByDenom { denom } => try_get_marker_by_denom(deps, denom),
    }
}

// Query a marker by address.
fn try_get_marker_by_address(deps: Deps, address: String) -> Result<QueryResponse, StdError> {
    let address = deps.api.addr_validate(&address)?;
    let querier = MarkerQuerier::new(&deps.querier);
    let marker = get_marker_by_address(address, &querier)?;
    to_binary(&marker)
}

// Query a marker by denom.
fn try_get_marker_by_denom(deps: Deps, denom: String) -> Result<QueryResponse, StdError> {
    let querier = MarkerQuerier::new(&deps.querier);
    let marker = get_marker_by_denom(denom, &querier)?;
    to_binary(&marker)
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_env, mock_info};
//     use cosmwasm_std::{coin, from_binary, CosmosMsg};
//     use provwasm_mocks::{mock_dependencies, must_read_binary_file};
//     use provwasm_std::{Marker, MarkerMsgParams, ProvenanceMsgParams};
//
//     // A helper function that will extract marker message params from a custom cosmos message.
//     fn unwrap_marker_params(msg: &CosmosMsg<ProvenanceMsg>) -> &MarkerMsgParams {
//         match &msg {
//             CosmosMsg::Custom(msg) => match &msg.params {
//                 ProvenanceMsgParams::Marker(mp) => mp,
//                 _ => panic!("unexpected provenance params"),
//             },
//             _ => panic!("unexpected cosmos message"),
//         }
//     }
//
//     #[test]
//     fn valid_init() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         // Call init
//         let res = instantiate(
//             deps.as_mut(),
//             mock_env(),
//             mock_info("sender", &[]),
//             InitMsg {
//                 name: "contract.pb".into(),
//             },
//         )
//         .unwrap();
//         // Ensure a message was created to bind the name to the contract address.
//         assert_eq!(1, res.messages.len());
//     }
//
//     #[test]
//     fn create_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Expected marker supply
//         let expected_coin = coin(420, "budz");
//
//         // Create marker execute message
//         let msg = ExecuteMsg::Create {
//             supply: Uint128::new(420),
//             denom: "budz".into(),
//             allow_forced_transfer: false,
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::CreateMarker {
//                 coin,
//                 marker_type,
//                 allow_forced_transfer,
//             } => {
//                 assert_eq!(*coin, expected_coin);
//                 assert_eq!(*marker_type, MarkerType::Restricted);
//                 assert!(!*allow_forced_transfer);
//             }
//             _ => panic!("expected marker create params"),
//         }
//     }
//
//     #[test]
//     fn create_forced_transfer_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Expected marker supply
//         let expected_coin = coin(420, "budz");
//
//         // Create marker execute message
//         let msg = ExecuteMsg::Create {
//             supply: Uint128::new(420),
//             denom: "budz".into(),
//             allow_forced_transfer: true,
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::CreateMarker {
//                 coin,
//                 marker_type,
//                 allow_forced_transfer,
//             } => {
//                 assert_eq!(*coin, expected_coin);
//                 assert_eq!(*marker_type, MarkerType::Restricted);
//                 assert!(*allow_forced_transfer);
//             }
//             _ => panic!("expected marker create params"),
//         }
//     }
//
//     #[test]
//     fn grant_access() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create expected access permissions (all)
//         let expected_permissions = MarkerAccess::all();
//
//         // Create an access grant execute message
//         let msg = ExecuteMsg::GrantAccess {
//             denom: "budz".into(),
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::GrantMarkerAccess {
//                 denom,
//                 address,
//                 permissions,
//             } => {
//                 assert_eq!(denom, "budz");
//                 assert_eq!(address, &env.contract.address);
//                 assert_eq!(*permissions, expected_permissions);
//             }
//             _ => panic!("expected marker grant params"),
//         }
//     }
//
//     #[test]
//     fn finalize_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create a finalize marker execute message
//         let msg = ExecuteMsg::Finalize {
//             denom: "budz".into(),
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::FinalizeMarker { denom } => {
//                 assert_eq!(denom, "budz");
//             }
//             _ => panic!("expected marker finalize params"),
//         }
//     }
//
//     #[test]
//     fn activate_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create an activate marker execute message
//         let msg = ExecuteMsg::Activate {
//             denom: "budz".into(),
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::ActivateMarker { denom } => {
//                 assert_eq!(denom, "budz");
//             }
//             _ => panic!("expected marker activate params"),
//         }
//     }
//
//     #[test]
//     fn withdraw_coins() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Expected withdraw amount
//         let expected_coin = coin(20, "budz");
//
//         // Create a withdraw execute message
//         let msg = ExecuteMsg::Withdraw {
//             amount: Uint128::new(20),
//             denom: "budz".into(),
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::WithdrawCoins {
//                 marker_denom,
//                 coin,
//                 recipient,
//             } => {
//                 assert_eq!(marker_denom, "budz");
//                 assert_eq!(*coin, expected_coin);
//                 assert_eq!(recipient, &env.contract.address);
//             }
//             _ => panic!("expected marker withdraw params"),
//         }
//     }
//
//     #[test]
//     fn mint_coins() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Expect to mint this amount
//         let expected_coin = coin(20, "budz");
//
//         // Create a mint coins marker handler message
//         let msg = ExecuteMsg::Mint {
//             amount: Uint128::new(20),
//             denom: "budz".into(),
//         };
//
//         // Call handle and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::MintMarkerSupply { coin } => assert_eq!(*coin, expected_coin),
//             _ => panic!("expected marker mint params"),
//         }
//     }
//
//     #[test]
//     fn burn_coins() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Expect to burn this amount
//         let expected_coin = coin(20, "budz");
//
//         // Create a burn coins marker handler message
//         let msg = ExecuteMsg::Burn {
//             amount: Uint128::new(20),
//             denom: "budz".into(),
//         };
//
//         // Call handle and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::BurnMarkerSupply { coin } => assert_eq!(*coin, expected_coin),
//             _ => panic!("expected marker burn params"),
//         }
//     }
//
//     #[test]
//     fn cancel_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create a cancel marker handler message
//         let msg = ExecuteMsg::Cancel {
//             denom: "budz".into(),
//         };
//
//         // Call handle and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::CancelMarker { denom } => assert_eq!(denom, "budz"),
//             _ => panic!("expected marker cancel params"),
//         }
//     }
//
//     #[test]
//     fn destroy_marker() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create a destroy marker handler message
//         let msg = ExecuteMsg::Destroy {
//             denom: "budz".into(),
//         };
//
//         // Call handle and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), env, info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::DestroyMarker { denom } => assert_eq!(denom, "budz"),
//             _ => panic!("expected marker destroy params"),
//         }
//     }
//
//     #[test]
//     fn transfer_coins() {
//         // Create default provenance mocks.
//         let mut deps = mock_dependencies(&[]);
//         let env = mock_env();
//         let info = mock_info("sender", &[]);
//
//         // Create a transfer execute message
//         let msg = ExecuteMsg::Transfer {
//             amount: Uint128::new(20),
//             denom: "budz".into(),
//             to: "toaddress".into(),
//         };
//
//         // Call execute and ensure a cosmos message was dispatched
//         let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(1, res.messages.len());
//
//         // Assert the correct params were created
//         let expected_coin = coin(20, "budz");
//         match unwrap_marker_params(&res.messages[0].msg) {
//             MarkerMsgParams::TransferMarkerCoins { coin, to, from } => {
//                 assert_eq!(*coin, expected_coin);
//                 assert_eq!(*to, Addr::unchecked("toaddress"));
//                 assert_eq!(from, &env.contract.address);
//             }
//             _ => panic!("expected marker transfer params"),
//         }
//     }
//
//     #[test]
//     fn query_marker() {
//         // Create a mock querier with our expected marker.
//         let bin = must_read_binary_file("testdata/marker.json");
//         let expected_marker: Marker = from_binary(&bin).unwrap();
//         let mut deps = mock_dependencies(&[]);
//         deps.querier.with_markers(vec![expected_marker.clone()]);
//         // Query and ensure we got the expected marker
//         let req = QueryMsg::GetByDenom {
//             denom: "nugz".into(),
//         };
//         let bin = query(deps.as_ref(), mock_env(), req).unwrap();
//         let marker: Marker = from_binary(&bin).unwrap();
//         assert_eq!(marker, expected_marker);
//         assert_eq!(marker.address, "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u")
//     }
// }
