use cosmwasm_std::{Attribute, Response};

use crate::core::constants::{ACTION_ATTRIBUTE, ACTION_TYPE_EXECUTE, ACTION_TYPE_INITIALIZE};

/// The different types of actions performed by the contract
pub enum ActionType {
    Initialize,
    Execute,
}

/// Provides a simple way to convert the ActionType to a string
impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::Initialize => ACTION_TYPE_INITIALIZE.to_string(),
            ActionType::Execute => ACTION_TYPE_EXECUTE.to_string(),
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
