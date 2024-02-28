use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{REMOVE_SECURITY_ASSET, REMOVE_SECURITY_EVENT};

/// An event to represent removing the security from an asset.
pub struct RemoveSecurityEvent {
    pub asset_addr: Addr,
}

impl RemoveSecurityEvent {
    /// Creates a new instance of RemoveSecurityEvent
    ///
    /// # Arguments
    ///
    /// * `asset_addr` - The address of the asset being modified.
    ///
    /// # Examples
    /// ```
    /// let asset_addr = Addr::unchecked("address");
    /// let event = RemoveSecurityEvent::new(asset_addr);
    /// ```
    pub fn new(asset_addr: &Addr) -> Self {
        Self {
            asset_addr: asset_addr.clone(),
        }
    }
}

/// Allows us to easily convert a RemoveSecurityEvent into an Event.
impl From<RemoveSecurityEvent> for Event {
    fn from(val: RemoveSecurityEvent) -> Self {
        let event = Event::new(REMOVE_SECURITY_EVENT);
        event.add_attributes([Attribute::new(
            REMOVE_SECURITY_ASSET,
            val.asset_addr.to_string(),
        )])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};

    use crate::{
        core::constants::{REMOVE_SECURITY_ASSET, REMOVE_SECURITY_EVENT},
        events::remove_security::RemoveSecurityEvent,
    };

    #[test]
    fn test_new_remove_security_event() {
        let addr = Addr::unchecked("asset");
        let event = RemoveSecurityEvent::new(&addr);
        assert_eq!(addr, event.asset_addr);
        assert_eq!(addr.to_string(), event.asset_addr);
    }

    #[test]
    fn test_from() {
        let asset = Addr::unchecked("asset");
        let event = RemoveSecurityEvent::new(&asset);

        let mut expected = Event::new(REMOVE_SECURITY_EVENT);
        expected =
            expected.add_attributes([Attribute::new(REMOVE_SECURITY_ASSET, asset.to_string())]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
