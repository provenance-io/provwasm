use cosmwasm_std::Attribute;

use crate::core::{
    aliases::ProvResponse,
    constants::{
        ACTION_ATTRIBUTE, ACTION_TYPE_ADD_TAGS, ACTION_TYPE_CHANGE_OWNER, ACTION_TYPE_INITIALIZE,
        ACTION_TYPE_MIGRATE, ACTION_TYPE_REMOVE_TAGS, ACTION_TYPE_SET_TAG,
    },
};

/// The different types of actions performed by the contract
pub enum ActionType {
    Initialize,
    Migrate,
    ChangeOwner,
    SetTag,
    AddTagTypes,
    RemoveTagTypes,
}

/// Provides a simple way to convert the ActionType to a string
impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::ChangeOwner => ACTION_TYPE_CHANGE_OWNER.to_string(),
            ActionType::SetTag => ACTION_TYPE_SET_TAG.to_string(),
            ActionType::AddTagTypes => ACTION_TYPE_ADD_TAGS.to_string(),
            ActionType::RemoveTagTypes => ACTION_TYPE_REMOVE_TAGS.to_string(),
            ActionType::Initialize => ACTION_TYPE_INITIALIZE.to_string(),
            ActionType::Migrate => ACTION_TYPE_MIGRATE.to_string(),
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
impl Action for ProvResponse {
    fn set_action(self, action_type: ActionType) -> Self {
        self.add_attribute(ACTION_ATTRIBUTE, action_type.to_string())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Attribute;

    use crate::core::constants::{
        ACTION_ATTRIBUTE, ACTION_TYPE_ADD_TAGS, ACTION_TYPE_CHANGE_OWNER, ACTION_TYPE_INITIALIZE,
        ACTION_TYPE_MIGRATE, ACTION_TYPE_REMOVE_TAGS, ACTION_TYPE_SET_TAG,
    };

    use super::ActionType;

    #[test]
    fn test_to_string() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;
        let action_type4 = ActionType::SetTag;
        let action_type5 = ActionType::AddTagTypes;
        let action_type6 = ActionType::RemoveTagTypes;
        assert_eq!(
            ACTION_TYPE_CHANGE_OWNER.to_string(),
            action_type1.to_string()
        );
        assert_eq!(ACTION_TYPE_INITIALIZE.to_string(), action_type2.to_string());
        assert_eq!(ACTION_TYPE_MIGRATE.to_string(), action_type3.to_string());
        assert_eq!(ACTION_TYPE_SET_TAG.to_string(), action_type4.to_string());
        assert_eq!(ACTION_TYPE_ADD_TAGS.to_string(), action_type5.to_string());
        assert_eq!(
            ACTION_TYPE_REMOVE_TAGS.to_string(),
            action_type6.to_string()
        );
    }

    #[test]
    fn test_from() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;
        let action_type4 = ActionType::SetTag;
        let action_type5 = ActionType::AddTagTypes;
        let action_type6 = ActionType::RemoveTagTypes;

        let attribute1 = Attribute::from(action_type1);
        let attribute2 = Attribute::from(action_type2);
        let attribute3 = Attribute::from(action_type3);
        let attribute4 = Attribute::from(action_type4);
        let attribute5 = Attribute::from(action_type5);
        let attribute6 = Attribute::from(action_type6);

        assert_eq!(attribute1.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute2.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute3.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute4.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute5.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute6.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute1.value, ACTION_TYPE_CHANGE_OWNER);
        assert_eq!(attribute2.value, ACTION_TYPE_INITIALIZE);
        assert_eq!(attribute3.value, ACTION_TYPE_MIGRATE);
        assert_eq!(attribute4.value, ACTION_TYPE_SET_TAG);
        assert_eq!(attribute5.value, ACTION_TYPE_ADD_TAGS);
        assert_eq!(attribute6.value, ACTION_TYPE_REMOVE_TAGS);
    }
}
