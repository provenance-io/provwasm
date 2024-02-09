use cosmwasm_std::{testing::mock_env, Addr, Coin, DepsMut, MessageInfo};

use crate::contract;

use super::{
    constants::{CREATOR, TEST_AMOUNT, TEST_DENOM},
    msg::mock_instantiate_msg,
};

pub fn mock_info(funds: bool, sender: &str) -> MessageInfo {
    if funds {
        return MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![Coin::new(TEST_AMOUNT, TEST_DENOM)],
        };
    }
    return MessageInfo {
        sender: Addr::unchecked(sender),
        funds: vec![],
    };
}

pub fn mock_contract(deps: DepsMut) {
    let info = mock_info(false, CREATOR);
    let env = mock_env();
    let msg = mock_instantiate_msg();
    contract::instantiate(deps, env, info, msg).unwrap();
}

pub fn mock_scopes(deps: DepsMut) {
    let path = "/provenance.metadata.v1.Query/Scope";
    //deps.querier.
}

pub fn mock_markers(deps: DepsMut) {
    let path = "/provenance.marker.v1.Query/Marker";
    //deps.querier.
}
