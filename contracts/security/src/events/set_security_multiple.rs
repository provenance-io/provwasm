use cosmwasm_std::{Attribute, Event, Uint64};

use crate::core::{
    constants::{
        SET_SECURITY_MULTIPLE_EVENT, SET_SECURITY_MULTIPLE_NUM, SET_SECURITY_MULTIPLE_VALUE,
    },
    msg::Security,
};

/// An event to represent setting the security for multiple assets.
pub struct SetSecurityMultipleEvent {
    pub num_assets: Uint64,
    pub security: String,
}

impl SetSecurityMultipleEvent {
    /// Creates a new instance of SetSecurityMultipleEvent
    ///
    /// # Arguments
    ///
    /// * `num_assets` - The number of assets linked to the security.
    /// * `security` - The security that has been linked against each asset.
    ///
    /// # Examples
    /// ```
    /// let event = SetSecurityMultipleEvent::new(5, &Security::new("tag1"));
    /// ```
    pub fn new(num_assets: Uint64, security: &Security) -> Self {
        Self {
            num_assets,
            security: security.to_string(),
        }
    }
}

/// Allows us to easily convert a SetSecurityEvent into an Event.
impl From<SetSecurityMultipleEvent> for Event {
    fn from(val: SetSecurityMultipleEvent) -> Self {
        let event = Event::new(SET_SECURITY_MULTIPLE_EVENT);
        event.add_attributes([
            Attribute::new(SET_SECURITY_MULTIPLE_NUM, val.num_assets),
            Attribute::new(SET_SECURITY_MULTIPLE_VALUE, val.security),
        ])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Attribute, Event, Uint64};

    use crate::{
        core::{
            constants::{
                SET_SECURITY_MULTIPLE_EVENT, SET_SECURITY_MULTIPLE_NUM, SET_SECURITY_MULTIPLE_VALUE,
            },
            msg::Security,
        },
        events::set_security_multiple::SetSecurityMultipleEvent,
    };

    #[test]
    fn test_new_set_tag_event() {
        let amount = Uint64::new(5);
        let security = Security::new("tag");
        let event = SetSecurityMultipleEvent::new(amount, &security);
        assert_eq!(amount, event.num_assets);
        assert_eq!(security.to_string(), event.security);
    }

    #[test]
    fn test_from() {
        let amount = Uint64::new(5);
        let security = Security::new("tag");
        let event = SetSecurityMultipleEvent::new(amount, &security);

        let mut expected = Event::new(SET_SECURITY_MULTIPLE_EVENT);
        expected = expected.add_attributes([
            Attribute::new(SET_SECURITY_MULTIPLE_NUM, amount),
            Attribute::new(SET_SECURITY_MULTIPLE_VALUE, security.to_string()),
        ]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
