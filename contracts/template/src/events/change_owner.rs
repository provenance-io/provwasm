use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{CHANGE_OWNER_EVENT, CHANGE_OWNER_NEW, CHANGE_OWNER_PREVIOUS};

pub struct ChangeOwnerEvent {
    pub previous_owner: Addr,
    pub new_owner: Addr,
}

impl ChangeOwnerEvent {
    pub fn new(previous_owner: Addr, new_owner: Addr) -> Self {
        Self {
            previous_owner,
            new_owner,
        }
    }
}

impl From<ChangeOwnerEvent> for Event {
    fn from(val: ChangeOwnerEvent) -> Self {
        let event = Event::new(CHANGE_OWNER_EVENT);
        event.add_attributes([
            Attribute::new(CHANGE_OWNER_PREVIOUS, val.previous_owner.to_string()),
            Attribute::new(CHANGE_OWNER_NEW, val.new_owner.to_string()),
        ])
    }
}
