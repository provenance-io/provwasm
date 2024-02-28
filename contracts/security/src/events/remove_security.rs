use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::{
    constants::{REMOVE_SECURITY_ASSET, REMOVE_SECURITY_EVENT, REMOVE_SECURITY_SECURITY},
    msg::Security,
};

/// An event to represent removing the security from an asset.
pub struct RemoveSecurityEvent {
    pub asset_addr: Addr,
    pub security: String,
}

impl RemoveSecurityEvent {
    /// Creates a new instance of RemoveSecurityEvent
    ///
    /// # Arguments
    ///
    /// * `asset_addr` - The address of the asset being modified.
    /// * `security` - The security tied to the asset.
    ///
    /// # Examples
    /// ```
    /// let asset_addr = Addr::unchecked("address");
    /// let security = Security::new("category");
    /// let event = RemoveSecurityEvent::new(&asset_addr, &security);
    /// ```
    pub fn new(asset_addr: &Addr, security: &Security) -> Self {
        Self {
            asset_addr: asset_addr.clone(),
            security: security.to_string(),
        }
    }
}

/// Allows us to easily convert a RemoveSecurityEvent into an Event.
impl From<RemoveSecurityEvent> for Event {
    fn from(val: RemoveSecurityEvent) -> Self {
        let event = Event::new(REMOVE_SECURITY_EVENT);
        event.add_attributes([
            Attribute::new(REMOVE_SECURITY_ASSET, val.asset_addr.to_string()),
            Attribute::new(REMOVE_SECURITY_SECURITY, val.security),
        ])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};

    use crate::{
        core::{
            constants::{REMOVE_SECURITY_ASSET, REMOVE_SECURITY_EVENT, REMOVE_SECURITY_SECURITY},
            msg::Security,
        },
        events::remove_security::RemoveSecurityEvent,
    };

    #[test]
    fn test_new_remove_security_event() {
        let addr = Addr::unchecked("asset");
        let security = Security::new("category");
        let event = RemoveSecurityEvent::new(&addr, &security);
        assert_eq!(addr, event.asset_addr);
        assert_eq!(addr.to_string(), event.asset_addr);
        assert_eq!(security.to_string(), event.security);
    }

    #[test]
    fn test_from() {
        let asset = Addr::unchecked("asset");
        let security = Security::new("category");
        let event = RemoveSecurityEvent::new(&asset, &security);

        let mut expected = Event::new(REMOVE_SECURITY_EVENT);
        expected = expected.add_attributes([
            Attribute::new(REMOVE_SECURITY_ASSET, asset.to_string()),
            Attribute::new(REMOVE_SECURITY_SECURITY, security.to_string()),
        ]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
