use cosmwasm_std::{Addr, Coin};

use crate::{core::msg::InstantiateMsg, util::fee::Fee};

use super::constants::{OWNER, TEST_DENOM};

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
