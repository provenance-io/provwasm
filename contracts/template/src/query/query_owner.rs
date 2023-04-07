use cosmwasm_std::{to_binary, Addr};

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryOwnerResponse,
};

pub fn handle(_deps: ProvDeps) -> ProvQueryResponse {
    let res = QueryOwnerResponse {
        owner: Addr::unchecked("test"),
    };
    Ok(to_binary(&res)?)
}
