use cosmwasm_std::{Addr, CosmosMsg, Empty, StdError, StdResult};
use provwasm_std::{
    shim::Any,
    types::{
        cosmos::base::v1beta1::Coin,
        provenance::{
            marker::v1::{
                Access, AccessGrant, MarkerAccount, MarkerQuerier, MarkerStatus, MarkerType,
                MsgActivateRequest, MsgAddAccessRequest, MsgAddMarkerRequest, MsgBurnRequest,
                MsgCancelRequest, MsgDeleteRequest, MsgFinalizeRequest, MsgMintRequest,
                MsgTransferRequest, MsgWithdrawRequest, MsgWithdrawResponse,
            },
            name::v1::{MsgBindNameRequest, NameRecord},
        },
    },
};
use serde::Deserialize;

pub fn bind_name(
    name: &str,
    address: &Addr,
    contract_address: &Addr,
    restricted: bool,
) -> StdResult<CosmosMsg> {
    let addresses = name.split_once(".");
    if addresses.is_none() {
        return Err(StdError::generic_err("invalid bind name"));
    }
    Ok(MsgBindNameRequest {
        parent: Some(NameRecord {
            name: addresses.unwrap().1.to_string(),
            address: contract_address.to_string(),
            restricted: restricted,
        }),
        record: Some(NameRecord {
            name: addresses.unwrap().0.to_string(),
            address: address.to_string(),
            restricted: restricted,
        }),
    }
    .into())
}

pub fn create_marker<S: Into<String>>(
    amount: u128,
    denom: S,
    marker_type: MarkerType,
    contract_address: Addr,
    allow_forced_transfer: bool,
) -> StdResult<CosmosMsg> {
    let coin = Coin {
        amount: amount.to_string(),
        denom: validate_string(denom, "denom")?,
    };

    Ok(MsgAddMarkerRequest {
        amount: Some(coin),
        manager: validate_address(contract_address)?.to_string(),
        from_address: validate_address(contract_address)?.to_string(),
        status: MarkerStatus::Proposed.into(),
        marker_type: marker_type.into(),
        access_list: vec![],
        supply_fixed: false,
        allow_governance_control: false,
        allow_forced_transfer,
        required_attributes: vec![],
    }
    .into())
}

pub fn grant_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
    permissions: Vec<AccessGrant>,
) -> StdResult<CosmosMsg> {
    Ok(MsgAddAccessRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(address)?.to_string(),
        access: permissions,
    }
    .into())
}

pub fn finalize_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgFinalizeRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn activate_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgActivateRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn withdraw_coins<S: Into<String>, H: Into<Addr>>(
    marker_denom: S,
    amount: u128,
    denom: S,
    recipient: H,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("withdraw amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgWithdrawRequest {
        denom: validate_string(marker_denom, "marker_denom")?,
        administrator: validate_address(contract_address)?.to_string(),
        to_address: validate_address(recipient)?.to_string(),
        amount: vec![coin],
    }
    .into())
}

pub fn mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("mint amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };

    Ok(MsgMintRequest {
        amount: Some(coin),
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn burn_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
    contract_address: Addr,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("burn amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgBurnRequest {
        amount: Some(coin),
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn cancel_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgCancelRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn destroy_marker<S: Into<String>>(denom: S, contract_address: Addr) -> StdResult<CosmosMsg> {
    Ok(MsgDeleteRequest {
        denom: validate_string(denom, "denom")?,
        administrator: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
    contract_address: H,
) -> StdResult<CosmosMsg> {
    if amount == 0 {
        return Err(StdError::generic_err("transfer amount must be > 0"));
    }
    let coin = Coin {
        denom: validate_string(denom, "denom")?,
        amount: amount.to_string(),
    };
    Ok(MsgTransferRequest {
        amount: Some(coin),
        administrator: contract_address.into().to_string(),
        from_address: validate_address(from)?.to_string(),
        to_address: validate_address(to)?.to_string(),
    }
    .into())
}

pub fn get_marker_by_address<H: Into<Addr>>(
    address: H,
    querier: &MarkerQuerier<Empty>,
) -> StdResult<Marker> {
    let response = querier.marker(validate_address(address)?.to_string())?;
    if let Some(marker) = response.marker {
    } else {
        return Err(StdError::generic_err("no marker found for address"));
    }
    return Err(StdError::generic_err("no marker found for address"));
}

pub fn all_access(address: &Addr) -> Vec<AccessGrant> {
    vec![AccessGrant {
        address: address.to_string(),
        permissions: vec![
            Access::Admin.into(),
            Access::Burn.into(),
            Access::Deposit.into(),
            Access::Delete.into(),
            Access::Mint.into(),
            Access::Transfer.into(),
            Access::Withdraw.into(),
        ],
    }]
}

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
pub fn validate_address<H: Into<Addr>>(input: H) -> StdResult<Addr> {
    let h: Addr = input.into();
    if h.to_string().trim().is_empty() {
        Err(StdError::generic_err("address must not be empty"))
    } else {
        Ok(h)
    }
}
