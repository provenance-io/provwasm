use cosmwasm_std::Attribute;

use crate::core::{
    aliases::ProvResponse,
    constants::{
        ACTION_ATTRIBUTE, ACTION_TYPE_ADD_SECURITIES, ACTION_TYPE_CHANGE_OWNER,
        ACTION_TYPE_INITIALIZE, ACTION_TYPE_MIGRATE, ACTION_TYPE_REMOVE_SECURITIES,
        ACTION_TYPE_REMOVE_SECURITY, ACTION_TYPE_SET_SECURITY, ACTION_TYPE_SET_SECURITY_MULTIPLE,
    },
};

/// The different types of actions performed by the contract
pub enum ActionType {
    Initialize,
    Migrate,
    ChangeOwner,
    SetSecurity,
    SetSecurityMultiple,
    RemoveSecurity,
    AddSecurityTypes,
    RemoveSecurityTypes,
}

/// Provides a simple way to convert the ActionType to a string
impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::ChangeOwner => ACTION_TYPE_CHANGE_OWNER.to_string(),
            ActionType::SetSecurity => ACTION_TYPE_SET_SECURITY.to_string(),
            ActionType::SetSecurityMultiple => ACTION_TYPE_SET_SECURITY_MULTIPLE.to_string(),
            ActionType::RemoveSecurity => ACTION_TYPE_REMOVE_SECURITY.to_string(),
            ActionType::AddSecurityTypes => ACTION_TYPE_ADD_SECURITIES.to_string(),
            ActionType::RemoveSecurityTypes => ACTION_TYPE_REMOVE_SECURITIES.to_string(),
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
        ACTION_ATTRIBUTE, ACTION_TYPE_ADD_SECURITIES, ACTION_TYPE_CHANGE_OWNER,
        ACTION_TYPE_INITIALIZE, ACTION_TYPE_MIGRATE, ACTION_TYPE_REMOVE_SECURITIES,
        ACTION_TYPE_REMOVE_SECURITY, ACTION_TYPE_SET_SECURITY, ACTION_TYPE_SET_SECURITY_MULTIPLE,
    };

    use super::ActionType;

    #[test]
    fn test_to_string() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;
        let action_type4 = ActionType::SetSecurity;
        let action_type5 = ActionType::AddSecurityTypes;
        let action_type6 = ActionType::RemoveSecurityTypes;
        let action_type7 = ActionType::RemoveSecurity;
        let action_type8 = ActionType::SetSecurityMultiple;
        assert_eq!(
            ACTION_TYPE_CHANGE_OWNER.to_string(),
            action_type1.to_string()
        );
        assert_eq!(ACTION_TYPE_INITIALIZE.to_string(), action_type2.to_string());
        assert_eq!(ACTION_TYPE_MIGRATE.to_string(), action_type3.to_string());
        assert_eq!(
            ACTION_TYPE_SET_SECURITY.to_string(),
            action_type4.to_string()
        );
        assert_eq!(
            ACTION_TYPE_ADD_SECURITIES.to_string(),
            action_type5.to_string()
        );
        assert_eq!(
            ACTION_TYPE_REMOVE_SECURITIES.to_string(),
            action_type6.to_string()
        );
        assert_eq!(
            ACTION_TYPE_REMOVE_SECURITY.to_string(),
            action_type7.to_string()
        );
        assert_eq!(
            ACTION_TYPE_SET_SECURITY_MULTIPLE.to_string(),
            action_type8.to_string()
        );
    }

    #[test]
    fn test_from() {
        let action_type1 = ActionType::ChangeOwner;
        let action_type2 = ActionType::Initialize;
        let action_type3 = ActionType::Migrate;
        let action_type4 = ActionType::SetSecurity;
        let action_type5 = ActionType::AddSecurityTypes;
        let action_type6 = ActionType::RemoveSecurityTypes;
        let action_type7 = ActionType::RemoveSecurity;
        let action_type8 = ActionType::SetSecurityMultiple;

        let attribute1 = Attribute::from(action_type1);
        let attribute2 = Attribute::from(action_type2);
        let attribute3 = Attribute::from(action_type3);
        let attribute4 = Attribute::from(action_type4);
        let attribute5 = Attribute::from(action_type5);
        let attribute6 = Attribute::from(action_type6);
        let attribute7 = Attribute::from(action_type7);
        let attribute8 = Attribute::from(action_type8);

        assert_eq!(attribute1.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute2.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute3.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute4.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute5.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute6.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute7.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute8.key, ACTION_ATTRIBUTE);
        assert_eq!(attribute1.value, ACTION_TYPE_CHANGE_OWNER);
        assert_eq!(attribute2.value, ACTION_TYPE_INITIALIZE);
        assert_eq!(attribute3.value, ACTION_TYPE_MIGRATE);
        assert_eq!(attribute4.value, ACTION_TYPE_SET_SECURITY);
        assert_eq!(attribute5.value, ACTION_TYPE_ADD_SECURITIES);
        assert_eq!(attribute6.value, ACTION_TYPE_REMOVE_SECURITIES);
        assert_eq!(attribute7.value, ACTION_TYPE_REMOVE_SECURITY);
        assert_eq!(attribute8.value, ACTION_TYPE_SET_SECURITY_MULTIPLE);
    }
}
