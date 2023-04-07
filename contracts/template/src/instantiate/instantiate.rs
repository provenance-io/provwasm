use cosmwasm_std::{Addr, Response};

use crate::core::aliases::{ProvDepsMut, ProvTxResponse};

pub fn handle(deps: ProvDepsMut, owner: Addr) -> ProvTxResponse {
    Ok(Response::default())
}
