use cosmwasm_std::{
    attr, to_binary, BankMsg, Binary, Context, Deps, DepsMut, Env, HandleResponse, InitResponse,
    MessageInfo, StdResult,
};
use provwasm_std::{bind_name, unbind_name, ProvenanceMsg};

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

// smart contract initialization entrypoint
pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<InitResponse<ProvenanceMsg>, ContractError> {
    // return error if no collateral received
    if info.sent_funds.is_empty() {
        return Err(ContractError::MissingCollateral {});
    }

    // return error if no ask received
    if msg.ask.is_empty() {
        return Err(ContractError::MissingAsk {});
    }

    let state = State {
        owner: info.sender.clone(),
        ask: msg.ask,
        collateral: info.sent_funds,
        name: msg.name.clone(),
    };
    config(deps.storage).save(&state)?;

    // create name binding provenance message
    let bind_name_msg = bind_name(msg.name.clone(), env.contract.address);
    Ok(InitResponse {
        messages: vec![bind_name_msg],
        attributes: vec![attr("action", "provwasm.contracts.bilateral-exchange.init")],
    })
}

// smart contract execute entrypoint
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    match msg {
        HandleMsg::Offer {} => offer(deps, env, info),
        HandleMsg::Cancel {} => cancel(deps, env, info),
    }
}

// offer action entrypoint
pub fn offer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    // check for funds sent
    if info.sent_funds.is_empty() {
        return Err(ContractError::MissingOffer {});
    }

    let state = config(deps.storage).load()?;

    // check offer sent equals ask
    if info.sent_funds != state.ask {
        return Err(ContractError::AskOfferMismatch {
            ask: state.ask,
            offer: info.sent_funds,
        });
    }

    let mut response = Context::new();

    // add 'send ask amount to owner' message
    response.add_message(BankMsg::Send {
        to_address: state.owner.clone(),
        from_address: env.contract.address.clone(),
        amount: state.ask,
    });

    // add 'send collateral to buyer' message
    response.add_message(BankMsg::Send {
        to_address: info.sender,
        from_address: env.contract.address,
        amount: state.collateral,
    });

    // add 'remove name binding' provenance message
    response.add_message(unbind_name(state.name.clone()));

    // delete the contract data
    config(deps.storage).remove();

    response.add_attribute("action", "provwasm.contracts.bilateral-exchange.offer");
    Ok(response.into())
}

// cancel action entrypoint
pub fn cancel(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<HandleResponse<ProvenanceMsg>, ContractError> {
    // return error if funds sent
    if !info.sent_funds.is_empty() {
        return Err(ContractError::CancelWithFunds {});
    }

    let state = config(deps.storage).load()?;

    if !info.sender.eq(&state.owner.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let mut response = Context::new();

    // add 'send collateral back to owner' message
    response.add_message(BankMsg::Send {
        from_address: env.contract.address,
        to_address: state.owner.clone(),
        amount: state.collateral,
    });

    // add remove name binding provenance message
    response.add_message(unbind_name(state.name.clone()));

    // delete the contract data
    config(deps.storage).remove();

    response.add_attribute("action", "provwasm.contracts.bilateral-exchange.cancel");

    Ok(response.into())
}

// smart contract query entrypoint
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryState {} => to_binary(&query_state(deps)?),
    }
}

// query state entrypoint
fn query_state(deps: Deps) -> StdResult<State> {
    let state = config_read(deps.storage).load()?;
    Ok(state.clone())
}

