use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, Uint64,
};

use provwasm_std::shim::{Any, Timestamp};
use provwasm_std::types::cosmos::bank::v1beta1::MsgSend;
use provwasm_std::types::provenance::trigger::v1::{
    BlockHeightEvent, BlockTimeEvent, MsgCreateTriggerRequest, MsgDestroyTriggerRequest,
    TriggerQuerier,
};

use crate::error::ContractError;
use crate::msg::{Event, ExecuteMsg, InitMsg, QueryMsg};

/// Initialize the smart contract
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<Response, ContractError> {
    let res = Response::new().add_attribute("action", "provwasm.contracts.trigger.init");
    Ok(res)
}

/// Handle messages that create or delete triggers.
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateTrigger { event, to_address } => {
            create_trigger(deps, env, info, event, to_address)
        }
        ExecuteMsg::DeleteTrigger { id } => delete_trigger(deps, env, info, id),
    }
}

// create a trigger that executes at a later time
pub fn create_trigger(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    event: Event,
    to_address: String,
) -> Result<Response, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::FundsEmpty);
    }

    let trigger_event: Any = match event {
        Event::BlockHeightEvent { block_height } => BlockHeightEvent {
            block_height: block_height.into(),
        }
        .to_any(),
        Event::BlockTimeEvent { timestamp } => BlockTimeEvent {
            time: Some(Timestamp {
                seconds: timestamp.u64() as i64,
                nanos: 0,
            }),
        }
        .to_any(),
    };

    let msg = MsgCreateTriggerRequest {
        authorities: vec![env.contract.address.to_string()],
        event: Some(trigger_event),
        actions: vec![MsgSend {
            from_address: env.contract.address.to_string(),
            to_address,
            amount: info.funds.into_iter().map(|fund| fund.into()).collect(),
        }
        .to_any()],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.trigger.create_trigger"))
}

// delete an existing trigger
pub fn delete_trigger(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    id: Uint64,
) -> Result<Response, ContractError> {
    let msg = MsgDestroyTriggerRequest {
        id: id.into(),
        authority: env.contract.address.to_string(),
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.trigger.delete_trigger"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    match msg {
        QueryMsg::GetTrigger { id } => get_trigger(deps, id),
    }
}

pub fn get_trigger(deps: Deps, id: Option<Uint64>) -> Result<QueryResponse, ContractError> {
    let trigger_querier = TriggerQuerier::new(&deps.querier);

    match id {
        Some(id) => Ok(to_json_binary(&trigger_querier.trigger_by_id(id.u64())?)?),
        None => Ok(to_json_binary(&trigger_querier.triggers(None)?)?),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{message_info, mock_env};
    use cosmwasm_std::{coin, Addr, AnyMsg, Binary, CosmosMsg};

    use provwasm_mocks::mock_provenance_dependencies;
    use provwasm_std::shim::Timestamp;
    use provwasm_std::types::cosmos::bank::v1beta1::MsgSend;
    use provwasm_std::types::provenance::trigger::v1::{
        BlockHeightEvent, BlockTimeEvent, MsgCreateTriggerRequest,
    };

    use crate::contract::{execute, instantiate};
    use crate::msg::ExecuteMsg::CreateTrigger;
    use crate::msg::{Event, InitMsg};

    #[test]
    pub fn init() {
        let mut deps = mock_provenance_dependencies();

        assert!(instantiate(
            deps.as_mut(),
            mock_env(),
            message_info(&Addr::unchecked("sender"), &[]),
            InitMsg {},
        )
        .is_ok());
    }

    #[test]
    pub fn create_block_height() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[coin(100, "hash")]);
        let contract_address = env.contract.address.to_string();
        let receiver = Addr::unchecked("receiver");

        let expected_msg: Binary = MsgCreateTriggerRequest {
            authorities: vec![contract_address],
            event: Some(BlockHeightEvent { block_height: 50 }.to_any()),
            actions: vec![MsgSend {
                from_address: env.contract.address.to_string(),
                to_address: receiver.to_string(),
                amount: vec![coin(100, "hash").into()],
            }
            .to_any()],
        }
        .into();

        let msg = CreateTrigger {
            event: Event::BlockHeightEvent {
                block_height: 50u64.into(),
            },
            to_address: receiver.to_string(),
        };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(1, res.messages.len());

        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                assert_eq!(type_url, "/provenance.trigger.v1.MsgCreateTriggerRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    pub fn create_block_time() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("sender"), &[coin(100, "hash")]);
        let contract_address = env.contract.address.to_string();
        let receiver = Addr::unchecked("receiver");

        let expected_msg: Binary = MsgCreateTriggerRequest {
            authorities: vec![contract_address],
            event: Some(
                BlockTimeEvent {
                    time: Some(Timestamp {
                        seconds: 1690517980,
                        nanos: 0,
                    }),
                }
                .to_any(),
            ),
            actions: vec![MsgSend {
                from_address: env.contract.address.to_string(),
                to_address: receiver.to_string(),
                amount: vec![coin(100, "hash").into()],
            }
            .to_any()],
        }
        .into();

        let msg = CreateTrigger {
            event: Event::BlockTimeEvent {
                timestamp: 1690517980u64.into(),
            },
            to_address: receiver.to_string(),
        };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(1, res.messages.len());

        match &res.messages[0].msg {
            CosmosMsg::Any(AnyMsg { type_url, value }) => {
                assert_eq!(type_url, "/provenance.trigger.v1.MsgCreateTriggerRequest");
                assert_eq!(value, &expected_msg)
            }
            _ => panic!("unexpected cosmos message"),
        }
    }
}
