use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw_utils::Expiration;

use crate::core::error::ContractError;
use crate::storage::nft::{Approval, TOKENS};
use crate::util::permission;

pub fn handle(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: &str,
    token_id: &str,
    // if add == false, remove. if add == true, remove then set with this expiration
    add: bool,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let mut nft = TOKENS.load(deps.storage, token_id)?;
    // ensure we have permissions
    permission::can_approve(deps.as_ref(), env, info, &nft)?;

    // update the approval list (remove any for the same spender before adding)
    let spender_addr = deps.api.addr_validate(spender)?;
    nft.approvals.retain(|apr| apr.spender != spender_addr);

    // only difference between approve and revoke
    if add {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }
        let approval = Approval {
            spender: spender_addr,
            expires,
        };
        nft.approvals.push(approval);
    }

    TOKENS.save(deps.storage, token_id, &nft)?;

    Ok()
}