// unit tests
#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, BankMsg, CosmosMsg};
    use provwasm_std::{NameMsgParams, ProvenanceMsg, ProvenanceMsgParams, ProvenanceRoute};

    use super::*;

    #[test]
    fn init_with_valid_data() {
        let mut deps = mock_dependencies(&[]);
        let info = mock_info("contract_owner", &coins(1, "mark_1"));
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: coins(2, "mark_2"),
        };

        let init_response = init(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(1, init_response.messages.len());
        assert_eq!(
            CosmosMsg::Custom(ProvenanceMsg {
                route: ProvenanceRoute::Name,
                params: ProvenanceMsgParams::Name(NameMsgParams::BindName {
                    name: msg.name.to_string(),
                    address: MOCK_CONTRACT_ADDR.into(),
                    restrict: true
                }),
                version: "2.0.0".to_string(),
            }),
            init_response.messages[0]
        );
        assert_eq!(1, init_response.attributes.len());
        assert_eq!(
            attr("action", "provwasm.contracts.bilateral-exchange.init"),
            init_response.attributes[0]
        );

        let query_response = query_state(deps.as_ref()).unwrap();
        assert_eq!("contract_owner", query_response.owner.as_str());
        assert_eq!(coins(1, "mark_1"), query_response.collateral);
        assert_eq!(coins(2, "mark_2"), query_response.ask);
    }

    #[test]
    fn init_with_invalid_data() {
        let mut deps = mock_dependencies(&[]);

        // missing ask
        let info = mock_info("contract_owner", &coins(1, "mark_1"));
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: vec![],
        };

        let init_response = init(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        match init_response {
            ContractError::MissingAsk {} => (),
            error => panic!("unexpected error: {}", error),
        }

        // missing collateral
        let info = mock_info("contract_owner", &[]);
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: coins(2, "mark_2"),
        };

        let init_response = init(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();

        match init_response {
            ContractError::MissingCollateral {} => (),
            error => panic!("unexpected error: {}", error),
        }
    }

    #[test]
    fn offer_with_valid_data() {
        let mut deps = mock_dependencies(&[]);
        let collateral = coins(1, "mark_1");
        let info = mock_info("contract_owner", &collateral.clone());
        let ask = coins(2, "mark_2");
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: ask.clone(),
        };

        init(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

        let offer_response =
            offer(deps.as_mut(), mock_env(), mock_info("buyer", &ask.clone())).unwrap();

        assert_eq!(3, offer_response.messages.len());
        assert_eq!(
            CosmosMsg::Bank(BankMsg::Send {
                from_address: MOCK_CONTRACT_ADDR.into(),
                to_address: info.sender.into(),
                amount: ask.clone(),
            }),
            offer_response.messages[0]
        );
        assert_eq!(
            CosmosMsg::Bank(BankMsg::Send {
                from_address: MOCK_CONTRACT_ADDR.into(),
                to_address: "buyer".into(),
                amount: collateral.clone(),
            }),
            offer_response.messages[1]
        );
        assert_eq!(
            CosmosMsg::Custom(ProvenanceMsg {
                route: ProvenanceRoute::Name,
                params: ProvenanceMsgParams::Name(NameMsgParams::DeleteName {
                    name: msg.name.to_string(),
                }),
                version: "2.0.0".to_string(),
            }),
            offer_response.messages[2]
        );

        assert_eq!(1, offer_response.attributes.len());
        assert_eq!(
            attr("action", "provwasm.contracts.bilateral-exchange.offer"),
            offer_response.attributes[0]
        );

        // verify contract data was deleted
        let _ = query_state(deps.as_ref()).unwrap_err();
    }

    #[test]
    fn offer_with_invalid_data() {
        let mut deps = mock_dependencies(&[]);
        let collateral = coins(1, "mark_1");
        let info = mock_info("contract_owner", &collateral.clone());
        let sent_ask = coins(2, "mark_2");
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: sent_ask.clone(),
        };

        let _ = init(deps.as_mut(), mock_env(), info, msg);

        // offer without sent_funds
        let offer_response = offer(deps.as_mut(), mock_env(), mock_info("buyer", &[])).unwrap_err();

        match offer_response {
            ContractError::MissingOffer {} => (),
            error => panic!("unexpected error: {}", error),
        }

        // offer with ask/offer denom mismatch
        let sent_offer = coins(2, "mark_3");
        let offer_response =
            offer(deps.as_mut(), mock_env(), mock_info("buyer", &sent_offer)).unwrap_err();

        match offer_response {
            ContractError::AskOfferMismatch { offer, ask } => {
                assert_eq!(sent_offer, offer);
                assert_eq!(sent_ask, ask);
            }
            error => panic!("unexpected error: {}", error),
        }

        // offer with ask/offer amount mismatch
        let sent_offer = coins(1, "mark_2");
        let offer_response =
            offer(deps.as_mut(), mock_env(), mock_info("buyer", &sent_offer)).unwrap_err();

        match offer_response {
            ContractError::AskOfferMismatch { offer, ask } => {
                assert_eq!(sent_offer, offer);
                assert_eq!(sent_ask, ask);
            }
            error => panic!("unexpected error: {}", error),
        }
    }

    #[test]
    fn cancel_with_valid_data() {
        let mut deps = mock_dependencies(&[]);
        let collateral = coins(1, "mark_1");
        let info = mock_info("contract_owner", &collateral.clone());
        let ask = coins(2, "mark_2");
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: ask.clone(),
        };

        let _ = init(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        // cancel without sent_funds
        let info = mock_info("contract_owner", &[]);
        let cancel_response = cancel(deps.as_mut(), mock_env(), info.clone()).unwrap();

        assert_eq!(2, cancel_response.messages.len());
        assert_eq!(
            CosmosMsg::Bank(BankMsg::Send {
                from_address: MOCK_CONTRACT_ADDR.into(),
                to_address: info.sender.clone().into(),
                amount: collateral,
            }),
            cancel_response.messages[0]
        );
        assert_eq!(
            CosmosMsg::Custom(ProvenanceMsg {
                route: ProvenanceRoute::Name,
                params: ProvenanceMsgParams::Name(NameMsgParams::DeleteName {
                    name: msg.name.to_string(),
                }),
                version: "2.0.0".to_string(),
            }),
            cancel_response.messages[1]
        );

        assert_eq!(1, cancel_response.attributes.len());
        assert_eq!(
            attr("action", "provwasm.contracts.bilateral-exchange.cancel"),
            cancel_response.attributes[0]
        );

        // verify contract data was deleted
        let _ = query_state(deps.as_ref()).unwrap_err();
    }

    #[test]
    fn cancel_with_invalid_data() {
        let mut deps = mock_dependencies(&[]);
        let collateral = coins(1, "mark_1");
        let info = mock_info("contract_owner", &collateral.clone());
        let ask = coins(2, "mark_2");
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: ask.clone(),
        };

        let _ = init(deps.as_mut(), mock_env(), info.clone(), msg);

        // cancel with sent_funds
        let cancel_response = cancel(deps.as_mut(), mock_env(), info.clone()).unwrap_err();
        match cancel_response {
            ContractError::CancelWithFunds {} => (),
            error => panic!("unexpected error: {}", error),
        }

        // cancel with sender not equal to owner
        let info = mock_info("not_the_owner", &[]);
        let cancel_response = cancel(deps.as_mut(), mock_env(), info.clone()).unwrap_err();
        match cancel_response {
            ContractError::Unauthorized {} => (),
            error => panic!("unexpected error: {}", error),
        }
    }

    #[test]
    fn query_state_with_valid_init() {
        let mut deps = mock_dependencies(&[]);
        let collateral = coins(1, "mark_1");
        let info = mock_info("contract_owner", &collateral.clone());
        let ask = coins(2, "mark_2");
        let msg = InitMsg {
            name: "contract_name".to_string(),
            ask: ask.clone(),
        };

        let _ = init(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        let query_response = query_state(deps.as_ref()).unwrap();
        assert_eq!(info.sender, query_response.owner);
        assert_eq!(info.sent_funds, query_response.collateral);
        assert_eq!(ask, query_response.ask);
        assert_eq!(msg.name, query_response.name);
    }
}
