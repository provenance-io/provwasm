use cosmwasm_std::{testing::mock_env, Addr, Coin, DepsMut, MessageInfo};

use crate::{contract, util::fee::Fee};

use super::{
    constants::{CREATOR, OWNER, TEST_AMOUNT, TEST_DENOM},
    msg::mock_instantiate_msg,
};

pub fn mock_fee() -> Fee {
    Fee {
        recipient: Some(Addr::unchecked(OWNER)),
        amount: Coin::new(TEST_AMOUNT, TEST_DENOM),
    }
}

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
    let info = mock_info(true, CREATOR);
    let env = mock_env();
    let msg = mock_instantiate_msg(true);
    contract::instantiate(deps, env, info, msg).unwrap();
}
