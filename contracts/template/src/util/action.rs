use cosmwasm_std::Attribute;

use crate::core::{
    aliases::ProvResponse,
    constants::{
        ACTION_ATTRIBUTE, ACTION_TYPE_CHANGE_OWNER, ACTION_TYPE_INITIALIZE, ACTION_TYPE_MIGRATE,
    },
};

pub enum ActionType {
    Initialize,
    Migrate,
    ChangeOwner,
}

impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::ChangeOwner => ACTION_TYPE_CHANGE_OWNER.to_string(),
            ActionType::Initialize => ACTION_TYPE_INITIALIZE.to_string(),
            ActionType::Migrate => ACTION_TYPE_MIGRATE.to_string(),
        }
    }
}

impl From<ActionType> for Attribute {
    fn from(val: ActionType) -> Self {
        Attribute::new(ACTION_ATTRIBUTE, val.to_string())
    }
}

pub trait Action {
    fn set_action(self, action_type: ActionType) -> Self;
}

impl Action for ProvResponse {
    fn set_action(self, action_type: ActionType) -> Self {
        self.add_attribute(ACTION_ATTRIBUTE, action_type.to_string())
    }
}
