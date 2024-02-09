use cosmwasm_std::Event;

use crate::core::constants::UPDATE_TAG_TYPES_EVENT;

/// An event to represent the contract's supported tag types changing.
pub struct UpdateTagTypesEvent {}

impl UpdateTagTypesEvent {
    /// Creates a new instance of UpdateTagTypesEvent
    ///
    /// # Arguments
    ///
    /// # Examples
    /// ```
    /// let event = UpdateTagTypesEvent::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Allows us to easily convert a ChangeOwnerEvent into an Event.
impl From<UpdateTagTypesEvent> for Event {
    fn from(_val: UpdateTagTypesEvent) -> Self {
        Event::new(UPDATE_TAG_TYPES_EVENT)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Event;

    use crate::{
        core::constants::UPDATE_TAG_TYPES_EVENT, events::update_tag_types::UpdateTagTypesEvent,
    };

    #[test]
    fn test_from() {
        let event = UpdateTagTypesEvent::new();
        let expected = Event::new(UPDATE_TAG_TYPES_EVENT);
        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
