use cosmwasm_std::{Addr, Coin};

use crate::{
    core::msg::{ExecuteMsg, InstantiateMsg},
    util::fee::Fee,
};

use super::constants::{NEW_OWNER, OWNER, TEST_AMOUNT, TEST_DENOM};

pub fn mock_instantiate_msg(has_fee: bool) -> InstantiateMsg {
    if has_fee {
        InstantiateMsg::Default {
            owner: Addr::unchecked(OWNER),
            fee: Fee {
                recipient: Some(Addr::unchecked(OWNER)),
                amount: Coin::new(TEST_AMOUNT, TEST_DENOM),
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

pub fn mock_change_owner_msg() -> ExecuteMsg {
    ExecuteMsg::ChangeOwner {
        new_owner: Addr::unchecked(NEW_OWNER),
    }
}
