use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::{
    constants::{SET_SECURITY_ASSET, SET_SECURITY_EVENT, SET_SECURITY_VALUE},
    msg::Security,
};

/// An event to represent setting the tag for an asset
pub struct SetSecurityEvent {
    pub asset_addr: Addr,
    pub security: String,
}

impl SetSecurityEvent {
    /// Creates a new instance of SetSecurityEvent
    ///
    /// # Arguments
    ///
    /// * `asset_addr` - The address of the asset being modified.
    /// * `security` - The security that has been linked against the asset.
    ///
    /// # Examples
    /// ```
    /// let asset_addr = Addr::unchecked("address");
    /// let event = SetSecurityEvent::new(asset_addr, &Security::new("tag1"));
    /// ```
    pub fn new(asset_addr: &Addr, security: &Security) -> Self {
        Self {
            asset_addr: asset_addr.clone(),
            security: security.to_string(),
        }
    }
}

/// Allows us to easily convert a SetSecurityEvent into an Event.
impl From<SetSecurityEvent> for Event {
    fn from(val: SetSecurityEvent) -> Self {
        let event = Event::new(SET_SECURITY_EVENT);
        event.add_attributes([
            Attribute::new(SET_SECURITY_ASSET, val.asset_addr.to_string()),
            Attribute::new(SET_SECURITY_VALUE, val.security),
        ])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};

    use crate::{
        core::{
            constants::{SET_SECURITY_ASSET, SET_SECURITY_EVENT, SET_SECURITY_VALUE},
            msg::Security,
        },
        events::set_security::SetSecurityEvent,
    };

    #[test]
    fn test_new_set_tag_event() {
        let addr = Addr::unchecked("asset");
        let security = Security::new("tag");
        let event = SetSecurityEvent::new(&addr, &security);
        assert_eq!(addr, event.asset_addr);
        assert_eq!(security.to_string(), event.security);
    }

    #[test]
    fn test_from() {
        let asset = Addr::unchecked("asset");
        let security = Security::new("tag");
        let event = SetSecurityEvent::new(&asset, &security);

        let mut expected = Event::new(SET_SECURITY_EVENT);
        expected = expected.add_attributes([
            Attribute::new(SET_SECURITY_ASSET, asset.to_string()),
            Attribute::new(SET_SECURITY_VALUE, security.to_string()),
        ]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
