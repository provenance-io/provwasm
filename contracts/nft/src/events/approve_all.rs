use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{
    EVENT_APPROVE_ALL, EVENT_APPROVE_ALL_OPERATOR, EVENT_APPROVE_ALL_SENDER,
};

pub struct EventApproveAll {
    pub operator: Addr,
    pub sender: Addr,
}

impl From<EventApproveAll> for Event {
    fn from(value: EventApproveAll) -> Self {
        Event::new(EVENT_APPROVE_ALL).add_attributes([
            Attribute::new(EVENT_APPROVE_ALL_OPERATOR, value.operator),
            Attribute::new(EVENT_APPROVE_ALL_SENDER, value.sender),
        ])
    }
}
