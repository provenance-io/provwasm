use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{CHANGE_OWNER_EVENT, CHANGE_OWNER_NEW, CHANGE_OWNER_PREVIOUS};

/// An event to represent the contract's owner changing
pub struct ChangeOwnerEvent {
    pub previous_owner: Addr,
    pub new_owner: Addr,
}

impl ChangeOwnerEvent {
    /// Creates a new instance of ChangeOwnerEvent
    ///
    /// # Arguments
    ///
    /// * `previous_owner` - The address of the contract's current owner.
    /// * `new_owner` - The address of the contract's next owner.
    ///
    /// # Examples
    /// ```
    /// let previous_owner = Addr::unchecked("previous");
    /// let new_owner = Addr::unchecked("new");
    /// let event = ChangeOwnerEvent::new(previous_owner, new_owner);
    /// ```
    pub fn new(previous_owner: Addr, new_owner: Addr) -> Self {
        Self {
            previous_owner,
            new_owner,
        }
    }
}

/// Allows us to easily convert a ChangeOwnerEvent into an Event.
impl From<ChangeOwnerEvent> for Event {
    fn from(val: ChangeOwnerEvent) -> Self {
        let event = Event::new(CHANGE_OWNER_EVENT);
        event.add_attributes([
            Attribute::new(CHANGE_OWNER_PREVIOUS, val.previous_owner.to_string()),
            Attribute::new(CHANGE_OWNER_NEW, val.new_owner.to_string()),
        ])
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Attribute, Event};

    use crate::core::constants::{CHANGE_OWNER_EVENT, CHANGE_OWNER_NEW, CHANGE_OWNER_PREVIOUS};

    use super::ChangeOwnerEvent;

    #[test]
    fn test_new_change_owner_event() {
        let previous_owner = Addr::unchecked("previous");
        let new_owner = Addr::unchecked("new");
        let event = ChangeOwnerEvent::new(previous_owner.clone(), new_owner.clone());
        assert_eq!(previous_owner, event.previous_owner);
        assert_eq!(new_owner, event.new_owner);
    }

    #[test]
    fn test_from() {
        let previous_owner = Addr::unchecked("previous");
        let new_owner = Addr::unchecked("new");
        let event = ChangeOwnerEvent::new(previous_owner.clone(), new_owner.clone());

        let mut expected = Event::new(CHANGE_OWNER_EVENT);
        expected = expected.add_attributes([
            Attribute::new(CHANGE_OWNER_PREVIOUS, previous_owner.to_string()),
            Attribute::new(CHANGE_OWNER_NEW, new_owner.to_string()),
        ]);

        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
