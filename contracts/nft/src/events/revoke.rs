use cosmwasm_std::{Addr, Attribute, Event};

use crate::core::constants::{EVENT_REVOKE, EVENT_REVOKE_SPENDER, EVENT_REVOKE_TOKEN_ID};

pub struct EventRevoke<'a> {
    pub spender: Addr,
    pub token_id: &'a str,
}

impl<'a> From<EventRevoke<'a>> for Event {
    fn from(value: EventRevoke) -> Self {
        Event::new(EVENT_REVOKE).add_attributes([
            Attribute::new(EVENT_REVOKE_SPENDER, value.spender.as_str()),
            Attribute::new(EVENT_REVOKE_TOKEN_ID, value.token_id.to_string()),
        ])
    }
}
