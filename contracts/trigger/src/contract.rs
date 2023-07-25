use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
};
use provwasm_std::shim::Any;
use provwasm_std::types::cosmos::bank::v1beta1::MsgSend;
use provwasm_std::types::provenance::trigger::v1::{
    BlockHeightEvent, BlockTimeEvent, MsgCreateTriggerRequest, TransactionEvent,
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
        ExecuteMsg::DeleteTrigger {} => delete_trigger(deps, env, info),
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
        .try_into()
        .unwrap(),
        Event::BlockTimeEvent => BlockTimeEvent { time: None }.try_into().unwrap(),
        Event::TransactionEvent => TransactionEvent {
            name: "".to_string(),
            attributes: vec![],
        }
        .try_into()
        .unwrap(),
    };

    let msg = MsgCreateTriggerRequest {
        authorities: vec![env.contract.address.to_string()],
        event: Some(trigger_event),
        actions: vec![MsgSend {
            from_address: env.contract.address.to_string(),
            to_address,
            amount: info.funds.into_iter().map(|fund| fund.into()).collect(),
        }
        .try_into()
        .unwrap()],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "provwasm.contracts.trigger.create_trigger"))
}

// delete an existing trigger
pub fn delete_trigger(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}

/// Handle query requests for the provenance trigger module
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}
