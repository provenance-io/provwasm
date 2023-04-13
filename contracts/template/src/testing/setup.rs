use cosmwasm_std::{testing::mock_env, Addr, MessageInfo};

use crate::{contract, core::aliases::ProvDepsMut};

use super::{constants::CREATOR, msg::mock_instantiate_msg};

pub fn mock_contract(deps: ProvDepsMut) {
    let info = MessageInfo {
        sender: Addr::unchecked(CREATOR),
        funds: vec![],
    };
    let env = mock_env();
    let msg = mock_instantiate_msg(false);
    contract::instantiate(deps, env, info, msg).unwrap();
}
