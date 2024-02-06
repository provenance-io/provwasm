use cosmwasm_std::Addr;

use crate::core::msg::{ExecuteMsg, InstantiateMsg};

use super::constants::{NEW_OWNER, OWNER};

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg::Default {
        owner: Addr::unchecked(OWNER),
    }
}

pub fn mock_change_owner_msg() -> ExecuteMsg {
    ExecuteMsg::ChangeOwner {
        new_owner: Addr::unchecked(NEW_OWNER),
    }
}
