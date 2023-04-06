use cosmwasm_std::{Addr, Response};

use crate::core::aliases::ProvTxResponse;

pub fn handle(sender: Addr, new_owner: Addr) -> ProvTxResponse {
    Ok(Response::default())
}
