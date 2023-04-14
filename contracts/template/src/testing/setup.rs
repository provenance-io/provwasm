use cosmwasm_std::{testing::mock_env, Addr, Coin, MessageInfo};

use crate::{contract, core::aliases::ProvDepsMut, util::fee::Fee};

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

pub fn mock_info() -> MessageInfo {
    MessageInfo {
        sender: Addr::unchecked(CREATOR),
        funds: vec![Coin::new(TEST_AMOUNT, TEST_DENOM)],
    }
}

pub fn mock_contract(deps: ProvDepsMut) {
    let info = mock_info();
    let env = mock_env();
    let msg = mock_instantiate_msg(true);
    contract::instantiate(deps, env, info, msg).unwrap();
}
