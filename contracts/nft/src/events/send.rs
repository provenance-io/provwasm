use cosmwasm_std::{Addr, Attribute, Event};
use uuid::Uuid;

use crate::core::constants::{EVENT_SEND, EVENT_SEND_CONTRACT, EVENT_SEND_TOKEN_ID};

pub struct EventSend {
    pub contract: Addr,
    pub token_id: Uuid,
}

impl From<EventSend> for Event {
    fn from(value: EventSend) -> Self {
        Event::new(EVENT_SEND).add_attributes([
            Attribute::new(EVENT_SEND_CONTRACT, value.contract.as_str()),
            Attribute::new(EVENT_SEND_TOKEN_ID, value.token_id.to_string()),
        ])
    }
}
