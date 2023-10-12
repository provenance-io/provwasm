use cosmwasm_std::{Addr, Attribute, Event};
use uuid::Uuid;

use crate::core::constants::{EVENT_TRANSFER, EVENT_TRANSFER_RECIPIENT, EVENT_TRANSFER_TOKEN_ID};

pub struct EventTransfer {
    pub recipient: Addr,
    pub token_id: Uuid,
}

impl From<EventTransfer> for Event {
    fn from(value: EventTransfer) -> Self {
        Event::new(EVENT_TRANSFER).add_attributes([
            Attribute::new(EVENT_TRANSFER_RECIPIENT, value.recipient.as_str()),
            Attribute::new(EVENT_TRANSFER_TOKEN_ID, value.token_id.to_string()),
        ])
    }
}
