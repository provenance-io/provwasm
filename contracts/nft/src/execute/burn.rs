use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::MsgDeleteScopeRequest;
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::events::burn::EventBurn;
use crate::storage::nft::TOKENS;
use crate::storage::nft_count;
use crate::util::action::{Action, ActionType};
use crate::util::metadata_address::MetadataAddress;
use crate::util::permission;

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: Uuid,
) -> Result<Response, ContractError> {
    // check if sender can burn token
    let nft = TOKENS.load(deps.storage, &token_id.to_string())?;
    permission::can_send(deps.as_ref(), &env, &info, &nft)?;

    let burn_msg = MsgDeleteScopeRequest {
        scope_id: MetadataAddress::scope(token_id)?.bytes,
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
    };

    nft_count::decrement_nft_count(deps.storage)?;

    Ok(Response::default()
        .set_action(ActionType::Burn {})
        .add_event(EventBurn { token_id }.into())
        .add_message(burn_msg))
}
