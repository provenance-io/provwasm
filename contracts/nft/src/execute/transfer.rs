use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::MsgWriteScopeRequest;
use uuid::Uuid;

use crate::{
    core::error::ContractError,
    util::action::{Action, ActionType},
};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Addr,
    scope_uuid: Uuid,
) -> Result<Response, ContractError> {
    let change_owner_msg = MsgWriteScopeRequest {
        scope: None,
        signers: vec![],
        scope_uuid: scope_uuid.to_string(),
        spec_uuid: "".to_string(),
    };

    Ok(Response::default()
        .set_action(ActionType::Execute {})
        .add_message(change_owner_msg))
}
