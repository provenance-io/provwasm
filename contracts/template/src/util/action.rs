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

#[cfg(test)]
mod tests {
    use cosmwasm_std::Attribute;

    use crate::core::constants::{
        ACTION_ATTRIBUTE, ACTION_TYPE_CHANGE_OWNER, ACTION_TYPE_INITIALIZE, ACTION_TYPE_MIGRATE,
    };

    use super::ActionType;

    #[test]
    fn test_to_string() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;
        assert_eq!(
            ACTION_TYPE_CHANGE_OWNER.to_string(),
            action_type1.to_string()
        );
        assert_eq!(ACTION_TYPE_INITIALIZE.to_string(), action_type2.to_string());
        assert_eq!(ACTION_TYPE_MIGRATE.to_string(), action_type3.to_string());
    }

    #[test]
    fn test_from() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;

        let attribute1 = Attribute::from(action_type1);
        let attribute2 = Attribute::from(action_type2);
        let attribute3 = Attribute::from(action_type3);

        assert_eq!(attribute1.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute2.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute3.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute1.value, ACTION_TYPE_CHANGE_OWNER);
        assert_eq!(attribute2.value, ACTION_TYPE_INITIALIZE);
        assert_eq!(attribute3.value, ACTION_TYPE_MIGRATE);
    }
}
