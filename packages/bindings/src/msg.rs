#![allow(clippy::field_reassign_with_default)]
use cosmwasm_std::{coin, to_binary, Binary, Coin, CosmosMsg, HumanAddr, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{AttributeValueType, MarkerAccess, MarkerType, ProvenanceRoute};

// The data format version to pass into provenance for message encoding
static MSG_DATAFMT_VERSION: &str = "2.0.0";

/// Represents a request to encode custom provenance messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProvenanceMsg {
    pub route: ProvenanceRoute,      // The module router key
    pub params: ProvenanceMsgParams, // The module-specific encoder params
    pub version: String,             // The data format version
}

/// Input params for custom provenance message encoders.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProvenanceMsgParams {
    Name(NameMsgParams),
    Attribute(AttributeMsgParams),
    Marker(MarkerMsgParams),
}

/// Input params for creating name module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NameMsgParams {
    BindName {
        name: String,
        address: HumanAddr,
        restrict: bool,
    },
    DeleteName {
        name: String,
    },
}

// Create a custom cosmos message using name module params.
fn create_name_msg(params: NameMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Name,
        params: ProvenanceMsgParams::Name(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will bind a restricted name to an address.
pub fn bind_name<S: Into<String>, H: Into<HumanAddr>>(
    name: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name: name.into(),
        address: address.into(),
        restrict: true,
    })
}

/// Create a message that will bind an unrestricted name to an address.
pub fn bind_name_unrestricted<S: Into<String>, H: Into<HumanAddr>>(
    name: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name: name.into(),
        address: address.into(),
        restrict: false,
    })
}

/// Create a message that will un-bind a name from an address.
pub fn unbind_name<S: Into<String>>(name: S) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::DeleteName { name: name.into() })
}

/// Input params for creating attribute module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeMsgParams {
    AddAttribute {
        address: HumanAddr,
        name: String,
        value: Binary,
        value_type: AttributeValueType,
    },
    DeleteAttribute {
        address: HumanAddr,
        name: String,
    },
}

// Create a custom cosmos message using attribute module params.
fn create_attribute_msg(params: AttributeMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Attribute,
        params: ProvenanceMsgParams::Attribute(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will add a an attribute to an account.
pub fn add_attribute<H: Into<HumanAddr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    name: S,
    value: B,
    value_type: AttributeValueType,
) -> CosmosMsg<ProvenanceMsg> {
    create_attribute_msg(AttributeMsgParams::AddAttribute {
        address: address.into(),
        name: name.into(),
        value: value.into(),
        value_type,
    })
}

/// Create a message that will add a JSON attribute to an account. Serializable types can be passed
/// into this function, but it's up to the user to handle StdResult error case.
pub fn add_json_attribute<H: Into<HumanAddr>, S: Into<String>, T: Serialize + ?Sized>(
    address: H,
    name: S,
    data: &T,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    // Serialize the value, bailing on error
    let value = to_binary(data)?;
    // Create and return json typed message
    Ok(add_attribute(
        address,
        name,
        value,
        AttributeValueType::Json,
    ))
}

/// Create a message that will remove all attributes with the given name from an account.
pub fn delete_attributes<H: Into<HumanAddr>, S: Into<String>>(
    address: H,
    name: S,
) -> CosmosMsg<ProvenanceMsg> {
    create_attribute_msg(AttributeMsgParams::DeleteAttribute {
        address: address.into(),
        name: name.into(),
    })
}

/// Input params for creating marker module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerMsgParams {
    CreateMarker {
        coin: Coin,
        marker_type: MarkerType,
    },
    GrantMarkerAccess {
        denom: String,
        address: HumanAddr,
        permissions: Vec<MarkerAccess>,
    },
    RevokeMarkerAccess {
        denom: String,
        address: HumanAddr,
    },
    FinalizeMarker {
        denom: String,
    },
    ActivateMarker {
        denom: String,
    },
    CancelMarker {
        denom: String,
    },
    DestroyMarker {
        denom: String,
    },
    MintMarkerSupply {
        coin: Coin,
    },
    BurnMarkerSupply {
        coin: Coin,
    },
    WithdrawMarkerCoins {
        coin: Coin,
        recipient: HumanAddr,
    },
    TransferMarkerCoins {
        coin: Coin,
        to: HumanAddr,
        from: HumanAddr,
    },
}

// Create a custom cosmos message using marker module params.
fn create_marker_msg(params: MarkerMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will propose a new marker with the default type.
pub fn create_marker<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_with_type(amount, denom, MarkerType::Coin)
}

/// Create a message that will propose a new marker with the type set to restricted.
pub fn create_restricted_marker<S: Into<String>>(
    amount: u128,
    denom: S,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_with_type(amount, denom, MarkerType::Restricted)
}

// Create a message that will propose a new marker with a given type.
fn create_marker_with_type<S: Into<String>>(
    amount: u128,
    denom: S,
    marker_type: MarkerType,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::CreateMarker { coin, marker_type })
}

/// Create a message that will grant permissions on a marker.
pub fn grant_marker_access<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
    permissions: Vec<MarkerAccess>,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::GrantMarkerAccess {
        denom: denom.into(),
        address: address.into(),
        permissions,
    })
}

/// Create a message that will grant all available permissions on a marker.
pub fn grant_marker_access_all<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    grant_marker_access(
        denom,
        address,
        vec![
            MarkerAccess::Admin,
            MarkerAccess::Burn,
            MarkerAccess::Delete,
            MarkerAccess::Deposit,
            MarkerAccess::Mint,
            MarkerAccess::Transfer,
            MarkerAccess::Withdraw,
        ],
    )
}

/// Create a message that will revoke marker permissions.
pub fn revoke_marker_access<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::RevokeMarkerAccess {
        denom: denom.into(),
        address: address.into(),
    })
}

/// Create a message that will finalize a proposed marker.
pub fn finalize_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::FinalizeMarker {
        denom: denom.into(),
    })
}

/// Create a message that will activate a finalized marker.
pub fn activate_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::ActivateMarker {
        denom: denom.into(),
    })
}

/// Create a message that will cancel a marker.
pub fn cancel_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::CancelMarker {
        denom: denom.into(),
    })
}

/// Create a message that will destroy a marker.
pub fn destroy_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::DestroyMarker {
        denom: denom.into(),
    })
}

/// Create a message that will mint marker coins.
pub fn mint_marker_supply<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::MintMarkerSupply { coin })
}

/// Create a message that will burn marker coins.
pub fn burn_marker_supply<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::BurnMarkerSupply { coin })
}

/// Create a message that will transfer marker coins to a recipient account.
pub fn withdraw_marker_coins<S: Into<String>, H: Into<HumanAddr>>(
    amount: u128,
    denom: S,
    recipient: H,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::WithdrawMarkerCoins {
        coin,
        recipient: recipient.into(),
    })
}

/// Create a message that will transfer a marker amount from one account to another.
pub fn transfer_marker_coins<S: Into<String>, H: Into<HumanAddr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::TransferMarkerCoins {
        coin,
        to: to.into(),
        from: from.into(),
    })
}
