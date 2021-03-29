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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_string_empty() {
        let whitespace = " \r \n \t ";
        let err = validate_string(whitespace, "whitespace").unwrap_err();
        match err {
            StdError::GenericErr { msg, .. } => {
                assert_eq!(msg, "whitespace must not be empty")
            }
            _ => panic!("unexpected error"),
        }
    }

    #[test]
    fn validate_address_empty() {
        let empty = HumanAddr::from("");
        let err = validate_address(empty).unwrap_err();
        match err {
            StdError::GenericErr { msg, .. } => {
                assert_eq!(msg, "address must not be empty")
            }
            _ => panic!("unexpected error"),
        }
    }
}
