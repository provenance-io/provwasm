use cosmwasm_std::{testing::mock_env, Addr, Coin, DepsMut, MessageInfo};

use crate::contract;

use super::{
    constants::{CREATOR, TEST_AMOUNT, TEST_DENOM},
    msg::mock_instantiate_msg,
};

pub fn mock_info(funds: bool, sender: &str) -> MessageInfo {
    match funds {
        false => MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![Coin::new(TEST_AMOUNT, TEST_DENOM)],
        },
        true => MessageInfo {
            sender: Addr::unchecked(sender),
            funds: vec![],
        },
    }
}

pub fn mock_contract(deps: DepsMut) {
    let info = mock_info(true, CREATOR);
    let env = mock_env();
    let msg = mock_instantiate_msg();
    contract::instantiate(deps, env, info, msg).unwrap();
}
