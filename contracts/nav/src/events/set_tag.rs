use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{SET_TAG_ASSET, SET_TAG_EVENT, SET_TAG_VALUE};

/// An event to represent setting the tag for an asset
pub struct SetTagEvent {
    pub asset_addr: Addr,
    pub tag: String,
}

impl SetTagEvent {
    /// Creates a new instance of SetTagEvent
    ///
    /// # Arguments
    ///
    /// * `asset_addr` - The address of the asset being modified.
    /// * `tag` - The category to be applied to the asset.
    ///
    /// # Examples
    /// ```
    /// let asset_addr = Addr::unchecked("address");
    /// let tag = Addr::unchecked("new");
    /// let event = SetTagEvent::new(asset_addr, "random");
    /// ```
    pub fn new(asset_addr: &Addr, tag: &str) -> Self {
        Self {
            asset_addr: asset_addr.clone(),
            tag: tag.to_string(),
        }
    }
}

/// Allows us to easily convert a SetTagEvent into an Event.
impl From<SetTagEvent> for Event {
    fn from(val: SetTagEvent) -> Self {
        let event = Event::new(SET_TAG_EVENT);
        event.add_attributes([
            Attribute::new(SET_TAG_ASSET, val.asset_addr.to_string()),
            Attribute::new(SET_TAG_VALUE, val.tag),
        ])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};

    use crate::core::constants::{SET_TAG_ASSET, SET_TAG_EVENT, SET_TAG_VALUE};

    use super::SetTagEvent;

    #[test]
    fn test_new_set_tag_event() {
        let addr = Addr::unchecked("asset");
        let tag = "tag";
        let event = SetTagEvent::new(&addr, tag);
        assert_eq!(addr, event.asset_addr);
        assert_eq!(tag, event.tag);
    }

    #[test]
    fn test_from() {
        let asset = Addr::unchecked("asset");
        let tag = "tag";
        let event = SetTagEvent::new(&asset, tag);

        let mut expected = Event::new(SET_TAG_EVENT);
        expected = expected.add_attributes([
            Attribute::new(SET_TAG_ASSET, asset.to_string()),
            Attribute::new(SET_TAG_VALUE, tag.to_string()),
        ]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
