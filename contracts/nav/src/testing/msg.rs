use cosmwasm_std::Addr;

use crate::core::msg::{ExecuteMsg, InstantiateMsg};

use super::constants::{NEW_OWNER, OWNER};

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg::Default {
        owner: Addr::unchecked(OWNER),
        tag_types: vec!["tag1".to_string(), "tag2".to_string()],
    }
}

pub fn mock_change_owner_msg() -> ExecuteMsg {
    ExecuteMsg::ChangeOwner {
        new_owner: Addr::unchecked(NEW_OWNER),
    }
}
