use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use std::str::FromStr;
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::core::msg::InstantiateMsg;

use super::default;

pub fn route(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    match msg {
        InstantiateMsg::Default {
            contract_spec_uuid,
            minter,
            name,
            scope_spec_uuid,
            symbol,
        } => default::handle(
            deps,
            env,
            info,
            minter,
            name,
            symbol,
            Uuid::from_str(contract_spec_uuid.as_str()).unwrap(),
            Uuid::from_str(scope_spec_uuid.as_str()).unwrap(),
        ),
    }
}
