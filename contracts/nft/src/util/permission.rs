use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo};
use cw_ownable::OwnershipError;
use cw_utils::Expiration;

use crate::core::error::ContractError;
use crate::storage::nft::{Approval, Nft, TOKENS};
use crate::storage::operators::OPERATORS;

pub fn can_approve(
    deps: Deps,
    env: &Env,
    info: &MessageInfo,
    token: &Nft,
) -> Result<(), ContractError> {
    // owner can approve
    if token.owner == info.sender {
        return Ok(());
    }
    // operator can approve
    let op = OPERATORS.may_load(deps.storage, (&token.owner, &info.sender))?;
    match op {
        Some(ex) => {
            if ex.is_expired(&env.block) {
                Err(ContractError::Ownership(OwnershipError::NotOwner))
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
    }
}

pub fn can_send(
    deps: Deps,
    env: &Env,
    info: &MessageInfo,
    token: &Nft,
) -> Result<(), ContractError> {
    // owner can send
    if token.owner == info.sender {
        return Ok(());
    }

    // any non-expired token approval can send
    if token
        .approvals
        .iter()
        .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
    {
        return Ok(());
    }

    // operator can send
    let op = OPERATORS.may_load(deps.storage, (&token.owner, &info.sender))?;
    match op {
        Some(ex) => {
            if ex.is_expired(&env.block) {
                Err(ContractError::Ownership(OwnershipError::NotOwner))
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
    }
}

pub fn handle(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: &str,
    token_id: &str,
    // if add == false, remove. if add == true, remove then set with this expiration
    add: bool,
    expires: Option<Expiration>,
) -> Result<Nft, ContractError> {
    let mut nft = TOKENS.load(deps.storage, token_id)?;
    // ensure we have permissions
    can_approve(deps.as_ref(), env, info, &nft)?;

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

    Ok(nft)
}
