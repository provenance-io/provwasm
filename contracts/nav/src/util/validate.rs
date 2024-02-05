use cosmwasm_std::{Coin, Deps};

use crate::core::error::ContractError;

pub type ValidateResult = Result<(), ContractError>;

pub trait Validate {
    /// Performs basic error checking on the messages that implement this trait.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `deps` - A non mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
    ///
    /// # Examples
    /// ```
    /// let msg = MigrateMsg::Default {};
    /// msg.validate(deps)?;
    /// ```
    fn validate(&self, deps: Deps) -> ValidateResult;

    /// Performs basic error checking on the messages that implement this trait.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the message implementing this trait.
    /// * `funds` - A slice representing the funds included with the message.
    ///
    /// # Examples
    /// ```
    /// let msg = MigrateMsg::Default {};
    /// msg.validate_funds(deps, &info.funds)?;
    /// ```
    fn validate_funds(&self, funds: &[Coin]) -> ValidateResult;
}
