use cosmwasm_std::{Attribute, Response};

use crate::core::constants::{
    ACTION_ATTRIBUTE, ACTION_TYPE_APPROVE_ALL, ACTION_TYPE_BURN, ACTION_TYPE_INITIALIZE,
    ACTION_TYPE_MINT, ACTION_TYPE_REVOKE, ACTION_TYPE_SEND, ACTION_TYPE_TRANSFER,
};

/// The different types of actions performed by the contract
pub enum ActionType {
    Approve,
    ApproveAll,
    Burn,
    Initialize,
    Mint,
    Revoke,
    RevokeAll,
    Send,
    Transfer,
}

/// Provides a simple way to convert the ActionType to a string
impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::Approve => ACTION_TYPE_INITIALIZE.to_string(),
            ActionType::ApproveAll => ACTION_TYPE_APPROVE_ALL.to_string(),
            ActionType::Burn => ACTION_TYPE_BURN.to_string(),
            ActionType::Initialize => ACTION_TYPE_INITIALIZE.to_string(),
            ActionType::Mint => ACTION_TYPE_MINT.to_string(),
            ActionType::Revoke => ACTION_TYPE_REVOKE.to_string(),
            ActionType::RevokeAll => ACTION_TYPE_APPROVE_ALL.to_string(),
            ActionType::Send => ACTION_TYPE_SEND.to_string(),
            ActionType::Transfer => ACTION_TYPE_TRANSFER.to_string(),
        }
    }
}

/// Allows us to easily convert an ActionType to an Attribute
impl From<ActionType> for Attribute {
    fn from(val: ActionType) -> Self {
        Attribute::new(ACTION_ATTRIBUTE, val.to_string())
    }
}

/// Provides the implementer with the ability to add an ActionType
pub trait Action {
    fn set_action(self, action_type: ActionType) -> Self;
}

/// Adds an action attribute to the response
impl Action for Response {
    fn set_action(self, action_type: ActionType) -> Self {
        self.add_attribute(ACTION_ATTRIBUTE, action_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Attribute;

    use crate::core::constants::{ACTION_ATTRIBUTE, ACTION_TYPE_INITIALIZE};

    use super::ActionType;

    #[test]
    fn test_to_string() {
        let action_type2 = ActionType::Initialize;
        assert_eq!(ACTION_TYPE_INITIALIZE.to_string(), action_type2.to_string());
    }

    #[test]
    fn test_from() {
        let action_type2 = ActionType::Initialize;

        let attribute2 = Attribute::from(action_type2);

        assert_eq!(attribute2.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute2.value, ACTION_TYPE_INITIALIZE);
    }
}
