use std::vec;

use cosmwasm_std::Addr;

use crate::core::msg::{ExecuteMsg, InstantiateMsg};

use super::constants::{NEW_OWNER, OWNER, TAG1, TAG2};

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg::Default {
        owner: Addr::unchecked(OWNER),
        tag_types: vec![TAG1.to_string(), TAG2.to_string()],
    }
}

pub fn mock_change_owner_msg() -> ExecuteMsg {
    ExecuteMsg::ChangeOwner {
        new_owner: Addr::unchecked(NEW_OWNER),
    }
}

pub fn mock_add_tag_types_msg() -> ExecuteMsg {
    ExecuteMsg::AddTagTypes {
        tag_types: vec![TAG1.to_string(), TAG2.to_string()],
    }
}

pub fn mock_remove_tag_types_msg() -> ExecuteMsg {
    ExecuteMsg::RemoveTagTypes {
        tag_types: vec![TAG1.to_string(), TAG2.to_string()],
    }
}

pub fn mock_set_tag_msg(asset: &Addr) -> ExecuteMsg {
    ExecuteMsg::SetTag {
        asset_addr: asset.clone(),
        tag: TAG1.to_string(),
    }
}
