use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{
    EVENT_REVOKE_ALL, EVENT_REVOKE_ALL_OPERATOR, EVENT_REVOKE_ALL_SENDER,
};

pub struct EventRevokeAll {
    pub operator: Addr,
    pub sender: Addr,
}

impl From<EventRevokeAll> for Event {
    fn from(value: EventRevokeAll) -> Self {
        Event::new(EVENT_REVOKE_ALL).add_attributes([
            Attribute::new(EVENT_REVOKE_ALL_OPERATOR, value.operator),
            Attribute::new(EVENT_REVOKE_ALL_SENDER, value.sender),
        ])
    }
}
