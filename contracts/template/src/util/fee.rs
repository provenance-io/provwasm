use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, CosmosMsg, StdError, StdResult};
use provwasm_std::types::provenance::msgfees::v1::MsgAssessCustomMsgFeeRequest;

#[cw_serde]
pub struct Fee {
    pub recipient: Option<Addr>,
    pub amount: Coin,
}

/// A helper to replace the deprecated assess custom fee.
pub fn assess_custom_fee<S: Into<String>>(
    amount: Coin,
    name: Option<S>,
    from: Addr,
    recipient: Option<Addr>,
) -> Result<CosmosMsg, StdError> {
    let coin = provwasm_std::types::cosmos::base::v1beta1::Coin {
        denom: amount.denom,
        amount: amount.amount.to_string(),
    };

    Ok(MsgAssessCustomMsgFeeRequest {
        name: name.map(|s| s.into()).unwrap_or_else(|| "".to_string()),
        amount: Some(coin),
        recipient: recipient.unwrap_or_else(|| Addr::unchecked("")).to_string(),
        from: validate_address(from)?.to_string(),
        recipient_basis_points: "10000".to_string(),
    }
    .into())
}

/// A helper that ensures address params are non-empty.
pub fn validate_address<H: Into<Addr>>(input: H) -> StdResult<Addr> {
    let h: Addr = input.into();
    if h.to_string().trim().is_empty() {
        Err(StdError::generic_err("address must not be empty"))
    } else {
        Ok(h)
    }
}
