use std::vec;

use cosmwasm_std::Addr;

use crate::core::msg::{ExecuteMsg, InstantiateMsg, Security};

use super::constants::{NEW_OWNER, OWNER, TAG1, TAG2};

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg::Default {
        owner: Addr::unchecked(OWNER),
        security_types: vec![Security::new(TAG1), Security::new(TAG2)],
    }
}

pub fn mock_change_owner_msg() -> ExecuteMsg {
    ExecuteMsg::ChangeOwner {
        new_owner: Addr::unchecked(NEW_OWNER),
    }
}

pub fn mock_add_tag_types_msg() -> ExecuteMsg {
    ExecuteMsg::AddSecurityTypes {
        security_types: vec![Security::new(TAG1), Security::new(TAG2)],
    }
}

pub fn mock_remove_tag_types_msg() -> ExecuteMsg {
    ExecuteMsg::RemoveSecurityTypes {
        security_types: vec![Security::new(TAG1), Security::new(TAG2)],
    }
}

pub fn mock_set_tag_msg(asset: &Addr) -> ExecuteMsg {
    ExecuteMsg::SetSecurity {
        asset_addr: asset.clone(),
        security: Security::new(TAG1),
    }
}
