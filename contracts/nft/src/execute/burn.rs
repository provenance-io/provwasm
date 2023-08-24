use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::MsgDeleteScopeRequest;

use crate::core::error::ContractError;
use crate::util::action::{Action, ActionType};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let burn_msg = MsgDeleteScopeRequest {
        scope_id: id.into_bytes(),
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
    };

    Ok(Response::default()
        .set_action(ActionType::Execute {})
        .add_message(burn_msg))
}
