use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, Uint64,
};
use provwasm_std::shim::{Any, Timestamp};
use provwasm_std::types::cosmos::bank::v1beta1::MsgSend;
use provwasm_std::types::provenance::trigger::v1::{
    BlockHeightEvent, BlockTimeEvent, MsgCreateTriggerRequest, MsgDestroyTriggerRequest,
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
        .try_into()
        .unwrap(),
        Event::BlockTimeEvent { timestamp } => BlockTimeEvent {
            time: Some(Timestamp {
                seconds: timestamp.u64() as i64,
                nanos: 0,
            }),
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

/// Handle query requests for the provenance trigger module
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Err(ContractError::Std(StdError::GenericErr {
        msg: "not implemented".to_string(),
    }))
}

#[cfg(test)]
pub mod test {
    use provwasm_std::shim::Timestamp;

    #[test]
    pub fn timestamp_test() {
        let time = 1690500001u64;
        let timestamp = Timestamp {
            seconds: time as i64,
            nanos: 0,
        };

        println!("timestamp: {:?}", timestamp);
    }
}
