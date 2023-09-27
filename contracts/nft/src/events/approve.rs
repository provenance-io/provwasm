use crate::core::constants::{EVENT_APPROVE, EVENT_APPROVE_SPENDER, EVENT_APPROVE_TOKEN_ID};
use cosmwasm_std::{Addr, Attribute, Event};

pub struct EventApprove<'a> {
    pub spender: Addr,
    pub token_id: &'a str,
}

impl<'a> From<EventApprove<'a>> for Event {
    fn from(value: EventApprove) -> Self {
        Event::new(EVENT_APPROVE).add_attributes([
            Attribute::new(EVENT_APPROVE_SPENDER, value.spender.as_str()),
            Attribute::new(EVENT_APPROVE_TOKEN_ID, value.token_id),
        ])
    }
}
