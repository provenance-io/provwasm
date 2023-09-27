use cosmwasm_std::{Addr, Attribute, Event};
use uuid::Uuid;

use crate::core::constants::{EVENT_MINT, EVENT_MINT_RECIPIENT, EVENT_MINT_TOKEN_ID};

pub struct EventMint {
    pub recipient: Addr,
    pub token_id: Uuid,
}

impl From<EventMint> for Event {
    fn from(value: EventMint) -> Self {
        Event::new(EVENT_MINT).add_attributes([
            Attribute::new(EVENT_MINT_RECIPIENT, value.recipient.as_str()),
            Attribute::new(EVENT_MINT_TOKEN_ID, value.token_id.to_string()),
        ])
    }
}
