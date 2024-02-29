use cosmwasm_std::Event;

use crate::core::constants::UPDATE_SECURITY_TYPES_EVENT;

/// An event to represent the contract's supported security types changing.
pub struct UpdateSecurityTypesEvent {}

impl UpdateSecurityTypesEvent {
    /// Creates a new instance of UpdateSecurityTypesEvent
    ///
    /// # Arguments
    ///
    /// # Examples
    /// ```
    /// let event = UpdateSecurityTypesEvent::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Allows us to easily convert a UpdateSecurityTypesEvent into an Event.
impl From<UpdateSecurityTypesEvent> for Event {
    fn from(_val: UpdateSecurityTypesEvent) -> Self {
        Event::new(UPDATE_SECURITY_TYPES_EVENT)
    }
}

impl Default for UpdateSecurityTypesEvent {
    /// Creates a new default instance of UpdateSecurityTypesEvent
    ///
    /// # Arguments
    ///
    /// # Examples
    /// ```
    /// let event = UpdateSecurityTypesEvent::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Event;

    use crate::{
        core::constants::UPDATE_SECURITY_TYPES_EVENT,
        events::update_security_types::UpdateSecurityTypesEvent,
    };

    #[test]
    fn test_from() {
        let event = UpdateSecurityTypesEvent::new();
        let expected = Event::new(UPDATE_SECURITY_TYPES_EVENT);
        let from_event = Event::from(event);

        assert_eq!(expected, from_event);
    }
}
