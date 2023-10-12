use cosmwasm_std::{Attribute, Event};
use uuid::Uuid;

use crate::core::constants::{EVENT_BURN, EVENT_BURN_TOKEN_ID};

pub struct EventBurn {
    pub token_id: Uuid,
}

impl From<EventBurn> for Event {
    fn from(value: EventBurn) -> Self {
        Event::new(EVENT_BURN).add_attributes([Attribute::new(
            EVENT_BURN_TOKEN_ID,
            value.token_id.to_string(),
        )])
    }
}
