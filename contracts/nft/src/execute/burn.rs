use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::MsgDeleteScopeRequest;
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::events::burn::EventBurn;
use crate::util::action::{Action, ActionType};
use crate::util::metadata_address::MetadataAddress;

pub fn handle(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: Uuid,
) -> Result<Response, ContractError> {
    let burn_msg = MsgDeleteScopeRequest {
        scope_id: MetadataAddress::scope(token_id)?.bytes,
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
    };

    Ok(Response::default()
        .set_action(ActionType::Burn {})
        .add_event(EventBurn { token_id }.into())
        .add_message(burn_msg))
}
