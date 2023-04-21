use cosmwasm_std::{from_binary, to_binary, Addr, Binary, CosmosMsg, Empty, StdError, StdResult};
use provwasm_std::types::provenance::attribute::v1::{
    AttributeQuerier, AttributeType, MsgAddAttributeRequest, MsgDeleteAttributeRequest,
    MsgDeleteDistinctAttributeRequest, MsgUpdateAttributeRequest,
};
use provwasm_std::types::provenance::name::v1::{MsgBindNameRequest, NameRecord};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn bind_name(
    name: &str,
    address: &Addr,
    contract_address: &Addr,
    restricted: bool,
) -> StdResult<CosmosMsg> {
    let addresses = name.split_once('.');
    if addresses.is_none() {
        return Err(StdError::generic_err("invalid bind name"));
    }
    Ok(MsgBindNameRequest {
        parent: Some(NameRecord {
            name: addresses.unwrap().1.to_string(),
            address: contract_address.to_string(),
            restricted,
        }),
        record: Some(NameRecord {
            name: addresses.unwrap().0.to_string(),
            address: address.to_string(),
            restricted,
        }),
    }
    .into())
}

pub fn add_attribute<H: Into<Addr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    contract_address: H,
    name: S,
    value: B,
    value_type: AttributeType,
) -> StdResult<CosmosMsg> {
    if value_type == AttributeType::Unspecified {
        return Err(StdError::generic_err(
            "cannot add attribute with unspecified value type",
        ));
    }

    let bin: Binary = value.into();
    Ok(MsgAddAttributeRequest {
        name: validate_string(name, "name")?,
        value: bin.to_vec(),
        attribute_type: value_type.into(),
        account: validate_address(address)?.to_string(),
        owner: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn add_json_attribute<H: Into<Addr>, S: Into<String>, T: Serialize + ?Sized>(
    address: H,
    contract_address: H,
    name: S,
    data: &T,
) -> StdResult<CosmosMsg> {
    add_attribute(
        address,
        contract_address,
        name,
        to_binary(data)?,
        AttributeType::Json,
    )
}

pub fn delete_attributes<H: Into<Addr>, S: Into<String>>(
    address: H,
    contract_address: H,
    name: S,
) -> StdResult<CosmosMsg> {
    Ok(MsgDeleteAttributeRequest {
        name: validate_string(name, "name")?,
        account: validate_address(address)?.to_string(),
        owner: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn delete_distinct_attribute<H: Into<Addr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    contract_address: H,
    name: S,
    value: B,
) -> StdResult<CosmosMsg> {
    let bin: Binary = value.into();
    Ok(MsgDeleteDistinctAttributeRequest {
        name: validate_string(name, "name")?,
        value: bin.to_vec(),
        account: validate_address(address)?.to_string(),
        owner: validate_address(contract_address)?.to_string(),
    }
    .into())
}

pub fn update_attribute<H: Into<Addr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    contract_address: H,
    name: S,
    original_value: B,
    original_value_type: AttributeType,
    update_value: B,
    update_value_type: AttributeType,
) -> StdResult<CosmosMsg> {
    if original_value_type == AttributeType::Unspecified {
        return Err(StdError::generic_err(
            "cannot update attribute with unspecified original value type",
        ));
    }
    if update_value_type == AttributeType::Unspecified {
        return Err(StdError::generic_err(
            "cannot update attribute with unspecified update value type",
        ));
    }

    Ok(MsgUpdateAttributeRequest {
        original_value: (original_value.into() as Binary).to_vec(),
        update_value: (update_value.into() as Binary).to_vec(),
        original_attribute_type: original_value_type.into(),
        update_attribute_type: update_value_type.into(),
        account: validate_address(address)?.to_string(),
        owner: validate_address(contract_address)?.to_string(),
        name: validate_string(name, "name")?,
    }
    .into())
}

pub fn get_json_attributes<H: Into<Addr>, S: Into<String>, T: DeserializeOwned>(
    querier: AttributeQuerier<Empty>,
    address: H,
    name: S,
) -> StdResult<Vec<T>> {
    // Gather results
    let resp = querier.attribute(
        validate_address(address)?.to_string(),
        validate_string(name, "name")?,
        None,
    )?;

    // Map deserialize, returning values or failure.
    resp.attributes
        .iter()
        .filter(|a| a.attribute_type == AttributeType::Json as i32)
        .map(|a| from_binary(&Binary::from(a.value.clone())))
        .collect()
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
