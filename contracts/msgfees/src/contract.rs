use cosmwasm_std::{entry_point, Addr, Coin, DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::cosmos::bank::v1beta1::MsgSend;

use crate::error::ContractError;
use crate::helpers::assess_custom_fee;
use crate::msg::{ExecuteMsg, InitMsg};
use crate::state::{config, config_read, State};

/// Initialize the smart contract config state and bind a name to the contract address.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    _: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // Create contract config state.
    let state = State {
        fee_amount: msg.fee_amount,
        fee_recipient: msg.fee_recipient,
    };

    // Save contract config state.
    config(deps.storage).save(&state)?;

    // Dispatch bind name message and add event attributes.
    let res = Response::new()
        .add_attribute("integration_test", "msgfees")
        .add_attribute("action", "provwasm.contracts.msgfees.init")
        .add_attribute("fee_amount", format!("{:?}", state.fee_amount))
        .add_attribute("fee_recipient", format!("{:?}", state.fee_recipient));
    Ok(res)
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
        ExecuteMsg::SendFunds { funds, to_address } => {
            try_send_funds(deps, env, info, funds, to_address)
        }
    }
}

// Send funds to te recipient and assess a fee
pub fn try_send_funds(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    funds: Coin,
    to_address: Addr,
) -> Result<Response, ContractError> {
    // Load contract state
    let state = config_read(deps.storage).load()?;

    // Dispatch message to handler and emit events
    let mut res = Response::new()
        .add_attribute("integration_test", "msgfees")
        .add_attribute("action", "provwasm.contracts.msgfees.try_send_funds");

    if let Some(fee) = state.fee_amount {
        // Create a message that will assess a custom fee
        res = res.add_message(assess_custom_fee(
            fee.to_owned(),
            Some("std_contract_fee"),
            env.contract.address.clone(),
            state.fee_recipient.to_owned(),
        )?);
        res = res
            .add_attribute("fee_recipient", format!("{:?}", &state.fee_recipient))
            .add_attribute("fee_amount", format!("{:?}", &fee));
    }

    // Create a message that will send funds to the to_address.
    let send_funds = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: to_address.to_string(),
        amount: vec![provwasm_std::types::cosmos::base::v1beta1::Coin {
            denom: funds.clone().denom,
            amount: funds.amount.to_string(),
        }],
    };

    // Dispatch message to handler and emit events
    res = res
        .add_message(send_funds)
        .add_attribute("funds", format!("{:?}", funds))
        .add_attribute("to_address", format!("{:?}", to_address));
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{attr, coin, Binary, CosmosMsg};
    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::types::cosmos::base::v1beta1::Coin;
    use provwasm_std::types::provenance::msgfees::v1::MsgAssessCustomMsgFeeRequest;

    #[test]
    fn init_valid() {
        // Create default provenance mocks.
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        // Init with fees
        let msg = InitMsg {
            fee_amount: Some(coin(100_000, "nhash")),
            fee_recipient: Some(Addr::unchecked("fee_address")),
        };

        // Ensure contract init with fees
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert!(res.messages.is_empty());
        assert_eq!(res.attributes.len(), 4);
        assert_eq!(
            res.attributes,
            vec![
                attr("integration_test", "msgfees"),
                attr("action", "provwasm.contracts.msgfees.init"),
                attr("fee_amount", format!("{:?}", Some(coin(100_000, "nhash")))),
                attr(
                    "fee_recipient",
                    format!("{:?}", Some(Addr::unchecked("fee_address")))
                ),
            ]
        );
        assert_eq!(
            config_read(&deps.storage).load().unwrap(),
            State {
                fee_amount: Some(coin(100_000, "nhash")),
                fee_recipient: Some(Addr::unchecked("fee_address")),
            }
        )
    }

    #[test]
    fn send_funds_with_fees() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[coin(200_000, "nhash")]);

        config(&mut deps.storage)
            .save(&State {
                fee_amount: Some(coin(100_000, "nhash")),
                fee_recipient: Some(Addr::unchecked("fee_address")),
            })
            .expect("failed to save test state");

        let msg = ExecuteMsg::SendFunds {
            funds: coin(100_000, "nhash"),
            to_address: Addr::unchecked("to_address"),
        };
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(2, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                let expected: Binary = MsgAssessCustomMsgFeeRequest {
                    name: "std_contract_fee".to_string(),
                    amount: Some(Coin {
                        denom: "nhash".to_string(),
                        amount: "100000".to_string(),
                    }),
                    recipient: "fee_address".to_string(),
                    from: MOCK_CONTRACT_ADDR.to_string(),
                    recipient_basis_points: "10000".to_string(),
                }
                .try_into()
                .unwrap();

                assert_eq!(
                    type_url,
                    "/provenance.msgfees.v1.MsgAssessCustomMsgFeeRequest"
                );
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }

        match &res.messages[1].msg {
            CosmosMsg::Stargate { type_url, value } => {
                let expected: Binary = MsgSend {
                    from_address: MOCK_CONTRACT_ADDR.to_string(),
                    to_address: "to_address".to_string(),
                    amount: vec![provwasm_std::types::cosmos::base::v1beta1::Coin {
                        denom: "nhash".to_string(),
                        amount: "100000".to_string(),
                    }],
                }
                .try_into()
                .unwrap();

                assert_eq!(type_url, "/cosmos.bank.v1beta1.MsgSend");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn send_funds_without_fees() {
        // Init state
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        config(&mut deps.storage)
            .save(&State {
                fee_amount: None,
                fee_recipient: None,
            })
            .expect("failed to save test state");

        let msg = ExecuteMsg::SendFunds {
            funds: coin(100_000, "nhash"),
            to_address: Addr::unchecked("to_address"),
        };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        // Assert the correct message was created
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Stargate { type_url, value } => {
                let expected: Binary = MsgSend {
                    from_address: MOCK_CONTRACT_ADDR.to_string(),
                    to_address: "to_address".to_string(),
                    amount: vec![provwasm_std::types::cosmos::base::v1beta1::Coin {
                        denom: "nhash".to_string(),
                        amount: "100000".to_string(),
                    }],
                }
                .try_into()
                .unwrap();

                assert_eq!(type_url, "/cosmos.bank.v1beta1.MsgSend");
                assert_eq!(value, &expected)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }
}
