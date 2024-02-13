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
use crate::state::{State, CONFIG};

/// Initialize the smart contract config state, then bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create and save state
    CONFIG.save(
        deps.storage,
        &State {
            contract_name: msg.name.clone(),
        },
    )?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Marker;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{from_binary, Binary, CosmosMsg};
    use prost::Message;
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::shim::Any;
    use provwasm_std::types::cosmos::auth::v1beta1::BaseAccount;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::marker::v1::{
        AccessGrant, MarkerAccount, MarkerStatus, MsgActivateRequest, MsgAddAccessRequest,
        MsgAddMarkerRequest, MsgBurnRequest, MsgCancelRequest, MsgDeleteRequest,
        MsgFinalizeRequest, MsgMintRequest, MsgTransferRequest, MsgWithdrawRequest,
        QueryEscrowRequest, QueryEscrowResponse, QueryMarkerRequest, QueryMarkerResponse,
    };
    use std::convert::TryInto;

    #[test]
    fn valid_init() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        // Call init
        let res = instantiate(
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
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let contract_address = env.contract.address.to_string();

        // Expected marker supply
        let expected_msg: Binary = MsgAddMarkerRequest {
            amount: Some(Coin {
                denom: "budz".to_string(),
                amount: "420".to_string(),
            }),
            manager: contract_address.clone(),
            from_address: contract_address,
            status: MarkerStatus::Proposed.into(),
            marker_type: MarkerType::Restricted.into(),
            access_list: vec![],
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: false,
            required_attributes: vec![],
            usd_cents: 0,
            volume: 0,
        }
        .try_into()
        .unwrap();

        // Create marker execute message
        let msg = ExecuteMsg::Create {
            supply: Uint128::new(420),
            denom: "budz".into(),
            allow_forced_transfer: false,
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created

        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgAddMarkerRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn create_forced_transfer_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let contract_address = env.contract.address.to_string();

        // Expected marker supply
        let expected_msg: Binary = MsgAddMarkerRequest {
            amount: Some(Coin {
                denom: "budz".to_string(),
                amount: "420".to_string(),
            }),
            manager: contract_address.clone(),
            from_address: contract_address,
            status: MarkerStatus::Proposed.into(),
            marker_type: MarkerType::Restricted.into(),
            access_list: vec![],
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: true,
            required_attributes: vec![],
            usd_cents: 0,
            volume: 0,
        }
        .try_into()
        .unwrap();

        // Create marker execute message
        let msg = ExecuteMsg::Create {
            supply: Uint128::new(420),
            denom: "budz".into(),
            allow_forced_transfer: true,
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created

        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgAddMarkerRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn grant_access() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let contract_address = env.contract.address.to_string();

        let expected_msg: Binary = MsgAddAccessRequest {
            denom: "budz".into(),
            administrator: contract_address,
            access: all_access(&env.contract.address),
        }
        .try_into()
        .unwrap();

        // Create an access grant execute message
        let msg = ExecuteMsg::GrantAccess {
            denom: "budz".into(),
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgAddAccessRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn finalize_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgFinalizeRequest {
            denom: "budz".to_string(),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create a finalize marker execute message
        let msg = ExecuteMsg::Finalize {
            denom: "budz".into(),
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgFinalizeRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn activate_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgActivateRequest {
            denom: "budz".to_string(),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create an activate marker execute message
        let msg = ExecuteMsg::Activate {
            denom: "budz".into(),
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgActivateRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn withdraw_coins() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);
        let contract_address = env.contract.address.to_string();

        let expected_msg: Binary = MsgWithdrawRequest {
            denom: "budz".to_string(),
            administrator: contract_address.clone(),
            to_address: contract_address,
            amount: vec![Coin {
                denom: "budz".to_string(),
                amount: "20".to_string(),
            }],
        }
        .try_into()
        .unwrap();

        // Create a withdraw execute message
        let msg = ExecuteMsg::Withdraw {
            amount: Uint128::new(20),
            denom: "budz".into(),
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgWithdrawRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn mint_coins() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgMintRequest {
            amount: Some(Coin {
                denom: "budz".to_string(),
                amount: "20".to_string(),
            }),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create a mint coins marker handler message
        let msg = ExecuteMsg::Mint {
            amount: Uint128::new(20),
            denom: "budz".into(),
        };

        // Call handle and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgMintRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn burn_coins() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgBurnRequest {
            amount: Some(Coin {
                denom: "budz".to_string(),
                amount: "20".to_string(),
            }),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create a burn coins marker handler message
        let msg = ExecuteMsg::Burn {
            amount: Uint128::new(20),
            denom: "budz".into(),
        };

        // Call handle and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgBurnRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn cancel_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgCancelRequest {
            denom: "budz".to_string(),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create a cancel marker handler message
        let msg = ExecuteMsg::Cancel {
            denom: "budz".into(),
        };

        // Call handle and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgCancelRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn destroy_marker() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgDeleteRequest {
            denom: "budz".to_string(),
            administrator: env.contract.address.to_string(),
        }
        .try_into()
        .unwrap();

        // Create a destroy marker handler message
        let msg = ExecuteMsg::Destroy {
            denom: "budz".into(),
        };

        // Call handle and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgDeleteRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn transfer_coins() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let expected_msg: Binary = MsgTransferRequest {
            amount: Some(Coin {
                denom: "budz".to_string(),
                amount: "20".to_string(),
            }),
            administrator: env.contract.address.to_string(),
            from_address: env.contract.address.to_string(),
            to_address: "toaddress".to_string(),
        }
        .try_into()
        .unwrap();

        // Create a transfer execute message
        let msg = ExecuteMsg::Transfer {
            amount: Uint128::new(20),
            denom: "budz".into(),
            to: "toaddress".into(),
        };

        // Call execute and ensure a cosmos message was dispatched
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Assert the correct params were created
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                assert_eq!(type_url, "/provenance.marker.v1.MsgTransferRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn query_marker() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();

        // Create a mock querier with our expected marker.
        let expected_marker = MarkerAccount {
            base_account: Some(BaseAccount {
                address: "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u".to_string(),
                pub_key: None,
                account_number: 10,
                sequence: 0,
            }),
            manager: env.contract.address.to_string(),
            access_control: vec![AccessGrant {
                address: "tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz".to_string(),
                permissions: vec![1, 2, 3, 4, 5, 6, 7],
            }],
            status: MarkerStatus::Active.into(),
            denom: "nugz".to_string(),
            supply: "420".to_string(),
            marker_type: 0,
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: false,
            required_attributes: vec![],
        };

        let mock_marker_response = QueryMarkerResponse {
            marker: Some(Any {
                type_url: "/provenance.marker.v1.MarkerAccount".to_string(),
                value: expected_marker.encode_to_vec(),
            }),
        };

        QueryMarkerRequest::mock_response(&mut deps.querier, mock_marker_response);

        QueryEscrowRequest::mock_response(
            &mut deps.querier,
            QueryEscrowResponse {
                escrow: vec![Coin {
                    denom: "budz".to_string(),
                    amount: "20".to_string(),
                }],
            },
        );

        // Query and ensure we got the expected marker
        let req = QueryMsg::GetByDenom {
            denom: "nugz".into(),
        };

        let bin = query(deps.as_ref(), mock_env(), req).unwrap();

        let marker: Marker = from_binary(&bin).unwrap();
        assert_eq!(marker.marker_account, expected_marker);
        assert_eq!(
            marker.marker_account.base_account.unwrap().address,
            "tp18vmzryrvwaeykmdtu6cfrz5sau3dhc5c73ms0u"
        )
    }
}
