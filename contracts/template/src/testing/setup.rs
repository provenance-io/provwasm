use cosmwasm_std::{testing::mock_env, Addr, Coin, MessageInfo};

use crate::{
    contract,
    core::{aliases::ProvDepsMut, msg::InstantiateMsg},
    util::fee::Fee,
};

use super::constants::{CREATOR, OWNER, TEST_DENOM};

pub fn mock_instantiate_msg(has_fee: bool) -> InstantiateMsg {
    if has_fee {
        InstantiateMsg::Default {
            owner: Addr::unchecked(OWNER),
            fee: Fee {
                recipient: None,
                amount: Coin::new(100, TEST_DENOM),
            },
        }
    } else {
        InstantiateMsg::Default {
            owner: Addr::unchecked(OWNER),
            fee: Fee {
                recipient: None,
                amount: Coin::new(0, TEST_DENOM),
            },
        }
    }
}

pub fn mock_contract(deps: ProvDepsMut) {
    let info = MessageInfo {
        sender: Addr::unchecked(CREATOR),
        funds: vec![],
    };
    let env = mock_env();
    let msg = mock_instantiate_msg(false);
    contract::instantiate(deps, env, info, msg).unwrap();
}
