#![allow(clippy::field_reassign_with_default)]
use cosmwasm_std::{HumanAddr, StdError, StdResult};

/// A helper that ensures string params are non-empty.
pub fn validate_string<S: Into<String>>(input: S, param_name: &str) -> StdResult<String> {
    let s: String = input.into();
    if s.trim().is_empty() {
        let errm = format!("{} must not be empty", param_name);
        Err(StdError::generic_err(errm))
    } else {
        Ok(s)
    }
}

/// A helper that ensures address params are non-empty.
pub fn validate_address<H: Into<HumanAddr>>(input: H) -> StdResult<HumanAddr> {
    let h: HumanAddr = input.into();
    if h.trim().is_empty() {
        Err(StdError::generic_err("address must not be empty"))
    } else {
        Ok(h)
    }
}
