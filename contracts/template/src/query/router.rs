use cosmwasm_std::Env;

use crate::core::{
    aliases::{ProvDeps, ProvQueryResponse},
    msg::QueryMsg,
};

use super::{query_owner, query_version};

pub fn route(deps: ProvDeps, _env: Env, msg: QueryMsg) -> ProvQueryResponse {
    match msg {
        QueryMsg::QueryOwner {} => query_owner::handle(deps),
        QueryMsg::QueryVersion {} => query_version::handle(deps),
    }
}
